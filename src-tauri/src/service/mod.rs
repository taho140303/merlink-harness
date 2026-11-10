//! Application orchestration, concurrency, and transaction boundaries module root (REQ-ARCH-001, P6S2).
//!
//! Depends on `domain`, `storage`, and `providers`.

pub mod character_service;
pub mod chat_catalog;
pub mod chat_orchestrator;
pub mod commit_service;
pub mod draft_runtime;
pub mod draft_service;
pub mod patch_service;
pub mod project_service;
pub mod receipt_verifier;
pub mod recovery_service;
pub mod round_seal_service;
pub mod workflow_llm_executor;
pub mod workflow_service;
pub mod writing_generation;
pub mod writing_persistence;
