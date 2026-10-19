//! Test-only helpers.
//!
//! `MERLINK_HOME` is process-global and `cargo test` runs tests in parallel, so
//! every test that points storage at a temp directory must hold [`ENV_LOCK`].
//! Without it two tests would fight over the same env var and flake.

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

pub static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Points `MERLINK_HOME` at a fresh temp directory for the test's lifetime and
/// removes that directory on drop. Never touches the developer's real
/// `~/.merlink`.
pub struct TempHome {
    _guard: MutexGuard<'static, ()>,
    root: PathBuf,
    previous: Option<OsString>,
}

impl TempHome {
    pub fn new(name: &str) -> Self {
        // Recover from a poisoned lock so one failing test does not cascade.
        let guard = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        let root = std::env::temp_dir().join(format!(
            "merlink-test-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create temp home");
        let previous = std::env::var_os("MERLINK_HOME");
        unsafe { std::env::set_var("MERLINK_HOME", &root) };
        Self {
            _guard: guard,
            root,
            previous,
        }
    }

    pub fn path(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempHome {
    fn drop(&mut self) {
        match self.previous.take() {
            Some(value) => unsafe { std::env::set_var("MERLINK_HOME", value) },
            None => unsafe { std::env::remove_var("MERLINK_HOME") },
        }
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// Guard for the ignored online e2e test.
///
/// Unlike [`TempHome`] it never invents a directory: it *requires* the operator
/// to set `MERLINK_HOME`, so artifacts land somewhere inspectable and the test
/// can never silently write to a real `~/.merlink`.
pub struct RequiredHome {
    _guard: MutexGuard<'static, ()>,
    pub root: PathBuf,
}

impl RequiredHome {
    pub fn new() -> Self {
        let root = std::env::var_os("MERLINK_HOME")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .expect("MERLINK_HOME must point at a scratch directory to run the online e2e test");
        let guard = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());

        // Start from a clean slate so "missing required" counts are meaningful.
        // `config.json` is deliberately left alone.
        let _ = std::fs::remove_dir_all(root.join("projects"));
        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(root.join(format!("merlink.db{suffix}")));
        }
        std::fs::create_dir_all(&root).expect("create e2e home");

        Self { _guard: guard, root }
    }
}
