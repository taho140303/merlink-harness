use std::{fs, path::PathBuf};

use crate::models::ProviderConfig;
#[cfg(test)]
use crate::models::{InputTypes, ModelConfig};

/// The ten providers older versions seeded on first run (Sprint 11.13 · D2).
///
/// This is **not** a catalog: it exists only so [`load_providers`] can recognise
/// untouched seed entries left on disk by an older install and drop them once.
/// A new install starts with no providers at all; providers are only ever what
/// the author adds in Settings.
const LEGACY_SEEDED_IDS: [&str; 10] = [
    "openai",
    "anthropic",
    "deepseek",
    "google",
    "qwen",
    "glm",
    "zai",
    "kimi",
    "command-code",
    "ollama",
];

pub fn get_config_path() -> Result<PathBuf, String> {
    crate::paths::config_path()
}

/// The old seed helper, test-only now that the shipped catalog is empty.
#[cfg(test)]
fn provider(
    id: &str,
    name: &str,
    kind: &str,
    base_url: &str,
    model_ids: &[&str],
) -> ProviderConfig {
    ProviderConfig {
        id: id.into(),
        name: name.into(),
        kind: kind.into(),
        api_key: String::new(),
        base_url: base_url.into(),
        default_model_id: model_ids.first().unwrap_or(&"").to_string(),
        models: model_ids
            .iter()
            .map(|model_id| {
                let is_gpt4o = *model_id == "gpt-4o";
                let is_sonnet = *model_id == "claude-sonnet-4-5";
                ModelConfig {
                    id: (*model_id).into(),
                    name: (*model_id).into(),
                    context_window: if is_sonnet { "200K" } else { "128K" }.into(),
                    max_tokens: if is_sonnet { "8K" } else { "4K" }.into(),
                    input_types: InputTypes {
                        text: true,
                        image: is_gpt4o || is_sonnet,
                    },
                    reasoning_level: is_sonnet || model_id.contains("reasoner"),
                }
            })
            .collect(),
    }
}

/// The ten-provider seed of every version before Sprint 11.13, kept **only** as
/// a test fixture: migration tests and IPC round-trip tests still need a realistic
/// config to write to disk. Production code never calls this (D2).
#[cfg(test)]
pub fn catalog_fixture() -> Vec<ProviderConfig> {
    vec![
        provider(
            "openai",
            "OpenAI",
            "openai",
            "https://api.openai.com/v1",
            &["gpt-4o", "gpt-4o-mini", "o1", "o1-mini"],
        ),
        provider(
            "anthropic",
            "Anthropic",
            "anthropic",
            "https://api.anthropic.com",
            &["claude-sonnet-4-5", "claude-opus-4", "claude-haiku-4"],
        ),
        provider(
            "deepseek",
            "DeepSeek",
            "openai",
            "https://api.deepseek.com",
            &["deepseek-chat", "deepseek-coder", "deepseek-reasoner"],
        ),
        provider(
            "google",
            "Google Gemini",
            "openai",
            "https://generativelanguage.googleapis.com/v1beta/openai",
            &["gemini-2.5-pro", "gemini-2.5-flash"],
        ),
        provider(
            "qwen",
            "Qwen (Alibaba)",
            "openai",
            "https://dashscope.aliyuncs.com/compatible-mode/v1",
            &["qwen-max", "qwen-plus"],
        ),
        provider(
            "glm",
            "Zhipu GLM",
            "openai",
            "https://open.bigmodel.cn/api/paas/v4",
            &["glm-4-plus", "glm-4"],
        ),
        provider(
            "zai",
            "Z.ai",
            "openai",
            "https://api.z.ai/api/paas/v4",
            &["glm-4.6"],
        ),
        provider(
            "kimi",
            "Kimi (Moonshot)",
            "openai",
            "https://api.moonshot.cn/v1",
            &["moonshot-v1-8k", "moonshot-v1-32k"],
        ),
        provider("command-code", "Command Code", "custom", "", &[]),
        provider(
            "ollama",
            "Ollama (local)",
            "openai",
            "http://127.0.0.1:11434/v1",
            &["llama3.2"],
        ),
    ]
}

/// The shipped provider list (Sprint 11.13 · D2).
///
/// There is no hardcoded catalog any more: the list is exactly what the author
/// has added in Settings. A fresh install therefore has no providers at all.
pub fn default_providers() -> Vec<ProviderConfig> {
    Vec::new()
}

/// One-shot cleanup predicate: an entry that is still a legacy seed nobody has
/// touched (its api key is empty). Anything the author created or configured is
/// left alone — `custom-<timestamp>` entries, named providers, any provider with
/// a key.
fn is_untouched_legacy_seed(provider: &ProviderConfig) -> bool {
    provider.api_key.trim().is_empty() && LEGACY_SEEDED_IDS.contains(&provider.id.as_str())
}

