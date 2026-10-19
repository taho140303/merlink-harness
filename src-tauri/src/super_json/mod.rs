//! Super JSON document: schema, locked IO, progress, and the Vietnamese summary.
//!
//! Callers keep using `crate::super_json::{...}`; each submodule owns one concern
//! and everything public is re-exported here. Moving code must not change the 25
//! paths, the 70/30 progress split, the JSON shape, or any error string.

mod io;
mod progress;
mod schema;
mod summary;

// `super_json` is private to the crate, so rustc cannot see these re-exports
// from outside; they stay public for call sites and for the crate's tests.
#[allow(unused_imports)]
pub use io::{get, init_project, load_or_init, set};
#[allow(unused_imports)]
pub use progress::{
    filled_counts, missing_required, progress, LABEL_ACCEPTABLE, LABEL_COLLECTING, LABEL_GOOD,
    LABEL_NO_PROJECT,
};
#[allow(unused_imports)]
pub use schema::{optional_paths, required_paths, skeleton, valid_paths};
#[allow(unused_imports)]
pub use summary::summary;
