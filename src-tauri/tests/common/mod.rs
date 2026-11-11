#![allow(dead_code)]

use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri must live inside the repository root")
        .to_path_buf()
}

fn git_ok(args: &[&str]) -> bool {
    Command::new("git")
        .args(args)
        .current_dir(repo_root())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn is_ancestor(sha: &str) -> bool {
    git_ok(&["merge-base", "--is-ancestor", sha, "HEAD"])
}

pub fn commit_exists(sha: &str) -> bool {
    git_ok(&["cat-file", "-e", &format!("{sha}^{{commit}}")])
}

fn recorded_commits() -> &'static HashSet<String> {
    static RECORDED: OnceLock<HashSet<String>> = OnceLock::new();
    RECORDED.get_or_init(|| {
        let path = repo_root()
            .join("tests")
            .join("fixtures")
            .join("lineage.json");
        let Ok(text) = std::fs::read_to_string(path) else {
            return HashSet::new();
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
            return HashSet::new();
        };

        let mut commits = HashSet::new();
        if let Some(baseline) = json.get("baseline") {
            for key in ["sha", "legacySha"] {
                if let Some(sha) = baseline.get(key).and_then(|value| value.as_str()) {
                    commits.insert(sha.to_string());
                }
            }
        }
        if let Some(accepted) = json
            .get("acceptedCommits")
            .and_then(|value| value.as_array())
        {
            for entry in accepted {
                if let Some(sha) = entry.get("sha").and_then(|value| value.as_str()) {
                    commits.insert(sha.to_string());
                }
            }
        }
        commits
    })
}

/// A commit is accepted when it is an ancestor of `head`, an object available in
/// this clone, or an entry in the checked-in lineage manifest.
pub fn verify_commit_at(sha: &str, head: &str) -> bool {
    git_ok(&["merge-base", "--is-ancestor", sha, head])
        || commit_exists(sha)
        || recorded_commits().contains(sha)
}

/// A commit is accepted when it is an ancestor of HEAD, an object available in
/// this clone, or an entry in the checked-in lineage manifest. The manifest keeps
/// phase-gate provenance verifiable in a clean clone where the original
/// predecessor branches were never published.
pub fn verify_commit(sha: &str) -> bool {
    verify_commit_at(sha, "HEAD")
}
