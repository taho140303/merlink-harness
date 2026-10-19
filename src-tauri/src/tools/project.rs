//! Project-id creation and the no-project guard for the super json tools.

use rusqlite::Connection;

use crate::{db, paths};

/// Prefix shared by every tool that needs a project (D2).
const NO_PROJECT: &str = "No project yet. Call project_create first.";

/// Guards the tools that cannot do anything without a project (D2).
pub(super) fn project_or_error(project_id: &str) -> Result<&str, String> {
    let trimmed = project_id.trim();
    if trimmed.is_empty() {
        return Err(format!(
            "{NO_PROJECT} This chat is not bound to a project, so its super json cannot be \
             read or written."
        ));
    }
    Ok(trimmed)
}

/// D3.3: lowercase, every character outside `[a-z0-9]` becomes `-`, runs of `-`
/// collapse, edges are trimmed; an empty result becomes `"project"`. The result
/// only contains `[a-z0-9-]`, so it always survives `paths::safe_segment`.
fn slugify(name: &str) -> String {
    let mut slug = String::new();
    for character in name.trim().to_lowercase().chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            slug.push(character);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-').to_owned();
    if slug.is_empty() {
        "project".to_owned()
    } else {
        slug
    }
}

/// Reserve the base slug or its `-2`, `-3`, … suffix in the caller's SQLite
/// transaction, never choosing an ID from a stale project list.
pub(super) fn unique_project_id(
    connection: &Connection,
    name: &str,
    title: &str,
) -> Result<String, String> {
    let base = slugify(name);
    for attempt in 1..=1_000 {
        let candidate = if attempt == 1 {
            base.clone()
        } else {
            format!("{base}-{attempt}")
        };
        // The id becomes a directory name, so validate before inserting it.
        let _ = paths::project_dir(&candidate)?;
        if db::create_project(connection, &candidate, title)? {
            return Ok(candidate);
        }
    }
    Err(format!(
        "Cannot create project: exhausted 1000 IDs for {base}"
    ))
}
