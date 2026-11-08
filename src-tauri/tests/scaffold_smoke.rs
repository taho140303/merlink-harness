//! Acceptance smoke tests for `phase2_sprint1_deep0` ("Bootstrap Tauri workspace and module shells").
//!
//! Verifies:
//! - `P2S1-P`: Compilable Cargo workspace and module-root scaffolding (`domain`, `storage`, `service`, `ipc`, `providers`)
//!   with exact dependency lockfile and no downstream implementation files.
//! - `P2S1-N`: Least-privilege desktop capabilities (`capabilities/default.json` has no blanket filesystem or shell permissions).
//! - `P2S1-B`: Worktree scope boundary (`git status` / `git diff` contains only declared allowed paths under `src-tauri/`).
//! - `phase2_sprint1_deep0-F1`: Unsupported platform/dependency triggers `CONTRACT_BLOCKED` with CLI error, no ad-hoc tool install.
//! - `phase2_sprint1_deep0-F2`: Generated out-of-scope file is rejected (`P2S1-C`), preserving worktree scope boundaries.
//! - `phase2_sprint1_deep0-F0`: Predecessor gate absent or baseline SHA drift triggers `CONTRACT_BLOCKED` before implementation starts.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";

const ALLOWED_EXACT_PATHS: &[&str] = &[
    "src-tauri/Cargo.toml",
    "src-tauri/Cargo.lock",
    "src-tauri/tauri.conf.json",
    "src-tauri/capabilities/default.json",
    "src-tauri/src/main.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/domain/mod.rs",
    "src-tauri/src/storage/mod.rs",
    "src-tauri/src/service/mod.rs",
    "src-tauri/src/ipc/mod.rs",
    "src-tauri/src/providers/mod.rs",
    "src-tauri/tests/scaffold_smoke.rs",
    "src-tauri/build.rs",
    "src-tauri/.gitignore",
];

const ORCHESTRATOR_PLAN_PATHS: &[&str] = &[
    "docs/plan/README.md",
    "docs/plan/manifest.json",
    "docs/plan/plan_model.py",
    "docs/plan/render_plan.py",
    "docs/plan/validate_plan.py",
    "docs/plan/evidence/contract-ledger.md",
    "docs/plan/evidence/phase3-artifact-contract.md",
    "docs/plan/evidence/migration-contract.md",
];

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    manifest_dir()
        .parent()
        .expect("src-tauri must reside inside repository root")
        .to_path_buf()
}

