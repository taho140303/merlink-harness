//! Read-only catalog for the System Prompt reference screens.

use crate::error::{ErrorCode, IpcError};
use crate::AppState;
use rusqlite::{types::ValueRef, Connection};
use serde_json::{Map, Value};
use tauri::State;

const TABLES: &[(&str, &str)] = &[
    ("motifs", "motifs"),
    ("genres", "genres"),
    ("motifApplicableGenres", "motif_applicable_genres"),
    ("components", "components"),
    ("states", "states"),
    ("stateRules", "state_rules"),
    ("stateDownByRules", "state_down_by_rules"),
    ("emotionalRanges", "emotional_ranges"),
    ("eventBlocks", "event_blocks"),
    ("eventTransitions", "event_transitions"),
    ("constraints", "constraints"),
    ("motifConstraints", "motif_constraints"),
    ("eventBlockEmotionals", "event_block_emotionals"),
    ("stylePresets", "style_presets"),
    ("rhetoricalDevices", "rhetorical_devices"),
    ("stylePresetRhetoricalDevices", "style_preset_rhetorical_devices"),
    ("stylePresetVocabItems", "style_preset_vocab_items"),
    ("stylePresetFocusItems", "style_preset_focus_items"),
    ("narratorConfigs", "narrator_configs"),
    ("narrativeDirectives", "narrative_directives"),
    ("stylePresetNarratives", "style_preset_narratives"),
    ("themes", "themes"),
    ("themeArguments", "theme_arguments"),
];

fn rows_as_json(conn: &Connection, table: &str) -> Result<Vec<Value>, IpcError> {
    // `table` only comes from the fixed TABLES list above.
    let sql = match table {
        "themes" => "SELECT * FROM themes WHERE deleted_at IS NULL ORDER BY id".to_string(),
        "theme_arguments" => "SELECT ta.*, c.display_name AS character_name FROM theme_arguments ta JOIN characters c ON c.id = ta.character_id JOIN themes t ON t.id = ta.theme_id WHERE t.deleted_at IS NULL ORDER BY ta.id".to_string(),
        _ => format!("SELECT * FROM {table} ORDER BY id"),
    };
    let mut statement = conn.prepare(&sql)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let names: Vec<String> = statement.column_names().iter().map(|s| s.to_string()).collect();
    let mut rows = statement.query([])
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let mut result = Vec::new();
    while let Some(row) = rows.next()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))? {
        let mut object = Map::new();
        for (index, name) in names.iter().enumerate() {
            let value = match row.get_ref(index)
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))? {
                ValueRef::Null => Value::Null,
                ValueRef::Integer(n) => Value::from(n),
                ValueRef::Real(n) => Value::from(n),
                ValueRef::Text(bytes) => Value::String(String::from_utf8_lossy(bytes).into_owned()),
                ValueRef::Blob(_) => Value::Null,
            };
            object.insert(name.clone(), value);
        }
        result.push(Value::Object(object));
    }
    Ok(result)
}

fn read_catalog_for_tab(conn: &Connection, tab: &str) -> Result<Value, IpcError> {
    let mut catalog = Map::new();
    let wanted: &[&str] = match tab {
        "motifs" => &["motifs", "genres", "motifApplicableGenres", "components", "states", "stateRules", "stateDownByRules", "emotionalRanges", "eventBlocks", "eventTransitions", "constraints", "motifConstraints", "eventBlockEmotionals"],
        "stylePresets" => &["stylePresets", "rhetoricalDevices", "stylePresetRhetoricalDevices", "stylePresetVocabItems", "stylePresetFocusItems", "stylePresetNarratives", "narrativeDirectives"],
        "narrativeDirectives" => &["narratorConfigs", "narrativeDirectives"],
        "themes" => &["themes", "themeArguments"],
        "genres" => &["genres", "motifs", "motifApplicableGenres"],
        _ => return Err(IpcError::new(ErrorCode::ValidationFailed, "Unknown system prompt tab")),
    };
    for &(key, table) in TABLES {
        if wanted.contains(&key) {
            catalog.insert(key.to_string(), Value::Array(rows_as_json(&conn, table)?));
        }
    }
    Ok(Value::Object(catalog))
}

#[tauri::command]
pub fn get_system_prompt_catalog(state: State<'_, AppState>, tab: String) -> Result<Value, IpcError> {
    let conn = state.db.get_connection()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    read_catalog_for_tab(&conn, &tab)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_reference_tabs_read_real_sqlite_rows() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::storage::migration::run_migrations(&mut conn).unwrap();
        assert_eq!(rows_as_json(&conn, "style_presets").unwrap().len(), 54);
        assert_eq!(rows_as_json(&conn, "genres").unwrap().len(), 50);
        assert_eq!(rows_as_json(&conn, "style_preset_rhetorical_devices").unwrap().len(), 273);
        let style = read_catalog_for_tab(&conn, "stylePresets").unwrap();
        assert_eq!(style["stylePresets"].as_array().unwrap().len(), 54);
        assert!(style.get("motifs").is_none());
        assert_eq!(read_catalog_for_tab(&conn, "genres").unwrap()["genres"].as_array().unwrap().len(), 50);
        for &(_, table) in TABLES {
            rows_as_json(&conn, table).unwrap();
        }
    }
}
