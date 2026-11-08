//! SQLite persistence and temporary file storage module root (REQ-ARCH-001, P6S2).
//!
//! Depends on `domain`, must not call up into `service` or `ipc`.

pub mod catalog_repo;
pub mod character_repo;
pub mod commit_repo;
pub mod db;
pub mod draft_repo;
pub mod migration;
pub mod pending_repo;
pub mod project_repo;
pub mod temp_store;
pub mod workflow_repo;
pub mod writing_repo;
