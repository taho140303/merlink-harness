//! Pure business domain module root (REQ-ARCH-001, P6S2).
//!
//! Must not depend on rusqlite, tokio::fs, reqwest, or tauri.

pub mod business_flow;
pub mod cascade_delete;
pub mod code_allocator;
pub mod fingerprint;
pub mod invariants;
pub mod phase_gate;
pub mod phase3_artifact;
pub mod phase3_merge;
pub mod progress_engine;
pub mod sf06;
pub mod tools;
pub mod word_count;
pub mod workflow_planner;
