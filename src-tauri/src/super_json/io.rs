//! Locked, atomic read/write access to one project's `super.json`.

use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

use serde_json::Value;

use super::schema::{field_kind, skeleton, validate, valid_paths, FieldKind};
use crate::paths;

fn file_path(project_id: &str) -> Result<PathBuf, String> {
    paths::super_json_path(project_id)
}

/// Process-wide per-project serialization. This protects only threads in this app
/// process; a second app process can still race on the same filesystem document.
fn project_lock(project_id: &str) -> Arc<Mutex<()>> {
    static LOCKS: OnceLock<Mutex<HashMap<String, Arc<Mutex<()>>>>> = OnceLock::new();
    let registry = LOCKS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut locks = registry.lock().unwrap_or_else(|poison| poison.into_inner());
    Arc::clone(
        locks
            .entry(project_id.to_owned())
            .or_insert_with(|| Arc::new(Mutex::new(()))),
    )
}

/// Reads the project's super json, creating it from the skeleton on first use.
pub fn load_or_init(project_id: &str) -> Result<Value, String> {
    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    load_or_init_unlocked(project_id)
}

/// The caller must already hold the project's lock.
fn load_or_init_unlocked(project_id: &str) -> Result<Value, String> {
    let path = file_path(project_id)?;
    match fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw)
            .map_err(|error| format!("Invalid super json at {}: {error}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let document = skeleton();
            save_unlocked(project_id, &document, false)?;
            Ok(document)
        }
        Err(error) => Err(format!("Cannot read super json: {error}")),
    }
}

/// The caller must already hold the project's lock. Exclusive saves publish via
/// hard link, which fails if `super.json` exists instead of replacing it.
fn save_unlocked(project_id: &str, document: &Value, exclusive: bool) -> Result<(), String> {
    let path = file_path(project_id)?;
    let parent = path.parent().ok_or("Invalid super json path")?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Cannot create project directory: {error}"))?;

    let bytes = serde_json::to_vec_pretty(document)
        .map_err(|error| format!("Cannot encode super json: {error}"))?;
    let tmp = parent.join(format!(
        ".super.{}.{}.tmp",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));

    let result = (|| -> Result<(), String> {
        #[cfg(unix)]
        {
            use std::{io::Write, os::unix::fs::OpenOptionsExt};
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&tmp)
                .map_err(|error| format!("Cannot create super json: {error}"))?;
            file.write_all(&bytes)
                .map_err(|error| format!("Cannot write super json: {error}"))?;
            file.sync_all()
                .map_err(|error| format!("Cannot sync super json: {error}"))?;
        }
        #[cfg(not(unix))]
        {
            fs::write(&tmp, &bytes).map_err(|error| format!("Cannot write super json: {error}"))?;
        }
        if exclusive {
            fs::hard_link(&tmp, &path).map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    format!("Super json already exists at {}", path.display())
                } else {
                    format!("Cannot initialize super json: {error}")
                }
            })?;
            let _ = fs::remove_file(&tmp);
            Ok(())
        } else {
            fs::rename(&tmp, &path).map_err(|error| format!("Cannot replace super json: {error}"))
        }
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// Initialize once, in one atomic publish, without replacing an existing file.
/// A successful return means this call created the file; callers may compensate
/// a later transaction failure by removing only that file.
pub fn init_project(project_id: &str, name: Option<&str>) -> Result<(), String> {
    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = skeleton();
    if let Some(name) = name {
        validate(FieldKind::Text, "require.overview.name", &Value::String(name.to_owned()))?;
        document["require"]["overview"]["name"] = Value::String(name.to_owned());
    }
    save_unlocked(project_id, &document, true)
}

pub fn get(project_id: &str, path: &str) -> Result<Value, String> {
    if !valid_paths().iter().any(|known| known == path) {
        return Err(format!("Unknown super json path: {path}"));
    }
    let document = load_or_init(project_id)?;
    let mut cursor = &document;
    for segment in path.split('.') {
        cursor = cursor
            .get(segment)
            .ok_or_else(|| format!("Path not found: {path}"))?;
    }
    Ok(cursor.clone())
}

