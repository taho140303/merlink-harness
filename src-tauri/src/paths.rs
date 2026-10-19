use std::path::PathBuf;

/// Root of every Merlink-owned file.
///
/// `MERLINK_HOME` wins over `$HOME` so unit tests and e2e runs can isolate
/// themselves from the developer's real `~/.merlink` directory.
pub fn merlink_root() -> Result<PathBuf, String> {
    if let Some(value) = std::env::var_os("MERLINK_HOME").filter(|value| !value.is_empty()) {
        return Ok(PathBuf::from(value));
    }
    let home = std::env::var_os("HOME")
        .filter(|value| !value.is_empty())
        .ok_or("HOME is not set")?;
    Ok(PathBuf::from(home).join(".merlink"))
}

/// Provider config stays a JSON file on purpose; only runtime data moves to SQLite.
pub fn config_path() -> Result<PathBuf, String> {
    Ok(merlink_root()?.join("config.json"))
}

pub fn db_path() -> Result<PathBuf, String> {
    Ok(merlink_root()?.join("merlink.db"))
}

/// A `project_id` reaches us from the frontend and, indirectly, from model
/// output. Reject anything that could escape the projects directory.
fn safe_segment(value: &str) -> Result<&str, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Identifier must not be empty".into());
    }
    if trimmed == "." || trimmed == ".." {
        return Err(format!("Invalid identifier: {trimmed}"));
    }
    if !trimmed
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(format!(
            "Identifier may only contain letters, digits, '-', '_' and '.': {trimmed}"
        ));
    }
    Ok(trimmed)
}

pub fn project_dir(project_id: &str) -> Result<PathBuf, String> {
    Ok(merlink_root()?.join("projects").join(safe_segment(project_id)?))
}

/// The super json is project-global: one file per project, not per session.
pub fn super_json_path(project_id: &str) -> Result<PathBuf, String> {
    Ok(project_dir(project_id)?.join("super.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `MERLINK_HOME` must win, otherwise every test below would touch the real
    /// `~/.merlink` of whoever runs `cargo test`.
    #[test]
    fn test_merlink_root_honours_env() {
        let home = crate::test_support::TempHome::new("paths");
        let root = home.path();
        assert_eq!(merlink_root().unwrap(), root);
        assert_eq!(db_path().unwrap(), root.join("merlink.db"));
        assert_eq!(config_path().unwrap(), root.join("config.json"));
        assert_eq!(
            super_json_path("default").unwrap(),
            root.join("projects/default/super.json")
        );
    }

    #[test]
    fn test_safe_segment_rejects_traversal() {
        let _home = crate::test_support::TempHome::new("paths-traversal");
        assert!(project_dir("../escape").is_err());
        assert!(project_dir("a/b").is_err());
        assert!(project_dir("..").is_err());
        assert!(project_dir("").is_err());
        assert!(project_dir("with space").is_err());
        assert!(project_dir("ok-project_1").is_ok());
    }
}
