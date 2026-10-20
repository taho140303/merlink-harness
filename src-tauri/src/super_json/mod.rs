//! Super JSON document: schema, locked IO, progress, and the Vietnamese summary.
//!
//! Callers keep using `crate::super_json::{...}`; each submodule owns one concern
//! and everything public is re-exported here. The public API (get/set/missing/
//! progress/summary) stays stable while the document behind it is v2.

mod io;
mod progress;
mod schema;
mod summary;
mod tables;

// `super_json` is private to the crate, so rustc cannot see these re-exports
// from outside; they stay public for call sites and for the crate's tests.
#[allow(unused_imports)]
pub use io::{add_field, add_row, get, init_project, load_existing, load_or_init, set, warnings};
#[allow(unused_imports)]
pub use progress::{
    LABEL_ACCEPTABLE, LABEL_COLLECTING, LABEL_GOOD, LABEL_NO_PROJECT, filled_counts,
    missing_required, progress,
};
#[allow(unused_imports)]
pub use schema::{
    countable_entry, countable_group as schema_table_group, optional_paths, required_paths,
    skeleton, tables as schema_tables, valid_paths,
};
#[allow(unused_imports)]
pub use summary::summary;