/// Sets exactly one whitelisted field. Never creates new fields.
pub fn set(project_id: &str, path: &str, value: Value) -> Result<Value, String> {
    let kind = field_kind(path).ok_or_else(|| {
        format!(
            "Unknown super json path: {path}. Valid paths are: {}",
            valid_paths().join(", ")
        )
    })?;
    validate(kind, path, &value)?;

    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = load_or_init_unlocked(project_id)?;
    let segments: Vec<&str> = path.split('.').collect();
    let (last, parents) = segments.split_last().ok_or("Empty path")?;

    let mut cursor = &mut document;
    for segment in parents {
        cursor = cursor
            .get_mut(*segment)
            .ok_or_else(|| format!("Path not found: {path}"))?;
    }
    let slot = cursor
        .get_mut(*last)
        .ok_or_else(|| format!("Path not found: {path}"))?;
    *slot = value.clone();

    save_unlocked(project_id, &document, false)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::super::progress::{required_value, value_at};
    use super::*;
    use crate::super_json::{missing_required, required_paths};
    use serde_json::json;

    #[test]
    fn test_concurrent_sets_preserve_all_required_values() {
        use std::sync::{Arc, Barrier};

        let _home = crate::test_support::TempHome::new("super-concurrent");
        let project_id = "concurrent".to_owned();
        let paths = required_paths().to_vec();
        let barrier = Arc::new(Barrier::new(paths.len()));
        let threads: Vec<_> = paths
            .iter()
            .cloned()
            .map(|path| {
                let barrier = Arc::clone(&barrier);
                let project_id = project_id.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    set(&project_id, &path, required_value(&path)).unwrap();
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
        let document = load_or_init(&project_id).unwrap();
        for path in &paths {
            assert_eq!(value_at(&document, path), Some(&required_value(path)), "{path}");
        }
    }

    #[test]
    fn test_init_project_refuses_to_overwrite_existing_json() {
        let _home = crate::test_support::TempHome::new("super-no-overwrite");
        init_project("existing", Some("First")).unwrap();
        let error = init_project("existing", Some("Second")).unwrap_err();
        assert!(error.contains("already exists"), "{error}");
        assert_eq!(get("existing", "require.overview.name").unwrap(), json!("First"));
    }

    #[test]
    fn test_super_json_skeleton_and_set_round_trip() {
        let _home = crate::test_support::TempHome::new("super-round-trip");

        let fresh = load_or_init("default").unwrap();
        assert_eq!(fresh, skeleton(), "file must start as the skeleton");

        let on_disk: Value =
            serde_json::from_str(&fs::read_to_string(paths::super_json_path("default").unwrap()).unwrap())
                .unwrap();
        assert_eq!(on_disk, skeleton());
        // Documentation keys must not leak into the user's data file.
        assert!(on_disk.get("$comment").is_none());

        let returned = set("default", "require.overview.name", json!("Nửa Đêm")).unwrap();
        assert_eq!(returned, json!("Nửa Đêm"));

        set("default", "require.overview.plan.arc.count", json!(5)).unwrap();
        set("default", "optional.genre.secondary", json!(["trinh thám"])).unwrap();

        assert_eq!(
            get("default", "require.overview.name").unwrap(),
            json!("Nửa Đêm")
        );
        assert_eq!(
            get("default", "require.overview.plan.arc.count").unwrap(),
            json!(5)
        );

        // A different project must not see the first project's edits.
        assert!(get("other", "require.overview.name").unwrap().is_null());

        // Re-reading from disk preserves the edits.
        assert_eq!(
            load_or_init("default").unwrap()["require"]["overview"]["name"],
            json!("Nửa Đêm")
        );

        let missing = missing_required("default").unwrap();
        assert_eq!(missing.len(), 9, "two required fields were filled");
        assert!(!missing.contains(&"require.overview.name".to_string()));
    }

    #[test]
    fn test_super_json_rejects_unknown_path() {
        let _home = crate::test_support::TempHome::new("super-unknown");

        for bad in [
            "require.overview.bogus",
            "project.name", // the path the model invented before the enum existed
            "",
            "require",
        ] {
            let error = set("default", bad, json!("x")).unwrap_err();
            assert!(
                error.contains("Unknown super json path"),
                "unexpected error for {bad}: {error}"
            );
        }

        // A rejected write must not add anything to the document.
        let document = load_or_init("default").unwrap();
        assert_eq!(document, skeleton());
        assert!(document["require"]["overview"].get("bogus").is_none());
    }

    #[test]
    fn test_super_json_rejects_wrong_type() {
        let _home = crate::test_support::TempHome::new("super-type");
        load_or_init("default").unwrap();

        // Models routinely send numbers as strings; that must be rejected.
        assert!(set("default", "require.overview.plan.arc.count", json!("5")).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(0)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(-1)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(true)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(5.5)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(5)).is_ok());

        assert!(set("default", "optional.overview.plan.arc.tolerance", json!(0)).is_ok());
        assert!(set("default", "optional.overview.plan.arc.tolerance", json!(-1)).is_err());

        assert!(set("default", "require.overview.name", json!("   ")).is_err());
        assert!(set("default", "require.overview.name", json!(7)).is_err());
        assert!(set("default", "require.overview.name", json!(null)).is_err());

        assert!(set("default", "optional.world.has-power-system", json!(true)).is_ok());
        assert!(set("default", "optional.world.has-power-system", json!("yes")).is_err());

        assert!(set("default", "optional.genre.secondary", json!(["a", "b"])).is_ok());
        assert!(set("default", "optional.genre.secondary", json!([])).is_ok());
        assert!(set("default", "optional.genre.secondary", json!([""])).is_err());
        assert!(set("default", "optional.genre.secondary", json!("a")).is_err());

        // Failed writes must leave the file untouched.
        assert_eq!(load_or_init("default").unwrap()["require"]["overview"]["plan"]["arc"]
            ["count"], json!(5));
    }

    #[test]
    fn test_super_json_enum_paths_reject_invalid_value() {
        let _home = crate::test_support::TempHome::new("super-enum");
        load_or_init("default").unwrap();

        assert!(set("default", "optional.theme.resolution-type", json!("MAYBE")).is_err());
        assert!(set("default", "optional.theme.resolution-type", json!("OPEN")).is_ok());

        assert!(set("default", "optional.world.setting-scale", json!("PLANET")).is_err());
        assert!(set("default", "optional.world.setting-scale", json!("CITY")).is_ok());

        assert!(set("default", "optional.overview.language", json!("VI")).is_ok());
        assert!(set("default", "optional.overview.language", json!("vi")).is_err());
        assert!(set("default", "optional.overview.language", json!("FR")).is_err());

        let error =
            set("default", "optional.world.setting-scale", json!("PLANET")).unwrap_err();
        assert!(error.contains("is not one of"), "unhelpful error: {error}");
    }
}