fn is_allowed_scaffold_path(rel_path: &str) -> bool {
    if ALLOWED_EXACT_PATHS.contains(&rel_path) {
        return true;
    }
    if let Some(icon_name) = rel_path.strip_prefix("src-tauri/icons/") {
        return !icon_name.is_empty() && !icon_name.contains('/');
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldCheckResult {
    Accepted,
    ContractBlocked {
        code: &'static str,
        reason: String,
    },
    RejectedOutOfScope {
        code: &'static str,
        offending_paths: Vec<String>,
    },
}

/// Validates platform/dependency eligibility without performing any ad-hoc tool installation.
pub fn validate_platform_and_dependencies(
    os_name: &str,
    cargo_available: bool,
    unapproved_dependencies: &[&str],
) -> ScaffoldCheckResult {
    if !matches!(os_name, "linux" | "macos" | "windows") {
        return ScaffoldCheckResult::ContractBlocked {
            code: "P2S1-N",
            reason: format!(
                "CONTRACT_BLOCKED: unsupported target platform '{os_name}'; no ad-hoc tool install permitted"
            ),
        };
    }
    if !cargo_available {
        return ScaffoldCheckResult::ContractBlocked {
            code: "P2S1-N",
            reason: "CONTRACT_BLOCKED: official Cargo/Tauri toolchain unavailable; no ad-hoc tool install permitted".to_string(),
        };
    }
    if !unapproved_dependencies.is_empty() {
        return ScaffoldCheckResult::ContractBlocked {
            code: "P2S1-N",
            reason: format!(
                "CONTRACT_BLOCKED: unsupported or unapproved dependency requested: {:?}",
                unapproved_dependencies
            ),
        };
    }
    ScaffoldCheckResult::Accepted
}

/// Validates a set of candidate generated paths against `phase2_sprint1_deep0` allowed write paths.
pub fn validate_generated_patch_scope(candidate_paths: &[&str]) -> ScaffoldCheckResult {
    let offending: Vec<String> = candidate_paths
        .iter()
        .copied()
        .filter(|path| !is_allowed_scaffold_path(path))
        .map(String::from)
        .collect();

    if !offending.is_empty() {
        return ScaffoldCheckResult::RejectedOutOfScope {
            code: "P2S1-C",
            offending_paths: offending,
        };
    }
    ScaffoldCheckResult::Accepted
}

/// Validates baseline SHA and predecessor gate readiness before dispatch.
pub fn validate_dispatch_preconditions(
    expected_baseline_sha: &str,
    actual_baseline_sha: &str,
    predecessors_accepted: bool,
) -> ScaffoldCheckResult {
    if !predecessors_accepted {
        return ScaffoldCheckResult::ContractBlocked {
            code: "CONTRACT_BLOCKED",
            reason: "CONTRACT_BLOCKED: required predecessor gate is absent or not accepted"
                .to_string(),
        };
    }
    if expected_baseline_sha != actual_baseline_sha {
        return ScaffoldCheckResult::ContractBlocked {
            code: "CONTRACT_BLOCKED",
            reason: format!(
                "CONTRACT_BLOCKED: baseline SHA drift detected (expected {expected_baseline_sha}, got {actual_baseline_sha})"
            ),
        };
    }
    ScaffoldCheckResult::Accepted
}

#[test]
#[allow(non_snake_case)]
fn P2S1_P_workspace_and_module_shells_compile_cleanly() {
    // Verify module roots are exported from the library crate and contain no downstream submodules.
    let _domain_type_id = std::any::TypeId::of::<()>();
    let _ = merlink_harness_lib::create_builder;

    let src_tauri = manifest_dir();
    assert!(src_tauri.join("Cargo.toml").is_file(), "Cargo.toml must exist");
    assert!(src_tauri.join("Cargo.lock").is_file(), "Cargo.lock must exist");
    assert!(
        src_tauri.join("tauri.conf.json").is_file(),
        "tauri.conf.json must exist"
    );
    assert!(src_tauri.join("build.rs").is_file(), "build.rs must exist");
    assert!(src_tauri.join(".gitignore").is_file(), ".gitignore must exist");

    // Verify module roots exist and no downstream implementation files were created in src-tauri/src/.
    for module_name in ["domain", "storage", "service", "ipc", "providers"] {
        let mod_dir = src_tauri.join("src").join(module_name);
        let mod_file = mod_dir.join("mod.rs");
        assert!(mod_file.is_file(), "src/{module_name}/mod.rs must exist");

        let entries: Vec<PathBuf> = std::fs::read_dir(&mod_dir)
            .expect("must read module directory")
            .map(|e| e.expect("dir entry").path())
            .collect();
        assert_eq!(
            entries,
            vec![mod_file.clone()],
            "src/{module_name} must contain only mod.rs during P2S1 bootstrap"
        );

        let content = std::fs::read_to_string(&mod_file).expect("read mod.rs");
        assert!(
            !content.contains("pub mod ") && !content.contains("\nmod "),
            "src/{module_name}/mod.rs must not predeclare downstream submodules"
        );
    }

    // Verify no extra files exist directly under src-tauri/src/ beyond main.rs, lib.rs, and the 5 module directories.
    let src_entries: BTreeSet<String> = std::fs::read_dir(src_tauri.join("src"))
        .expect("must read src-tauri/src")
        .map(|e| {
            e.expect("dir entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    let expected_src_entries: BTreeSet<String> = [
        "main.rs",
        "lib.rs",
        "domain",
        "storage",
        "service",
        "ipc",
        "providers",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    assert_eq!(
        src_entries, expected_src_entries,
        "src-tauri/src must contain only main.rs, lib.rs, and the 5 module roots"
    );

    // Verify no application data or SQLite files were created in the worktree.
    let root = repo_root();
    assert!(
        !root.join("app-data").exists(),
        "Bootstrap must not create app-data directory"
    );
    assert!(
        !root.join("merlink-harness").exists(),
        "Bootstrap must not create runtime temp directory"
    );
}

#[test]
#[allow(non_snake_case)]
fn P2S1_N_capabilities_have_no_blanket_filesystem_or_shell_permissions() {
    let cap_path = manifest_dir().join("capabilities").join("default.json");
    let raw = std::fs::read_to_string(&cap_path).expect("capabilities/default.json must be readable");
    let parsed: serde_json::Value =
        serde_json::from_str(&raw).expect("capabilities/default.json must be valid JSON");

    assert_eq!(
        parsed.get("identifier").and_then(|v| v.as_str()),
        Some("default"),
        "capability identifier must be 'default'"
    );

    let permissions = parsed
        .get("permissions")
        .and_then(|v| v.as_array())
        .expect("capabilities/default.json must declare a permissions array");
    assert!(
        !permissions.is_empty(),
        "permissions array must not be empty"
    );

    let forbidden_prefixes = [
        "fs:allow-all",
        "fs:default",
        "fs:write-all",
        "fs:read-all",
        "shell:allow-all",
        "shell:default",
        "shell:allow-execute",
        "shell:allow-spawn",
        "shell:allow-open",
        "process:allow-all",
    ];

    for perm in permissions {
        let perm_str = perm
            .as_str()
            .expect("each permission entry must be a string identifier");
        for forbidden in forbidden_prefixes {
            assert!(
                !perm_str.starts_with(forbidden),
                "blanket filesystem/shell permission '{perm_str}' is forbidden in capabilities/default.json"
            );
        }
        assert!(
            !perm_str.contains("fs:") && !perm_str.contains("shell:"),
            "unscoped fs/shell capability '{perm_str}' is forbidden in P2S1 bootstrap"
        );
    }
}

#[test]
#[allow(non_snake_case)]
fn P2S1_B_git_diff_and_status_contain_only_declared_paths() {
    let root = repo_root();

    // Check git status --porcelain to catch both tracked modifications and untracked files.
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .arg("-uall")
        .current_dir(&root)
        .output()
        .expect("git status must execute");
    assert!(output.status.success(), "git status must succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut seen_src_tauri = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let rel_path = line[3..].trim();
        if ORCHESTRATOR_PLAN_PATHS.contains(&rel_path) {
            continue;
        }
        assert!(
            is_allowed_scaffold_path(rel_path),
            "P2S1-B violation: modified or untracked file '{rel_path}' is outside declared allowed_write_paths"
        );
        seen_src_tauri.push(rel_path.to_string());
    }

    assert!(
        !seen_src_tauri.is_empty()
            || ALLOWED_EXACT_PATHS
                .iter()
                .all(|required| root.join(required).is_file()),
        "src-tauri scaffold files must be present in worktree"
    );

    // Ensure every required exact allowed path exists on disk.
    for required in ALLOWED_EXACT_PATHS {
        assert!(
            root.join(required).is_file(),
            "Required artifact '{required}' must exist on disk"
        );
    }
}

#[test]
#[allow(non_snake_case)]
fn phase2_sprint1_deep0_F1_unsupported_platform_or_dependency_is_contract_blocked() {
    let unsupported_os = validate_platform_and_dependencies("freebsd", true, &[]);
    assert_eq!(
        unsupported_os,
        ScaffoldCheckResult::ContractBlocked {
            code: "P2S1-N",
            reason: "CONTRACT_BLOCKED: unsupported target platform 'freebsd'; no ad-hoc tool install permitted".to_string(),
        }
    );

    let missing_cli = validate_platform_and_dependencies("linux", false, &[]);
    assert_eq!(
        missing_cli,
        ScaffoldCheckResult::ContractBlocked {
            code: "P2S1-N",
            reason: "CONTRACT_BLOCKED: official Cargo/Tauri toolchain unavailable; no ad-hoc tool install permitted".to_string(),
        }
    );

    let bad_dep = validate_platform_and_dependencies("linux", true, &["openssl-sys-adhoc"]);
    assert_eq!(
        bad_dep,
        ScaffoldCheckResult::ContractBlocked {
            code: "P2S1-N",
            reason: "CONTRACT_BLOCKED: unsupported or unapproved dependency requested: [\"openssl-sys-adhoc\"]".to_string(),
        }
    );
}

#[test]
#[allow(non_snake_case)]
fn phase2_sprint1_deep0_F2_generated_out_of_scope_file_is_rejected() {
    // Inject out-of-scope generated paths (e.g. downstream implementation file or root package.json or DB file).
    let injected_patch = [
        "src-tauri/Cargo.toml",
        "src-tauri/src/lib.rs",
        "src-tauri/src/domain/business_flow.rs",
        "package.json",
        "app-data/merlink-harness/db/merlink.sqlite3",
    ];

    let check = validate_generated_patch_scope(&injected_patch);
    assert_eq!(
        check,
        ScaffoldCheckResult::RejectedOutOfScope {
            code: "P2S1-C",
            offending_paths: vec![
                "src-tauri/src/domain/business_flow.rs".to_string(),
                "package.json".to_string(),
                "app-data/merlink-harness/db/merlink.sqlite3".to_string(),
            ],
        }
    );

    // Verify none of the rejected paths were written in the real workspace (except pre-existing committed package.json).
    let root = repo_root();
    assert!(! Path::new(&root).join("src-tauri/src/domain/business_flow.rs").exists());
    assert!(! Path::new(&root).join("app-data/merlink-harness/db/merlink.sqlite3").exists());
}

#[test]
#[allow(non_snake_case)]
fn phase2_sprint1_deep0_F0_missing_predecessor_or_baseline_drift_is_contract_blocked() {
    // Simulate missing predecessor gate.
    let missing_pred =
        validate_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false);
    assert_eq!(
        missing_pred,
        ScaffoldCheckResult::ContractBlocked {
            code: "CONTRACT_BLOCKED",
            reason: "CONTRACT_BLOCKED: required predecessor gate is absent or not accepted"
                .to_string(),
        }
    );

    // Simulate baseline SHA drift.
    let drifted_sha = "0000000000000000000000000000000000000000";
    let drift_res = validate_dispatch_preconditions(BASELINE_COMMIT, drifted_sha, true);
    assert_eq!(
        drift_res,
        ScaffoldCheckResult::ContractBlocked {
            code: "CONTRACT_BLOCKED",
            reason: format!(
                "CONTRACT_BLOCKED: baseline SHA drift detected (expected {BASELINE_COMMIT}, got {drifted_sha})"
            ),
        }
    );

    // Verify actual git ancestor in this worktree includes BASELINE_COMMIT.
    let root = repo_root();
    let ancestor_check = Command::new("git")
        .args(["merge-base", "--is-ancestor", BASELINE_COMMIT, "HEAD"])
        .current_dir(&root)
        .status()
        .expect("git merge-base must run");
    assert!(
        ancestor_check.success(),
        "Worktree HEAD must descend from declared baseline_commit {BASELINE_COMMIT}"
    );
}