pub fn load_providers() -> Result<Vec<ProviderConfig>, String> {
    let path = get_config_path()?;
    match fs::read_to_string(&path) {
        Ok(content) => {
            let providers: Vec<ProviderConfig> = serde_json::from_str(&content)
                .map_err(|error| format!("Invalid provider config: {error}"))?;
            let loaded = providers.len();
            let kept: Vec<ProviderConfig> = providers
                .into_iter()
                .filter(|provider| !is_untouched_legacy_seed(provider))
                .collect();
            // Persist the cleanup so the next start does not have to filter again.
            // Only write when something actually changed: never rewrite a config
            // the author owns for no reason.
            if kept.len() != loaded {
                save_providers(&kept)?;
            }
            Ok(kept)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(default_providers()),
        Err(error) => Err(format!("Cannot read provider config: {error}")),
    }
}

pub fn save_providers(providers: &[ProviderConfig]) -> Result<(), String> {
    let path = get_config_path()?;
    let parent = path.parent().ok_or("Invalid config path")?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Cannot create config directory: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(parent, fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("Cannot protect config directory: {error}"))?;
    }
    let bytes = serde_json::to_vec_pretty(providers).map_err(|error| error.to_string())?;
    let tmp = parent.join(format!(
        ".config.{}.{}.tmp",
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
                .map_err(|error| format!("Cannot create config: {error}"))?;
            file.write_all(&bytes)
                .map_err(|error| format!("Cannot write config: {error}"))?;
            file.sync_all()
                .map_err(|error| format!("Cannot sync config: {error}"))?;
        }
        #[cfg(not(unix))]
        {
            fs::write(&tmp, &bytes).map_err(|error| format!("Cannot write config: {error}"))?;
        }
        fs::rename(&tmp, &path).map_err(|error| format!("Cannot replace config: {error}"))
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TempHome;

    /// D2: nothing is shipped. The regression this guards against is a future
    /// patch re-introducing "helpful" seeds that silently reappear for everyone.
    #[test]
    fn test_default_providers_are_empty() {
        assert!(
            default_providers().is_empty(),
            "the hardcoded provider catalog must stay deleted"
        );
    }

    /// Migration: a config written by an older version holds all ten seeded
    /// providers with an empty api key. Loading it must drop every one of them
    /// *and* rewrite `config.json`, so the cleanup only happens once.
    #[test]
    fn test_legacy_seeded_providers_are_pruned() {
        let home = TempHome::new("store-prune-seeds");
        let seeded = catalog_fixture();
        assert_eq!(seeded.len(), 10);
        assert!(
            seeded.iter().all(|provider| provider.api_key.is_empty()),
            "the fixture must model untouched seed entries"
        );
        save_providers(&seeded).unwrap();

        let loaded = load_providers().unwrap();
        assert!(
            loaded.is_empty(),
            "all ten untouched seeds must be pruned, got {}",
            loaded.len()
        );

        // The rewrite is the point: read the file back, not the in-memory list.
        let raw = fs::read_to_string(home.path().join("config.json")).unwrap();
        let on_disk: Vec<ProviderConfig> = serde_json::from_str(&raw).unwrap();
        assert!(
            on_disk.is_empty(),
            "load_providers must rewrite config.json without the legacy seeds"
        );
        for id in LEGACY_SEEDED_IDS {
            assert!(
                !raw.contains(&format!("\"{id}\"")),
                "seed `{id}` is still on disk: {raw}"
            );
        }
    }

    /// The user's own provider (`custom-<timestamp>`, "sad", with a key) must
    /// survive the seed cleanup untouched — only untouched seed entries go.
    #[test]
    fn test_custom_provider_survives_the_seed_cleanup() {
        let home = TempHome::new("store-keep-custom");
        let mut providers = catalog_fixture();
        let mut custom = providers[0].clone();
        custom.id = "custom-1792378417139".into();
        custom.name = "sad".into();
        custom.kind = "custom".into();
        custom.api_key = "user-key-placeholder".into();
        custom.base_url = "https://example.com/v1".into();
        custom.default_model_id = "gpt-4o".into();
        providers.push(custom);
        assert_eq!(providers.len(), 11, "10 seeds + the author's provider");
        save_providers(&providers).unwrap();

        let loaded = load_providers().unwrap();
        assert_eq!(loaded.len(), 1, "exactly the custom provider survives");
        assert_eq!(loaded[0].id, "custom-1792378417139");
        assert_eq!(loaded[0].name, "sad");
        assert_eq!(loaded[0].api_key, "user-key-placeholder");

        let raw = fs::read_to_string(home.path().join("config.json")).unwrap();
        let on_disk: Vec<ProviderConfig> = serde_json::from_str(&raw).unwrap();
        assert_eq!(on_disk.len(), 1);
        assert_eq!(on_disk[0].id, "custom-1792378417139");
        assert!(!raw.contains("\"openai\""));
        assert!(!raw.contains("\"anthropic\""));
    }

    /// A provider the author configured is never mistaken for a seed, even if it
    /// kept a legacy id: a non-empty api key makes it "touched".
    #[test]
    fn test_seeded_id_with_a_key_is_not_pruned() {
        let home = TempHome::new("store-seed-with-key");
        let mut provider = catalog_fixture().remove(0); // id "openai"
        provider.api_key = "author-configured-placeholder".into();
        save_providers(std::slice::from_ref(&provider)).unwrap();

        let loaded = load_providers().unwrap();
        assert_eq!(
            loaded.len(),
            1,
            "a seed entry the author configured must survive: {loaded:?}"
        );
        assert_eq!(loaded[0].id, "openai");

        let raw = fs::read_to_string(home.path().join("config.json")).unwrap();
        assert!(raw.contains("author-configured-placeholder"));
    }
}
