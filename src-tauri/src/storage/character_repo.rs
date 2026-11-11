//! Character repository supporting 4-tier search, detail projections,
//! library character upserts, and draft linkage
//! (`REQ-PROJECT-001`, IPC #25, #26, #27, #28, `be-core.md` §10.2.7, `core.md` §3.8).

use chrono::Utc;
use rusqlite::{params, Connection, TransactionBehavior};
use serde_json::json;
use uuid::Uuid;

use crate::domain::code_allocator::CodeAllocator;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    CharacterAppearanceDto, CharacterCompetenceDto, CharacterDetailDto, CharacterLibraryItemDto,
    CharacterMemoryDto, CharacterPsychologicalDto, CharacterRelationshipDto,
    CharacterUpsertPayloadDto,
};

// ============================================================================
// Unicode NFC & Vietnamese Diacritic Helpers (§3.8)
// ============================================================================

/// Checks if character is a Unicode combining diacritical mark.
pub fn is_combining_mark(ch: char) -> bool {
    matches!(ch, '\u{0300}'..='\u{036f}' | '\u{1dc0}'..='\u{1dff}' | '\u{20d0}'..='\u{20ff}' | '\u{fe20}'..='\u{fe2f}')
}

/// Composes base character + combining mark into precomposed NFC character.
pub fn compose_vietnamese_char(base: char, mark: char) -> Option<char> {
    match (base, mark) {
        // a
        ('a', '\u{0300}') => Some('à'),
        ('a', '\u{0301}') => Some('á'),
        ('a', '\u{0303}') => Some('ã'),
        ('a', '\u{0309}') => Some('ả'),
        ('a', '\u{0323}') => Some('ạ'),
        ('a', '\u{0302}') => Some('â'),
        ('a', '\u{0306}') => Some('ă'),
        ('A', '\u{0300}') => Some('À'),
        ('A', '\u{0301}') => Some('Á'),
        ('A', '\u{0303}') => Some('Ã'),
        ('A', '\u{0309}') => Some('Ả'),
        ('A', '\u{0323}') => Some('Ạ'),
        ('A', '\u{0302}') => Some('Â'),
        ('A', '\u{0306}') => Some('Ă'),

        // â
        ('â', '\u{0300}') => Some('ầ'),
        ('â', '\u{0301}') => Some('ấ'),
        ('â', '\u{0303}') => Some('ẫ'),
        ('â', '\u{0309}') => Some('ẩ'),
        ('â', '\u{0323}') => Some('ậ'),
        ('Â', '\u{0300}') => Some('Ầ'),
        ('Â', '\u{0301}') => Some('Ấ'),
        ('Â', '\u{0303}') => Some('Ẫ'),
        ('Â', '\u{0309}') => Some('Ẩ'),
        ('Â', '\u{0323}') => Some('Ậ'),

        // ă
        ('ă', '\u{0300}') => Some('ằ'),
        ('ă', '\u{0301}') => Some('ắ'),
        ('ă', '\u{0303}') => Some('ẵ'),
        ('ă', '\u{0309}') => Some('ẳ'),
        ('ă', '\u{0323}') => Some('ặ'),
        ('Ă', '\u{0300}') => Some('Ằ'),
        ('Ă', '\u{0301}') => Some('Ắ'),
        ('Ă', '\u{0303}') => Some('Ẵ'),
        ('Ă', '\u{0309}') => Some('Ẳ'),
        ('Ă', '\u{0323}') => Some('Ặ'),

        // e
        ('e', '\u{0300}') => Some('è'),
        ('e', '\u{0301}') => Some('é'),
        ('e', '\u{0303}') => Some('ẽ'),
        ('e', '\u{0309}') => Some('ẻ'),
        ('e', '\u{0323}') => Some('ẹ'),
        ('e', '\u{0302}') => Some('ê'),
        ('E', '\u{0300}') => Some('È'),
        ('E', '\u{0301}') => Some('É'),
        ('E', '\u{0303}') => Some('Ẽ'),
        ('E', '\u{0309}') => Some('Ẻ'),
        ('E', '\u{0323}') => Some('Ẹ'),
        ('E', '\u{0302}') => Some('Ê'),

        // ê
        ('ê', '\u{0300}') => Some('ề'),
        ('ê', '\u{0301}') => Some('ế'),
        ('ê', '\u{0303}') => Some('ễ'),
        ('ê', '\u{0309}') => Some('ể'),
        ('ê', '\u{0323}') => Some('ệ'),
        ('Ê', '\u{0300}') => Some('Ề'),
        ('Ê', '\u{0301}') => Some('Ế'),
        ('Ê', '\u{0303}') => Some('Ễ'),
        ('Ê', '\u{0309}') => Some('Ể'),
        ('Ê', '\u{0323}') => Some('Ệ'),

        // i
        ('i', '\u{0300}') => Some('ì'),
        ('i', '\u{0301}') => Some('í'),
        ('i', '\u{0303}') => Some('ĩ'),
        ('i', '\u{0309}') => Some('ỉ'),
        ('i', '\u{0323}') => Some('ị'),
        ('I', '\u{0300}') => Some('Ì'),
        ('I', '\u{0301}') => Some('Í'),
        ('I', '\u{0303}') => Some('Ĩ'),
        ('I', '\u{0309}') => Some('Ỉ'),
        ('I', '\u{0323}') => Some('Ị'),

        // o
        ('o', '\u{0300}') => Some('ò'),
        ('o', '\u{0301}') => Some('ó'),
        ('o', '\u{0303}') => Some('õ'),
        ('o', '\u{0309}') => Some('ỏ'),
        ('o', '\u{0323}') => Some('ọ'),
        ('o', '\u{0302}') => Some('ô'),
        ('o', '\u{031B}') => Some('ơ'),
        ('O', '\u{0300}') => Some('Ò'),
        ('O', '\u{0301}') => Some('Ó'),
        ('O', '\u{0303}') => Some('Õ'),
        ('O', '\u{0309}') => Some('Ỏ'),
        ('O', '\u{0323}') => Some('Ọ'),
        ('O', '\u{0302}') => Some('Ô'),
        ('O', '\u{031B}') => Some('Ơ'),

        // ô
        ('ô', '\u{0300}') => Some('ồ'),
        ('ô', '\u{0301}') => Some('ố'),
        ('ô', '\u{0303}') => Some('ỗ'),
        ('ô', '\u{0309}') => Some('ổ'),
        ('ô', '\u{0323}') => Some('ộ'),
        ('Ô', '\u{0300}') => Some('Ồ'),
        ('Ô', '\u{0301}') => Some('Ố'),
        ('Ô', '\u{0303}') => Some('Ỗ'),
        ('Ô', '\u{0309}') => Some('Ổ'),
        ('Ô', '\u{0323}') => Some('Ộ'),

        // ơ
        ('ơ', '\u{0300}') => Some('ờ'),
        ('ơ', '\u{0301}') => Some('ớ'),
        ('ơ', '\u{0303}') => Some('ỡ'),
        ('ơ', '\u{0309}') => Some('ở'),
        ('ơ', '\u{0323}') => Some('ợ'),
        ('Ơ', '\u{0300}') => Some('Ờ'),
        ('Ơ', '\u{0301}') => Some('Ớ'),
        ('Ơ', '\u{0303}') => Some('Ỡ'),
        ('Ơ', '\u{0309}') => Some('Ở'),
        ('Ơ', '\u{0323}') => Some('Ợ'),

        // u
        ('u', '\u{0300}') => Some('ù'),
        ('u', '\u{0301}') => Some('ú'),
        ('u', '\u{0303}') => Some('ũ'),
        ('u', '\u{0309}') => Some('ủ'),
        ('u', '\u{0323}') => Some('ụ'),
        ('u', '\u{031B}') => Some('ư'),
        ('U', '\u{0300}') => Some('Ù'),
        ('U', '\u{0301}') => Some('Ú'),
        ('U', '\u{0303}') => Some('Ũ'),
        ('U', '\u{0309}') => Some('Ủ'),
        ('U', '\u{0323}') => Some('Ụ'),
        ('U', '\u{031B}') => Some('Ư'),

        // ư
        ('ư', '\u{0300}') => Some('ừ'),
        ('ư', '\u{0301}') => Some('ứ'),
        ('ư', '\u{0303}') => Some('ữ'),
        ('ư', '\u{0309}') => Some('ử'),
        ('ư', '\u{0323}') => Some('ự'),
        ('Ư', '\u{0300}') => Some('Ừ'),
        ('Ư', '\u{0301}') => Some('Ứ'),
        ('Ư', '\u{0303}') => Some('Ữ'),
        ('Ư', '\u{0309}') => Some('Ử'),
        ('Ư', '\u{0323}') => Some('Ự'),

        // y
        ('y', '\u{0300}') => Some('ỳ'),
        ('y', '\u{0301}') => Some('ý'),
        ('y', '\u{0303}') => Some('ỹ'),
        ('y', '\u{0309}') => Some('ỷ'),
        ('y', '\u{0323}') => Some('ỵ'),
        ('Y', '\u{0300}') => Some('Ỳ'),
        ('Y', '\u{0301}') => Some('Ý'),
        ('Y', '\u{0303}') => Some('Ỹ'),
        ('Y', '\u{0309}') => Some('Ỷ'),
        ('Y', '\u{0323}') => Some('Ỵ'),

        // d
        ('d', '\u{0335}' | '\u{0336}') => Some('đ'),
        ('D', '\u{0335}' | '\u{0336}') => Some('Đ'),

        _ => None,
    }
}

/// Normalizes string to NFC representation for Vietnamese and Latin.
pub fn normalize_to_nfc(input: &str) -> String {
    let mut out = Vec::with_capacity(input.len());
    for ch in input.chars() {
        if is_combining_mark(ch) {
            if let Some(prev) = out.last_mut() {
                if let Some(composed) = compose_vietnamese_char(*prev, ch) {
                    *prev = composed;
                    continue;
                }
            }
        }
        out.push(ch);
    }
    out.into_iter().collect()
}

/// Removes Vietnamese diacritics and converts to ASCII representation.
pub fn to_unaccented(input: &str) -> String {
    let nfc = normalize_to_nfc(input);
    let mut out = String::with_capacity(nfc.len());

    for ch in nfc.chars() {
        if is_combining_mark(ch) {
            continue;
        }
        let mapped = match ch {
            'à' | 'á' | 'ả' | 'ã' | 'ạ' | 'ă' | 'ằ' | 'ắ' | 'ẳ' | 'ẵ' | 'ặ' | 'â' | 'ầ' | 'ấ'
            | 'ẩ' | 'ẫ' | 'ậ' => 'a',
            'À' | 'Á' | 'Ả' | 'Ã' | 'Ạ' | 'Ă' | 'Ằ' | 'Ắ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Â' | 'Ầ' | 'Ấ'
            | 'Ẩ' | 'Ẫ' | 'Ậ' => 'A',
            'đ' => 'd',
            'Đ' => 'D',
            'è' | 'é' | 'ẻ' | 'ẽ' | 'ẹ' | 'ê' | 'ề' | 'ế' | 'ể' | 'ễ' | 'ệ' => {
                'e'
            }
            'È' | 'É' | 'Ẻ' | 'Ẽ' | 'Ẹ' | 'Ê' | 'Ề' | 'Ế' | 'Ể' | 'Ễ' | 'Ệ' => {
                'E'
            }
            'ì' | 'í' | 'ỉ' | 'ĩ' | 'ị' => 'i',
            'Ì' | 'Í' | 'Ỉ' | 'Ĩ' | 'Ị' => 'I',
            'ò' | 'ó' | 'ỏ' | 'õ' | 'ọ' | 'ô' | 'ồ' | 'ố' | 'ổ' | 'ỗ' | 'ộ' | 'ơ' | 'ờ' | 'ớ'
            | 'ở' | 'ỡ' | 'ợ' => 'o',
            'Ò' | 'Ó' | 'Ỏ' | 'Õ' | 'Ọ' | 'Ô' | 'Ồ' | 'Ố' | 'Ổ' | 'Ỗ' | 'Ộ' | 'Ơ' | 'Ờ' | 'Ớ'
            | 'Ở' | 'Ỡ' | 'Ợ' => 'O',
            'ù' | 'ú' | 'ủ' | 'ũ' | 'ụ' | 'ư' | 'ừ' | 'ứ' | 'ử' | 'ữ' | 'ự' => {
                'u'
            }
            'Ù' | 'Ú' | 'Ủ' | 'Ũ' | 'Ụ' | 'Ư' | 'Ừ' | 'Ứ' | 'Ử' | 'Ữ' | 'Ự' => {
                'U'
            }
            'ỳ' | 'ý' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
            'Ỳ' | 'Ý' | 'Ỷ' | 'Ỹ' | 'Ỵ' => 'Y',
            other => other,
        };
        out.push(mapped);
    }
    out
}

// ============================================================================
// IPC #25: list_characters (4-Tier Search Algorithm §3.8)
// ============================================================================

/// Raw character record from database for evaluation in 4-tier search.
#[derive(Debug, Clone)]
pub struct RawCharacterRecord {
    pub id: Uuid,
    pub code: String,
    pub display_name: Option<String>,
    pub nickname: Option<String>,
    pub species: String,
    pub biological_sex: String,
    pub gender_identity: String,
    pub gender_expression: String,
    pub default_summary: Option<String>,
    pub in_project: bool,
    pub primary_project_id: Option<Uuid>,
}

impl RawCharacterRecord {
    pub fn resolved_name(&self) -> String {
        self.display_name
            .clone()
            .or_else(|| self.nickname.clone())
            .unwrap_or_else(|| self.code.clone())
    }

    pub fn to_dto(&self, project_id: Option<Uuid>) -> CharacterLibraryItemDto {
        let resolved_project_id = if project_id.is_some() {
            if self.in_project {
                project_id
            } else {
                self.primary_project_id
            }
        } else {
            self.primary_project_id
        };
        let resolved_in_project = if project_id.is_some() {
            self.in_project
        } else {
            self.primary_project_id.is_some()
        };
        CharacterLibraryItemDto {
            id: self.id,
            project_id: resolved_project_id,
            code: self.code.clone(),
            name: self.resolved_name(),
            nickname: self.nickname.clone(),
            species: self.species.clone(),
            biological_sex: self.biological_sex.clone(),
            gender_identity: self.gender_identity.clone(),
            gender_expression: self.gender_expression.clone(),
            in_project: resolved_in_project,
            avatar_url: None,
            default_summary: self.default_summary.clone(),
        }
    }
}

/// Executes the 4-tier search algorithm over characters (`REQ-CMD-025`, `core.md` §3.8).
///
/// Tiers:
/// 1. NFC normalization, case-insensitive.
/// 2. Exact match on display_name or nickname.
/// 3. Prefix match, then substring match.
/// 4. Vietnamese diacritic-insensitive match.
///
/// If `project_id` is supplied:
/// Searches within `in_project == true` records first.
/// If 0 matches found in project, fallbacks to searching all library records.
pub fn list_characters(
    conn: &Connection,
    project_id: Option<Uuid>,
    biological_sex: Option<&str>,
    query: Option<&str>,
) -> Result<Vec<CharacterLibraryItemDto>, IpcError> {
    // 1. Fetch active characters with in_project flag
    let proj_id_str = project_id.map(|p| p.to_string());

    let sql = "SELECT c.id, c.code, c.display_name, c.nickname, c.species,
                      c.biological_sex, c.gender_identity, c.gender_expression,
                      ca.default_summary,
                      CASE WHEN ?1 IS NOT NULL AND pc.id IS NOT NULL THEN 1 ELSE 0 END AS in_project,
                      (SELECT pc2.project_id
                       FROM project_characters pc2
                       JOIN projects p2 ON p2.id = pc2.project_id AND p2.deleted_at IS NULL
                       WHERE pc2.character_id = c.id AND pc2.deleted_at IS NULL
                       ORDER BY pc2.created_at DESC
                       LIMIT 1) AS primary_project_id
               FROM characters c
               LEFT JOIN character_appearances ca ON ca.character_id = c.id
               LEFT JOIN project_characters pc ON pc.character_id = c.id AND pc.project_id = ?1 AND pc.deleted_at IS NULL
               WHERE c.deleted_at IS NULL
               ORDER BY c.code ASC";

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![proj_id_str], |r| {
            let id_str: String = r.get(0)?;
            let id = Uuid::parse_str(&id_str).unwrap_or_default();
            let code: Option<String> = r.get(1)?;
            let display_name: Option<String> = r.get(2)?;
            let nickname: Option<String> = r.get(3)?;
            let species: Option<String> = r.get(4)?;
            let biological_sex: Option<String> = r.get(5)?;
            let gender_identity: Option<String> = r.get(6)?;
            let gender_expression: Option<String> = r.get(7)?;
            let default_summary: Option<String> = r.get(8)?;
            let in_proj_int: i64 = r.get(9)?;
            let primary_proj_str: Option<String> = r.get(10)?;
            let primary_project_id = primary_proj_str.and_then(|s| Uuid::parse_str(&s).ok());

            Ok(RawCharacterRecord {
                id,
                code: code.unwrap_or_default(),
                display_name,
                nickname,
                species: species.unwrap_or_else(|| "HUMAN".to_string()),
                biological_sex: biological_sex.unwrap_or_else(|| "NONE".to_string()),
                gender_identity: gender_identity.unwrap_or_else(|| "NONE".to_string()),
                gender_expression: gender_expression.unwrap_or_else(|| "NONE".to_string()),
                default_summary,
                in_project: in_proj_int == 1,
                primary_project_id,
            })
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut records: Vec<RawCharacterRecord> = Vec::new();
    for r in rows {
        let rec = r.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // Optional biological_sex filter
        if let Some(sex) = biological_sex {
            if !sex.trim().is_empty() && !rec.biological_sex.eq_ignore_ascii_case(sex.trim()) {
                continue;
            }
        }
        records.push(rec);
    }

    let raw_q = query.unwrap_or("").trim();
    if raw_q.is_empty() {
        // Return all matching records, in-project first, then name ascending
        records.sort_by(|a, b| {
            b.in_project
                .cmp(&a.in_project)
                .then_with(|| a.resolved_name().cmp(&b.resolved_name()))
        });
        return Ok(records.into_iter().map(|r| r.to_dto(project_id)).collect());
    }

    // 4-Tier Search implementation (§3.8)
    let q_nfc_lower = normalize_to_nfc(raw_q).to_lowercase();
    let q_unacc_lower = to_unaccented(&q_nfc_lower).to_lowercase();

    // Helper: evaluate tiers against candidate list
    let search_pool = |candidates: &[RawCharacterRecord]| -> Option<Vec<RawCharacterRecord>> {
        // Tier 2: Exact match on display_name or nickname
        let exact_matches: Vec<RawCharacterRecord> = candidates
            .iter()
            .filter(|c| {
                let name_match = c
                    .display_name
                    .as_deref()
                    .map(|n| normalize_to_nfc(n).to_lowercase() == q_nfc_lower)
                    .unwrap_or(false);
                let nick_match = c
                    .nickname
                    .as_deref()
                    .map(|n| normalize_to_nfc(n).to_lowercase() == q_nfc_lower)
                    .unwrap_or(false);
                name_match || nick_match
            })
            .cloned()
            .collect();

        if !exact_matches.is_empty() {
            return Some(exact_matches);
        }

        // Tier 3: Prefix match, then substring match
        let mut prefix_matches = Vec::new();
        let mut substring_matches = Vec::new();

        for c in candidates {
            let name_nfc = c
                .display_name
                .as_deref()
                .map(|n| normalize_to_nfc(n).to_lowercase());
            let nick_nfc = c
                .nickname
                .as_deref()
                .map(|n| normalize_to_nfc(n).to_lowercase());

            let is_prefix = name_nfc
                .as_deref()
                .map(|n| n.starts_with(&q_nfc_lower))
                .unwrap_or(false)
                || nick_nfc
                    .as_deref()
                    .map(|n| n.starts_with(&q_nfc_lower))
                    .unwrap_or(false);

            if is_prefix {
                prefix_matches.push(c.clone());
            } else {
                let is_substr = name_nfc
                    .as_deref()
                    .map(|n| n.contains(&q_nfc_lower))
                    .unwrap_or(false)
                    || nick_nfc
                        .as_deref()
                        .map(|n| n.contains(&q_nfc_lower))
                        .unwrap_or(false);
                if is_substr {
                    substring_matches.push(c.clone());
                }
            }
        }

        if !prefix_matches.is_empty() || !substring_matches.is_empty() {
            prefix_matches.extend(substring_matches);
            return Some(prefix_matches);
        }

        // Tier 4: Vietnamese diacritic-insensitive match
        let unacc_matches: Vec<RawCharacterRecord> = candidates
            .iter()
            .filter(|c| {
                let name_unacc = c
                    .display_name
                    .as_deref()
                    .map(|n| to_unaccented(n).to_lowercase())
                    .unwrap_or_default();
                let nick_unacc = c
                    .nickname
                    .as_deref()
                    .map(|n| to_unaccented(n).to_lowercase())
                    .unwrap_or_default();

                name_unacc.contains(&q_unacc_lower) || nick_unacc.contains(&q_unacc_lower)
            })
            .cloned()
            .collect();

        if !unacc_matches.is_empty() {
            return Some(unacc_matches);
        }

        None
    };

    if project_id.is_some() {
        // Step A: Search within project scope
        let in_project_candidates: Vec<RawCharacterRecord> =
            records.iter().filter(|r| r.in_project).cloned().collect();

        if let Some(matches) = search_pool(&in_project_candidates) {
            return Ok(matches.into_iter().map(|r| r.to_dto(project_id)).collect());
        }

        // Step B: 0 matches in project -> fallback to entire library (§3.8)
        if let Some(matches) = search_pool(&records) {
            return Ok(matches.into_iter().map(|r| r.to_dto(project_id)).collect());
        }

        Ok(Vec::new())
    } else {
        // Search entire library directly
        if let Some(matches) = search_pool(&records) {
            return Ok(matches.into_iter().map(|r| r.to_dto(None)).collect());
        }

        Ok(Vec::new())
    }
}

// ============================================================================
// IPC #26: get_character_detail
// ============================================================================

/// Retrieves complete character detail across core, life, and psych tabs (`REQ-CMD-026`).
pub fn get_character_detail(
    conn: &Connection,
    character_id: &Uuid,
    project_id: Option<&Uuid>,
) -> Result<CharacterDetailDto, IpcError> {
    let char_id_str = character_id.to_string();

    // 1. Fetch character row
    let mut stmt = conn
        .prepare(
            "SELECT c.id, c.code, c.display_name, c.nickname, c.species,
                    c.biological_sex, c.gender_identity, c.gender_expression,
                    ca.default_summary
             FROM characters c
             LEFT JOIN character_appearances ca ON ca.character_id = c.id
             WHERE c.id = ?1 AND c.deleted_at IS NULL",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut rows = stmt
        .query(params![char_id_str])
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let row = match rows
        .next()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
    {
        Some(r) => r,
        None => {
            return Err(IpcError::new(
                ErrorCode::EntityNotFound,
                format!("Character '{}' not found", character_id),
            ));
        }
    };

    let id_str: String = row
        .get(0)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let code: Option<String> = row
        .get(1)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let display_name: Option<String> = row
        .get(2)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let nickname: Option<String> = row
        .get(3)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let species: Option<String> = row
        .get(4)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let biological_sex: Option<String> = row
        .get(5)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let gender_identity: Option<String> = row
        .get(6)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let gender_expression: Option<String> = row
        .get(7)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let default_summary: Option<String> = row
        .get(8)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let primary_project_id: Option<Uuid> = conn
        .query_row(
            "SELECT pc2.project_id
             FROM project_characters pc2
             JOIN projects p2 ON p2.id = pc2.project_id AND p2.deleted_at IS NULL
             WHERE pc2.character_id = ?1 AND pc2.deleted_at IS NULL
             ORDER BY pc2.created_at DESC
             LIMIT 1",
            params![char_id_str],
            |r| r.get::<_, String>(0),
        )
        .ok()
        .and_then(|s| Uuid::parse_str(&s).ok());

    let (in_project, resolved_project_id) = if let Some(pid) = project_id {
        let linked = conn
            .query_row(
                "SELECT EXISTS (SELECT 1 FROM project_characters WHERE project_id = ?1 AND character_id = ?2 AND deleted_at IS NULL)",
                params![pid.to_string(), char_id_str],
                |r| r.get(0),
            )
            .unwrap_or(false);
        (
            linked,
            if linked {
                Some(*pid)
            } else {
                primary_project_id
            },
        )
    } else {
        (primary_project_id.is_some(), primary_project_id)
    };

    let character = CharacterLibraryItemDto {
        id: *character_id,
        project_id: resolved_project_id,
        code: code.unwrap_or_default(),
        name: display_name
            .clone()
            .or_else(|| nickname.clone())
            .unwrap_or_else(|| id_str.clone()),
        nickname,
        species: species.unwrap_or_else(|| "HUMAN".to_string()),
        biological_sex: biological_sex.unwrap_or_else(|| "NONE".to_string()),
        gender_identity: gender_identity.unwrap_or_else(|| "NONE".to_string()),
        gender_expression: gender_expression.unwrap_or_else(|| "NONE".to_string()),
        in_project,
        avatar_url: None,
        default_summary,
    };

    // 2. Fetch Appearance
    let appearance = fetch_appearance(conn, &char_id_str)?;

    // 3. Fetch Distinguishing Marks
    let distinguishing_marks = fetch_distinguishing_marks(conn, &char_id_str)?;

    // 4. Fetch Psychological
    let psychological = fetch_psychological(conn, &char_id_str)?;

    // 5. Fetch Memories
    let memories = fetch_memories(conn, &char_id_str)?;

    // 6. Fetch Competences
    let competences = fetch_competences(conn, &char_id_str)?;

    // 7. Fetch Relationships
    let relationships = fetch_relationships(conn, &char_id_str)?;

    Ok(CharacterDetailDto {
        character,
        appearance,
        distinguishing_marks,
        psychological,
        memories,
        competences,
        relationships,
    })
}

// ============================================================================
// IPC #27: create_or_update_library_character
// ============================================================================

/// Upserts character and all associated tabs within `BEGIN IMMEDIATE` (`REQ-CMD-027`).
///
/// Validates invariants:
/// - `display_name` non-empty or `nickname` non-empty.
/// - Every competence must have `scene_id` referencing an existing scene.
pub fn create_or_update_library_character(
    conn: &mut Connection,
    character_id: Option<Uuid>,
    project_id: Option<Uuid>,
    profile: &CharacterUpsertPayloadDto,
) -> Result<CharacterDetailDto, IpcError> {
    // Invariant check 1: display_name or nickname must be provided
    let has_name = !profile.name.trim().is_empty();
    let has_nick = profile
        .nickname
        .as_deref()
        .map(|n| !n.trim().is_empty())
        .unwrap_or(false);

    if !has_name && !has_nick {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "Character requires non-empty display_name or nickname",
        ));
    }

    // Invariant check 2: competence scene_id must not be null and must exist
    for comp in &profile.competences {
        let sc_id = comp.scene_id.ok_or_else(|| {
            IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Competence '{}' requires non-null scene_id",
                    comp.competence_name
                ),
            )
        })?;

        let scene_exists: bool = conn
            .query_row(
                "SELECT EXISTS (SELECT 1 FROM scenes WHERE id = ?1 AND deleted_at IS NULL)",
                params![sc_id.to_string()],
                |r| r.get(0),
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if !scene_exists {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!("Scene '{}' referenced by competence not found", sc_id),
            ));
        }
    }

    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Begin transaction failed: {}", e),
            )
        })?;

    let now = Utc::now().to_rfc3339();
    let cid = match character_id {
        Some(existing_id) => {
            let id_str = existing_id.to_string();
            let exists: bool = tx
                .query_row(
                    "SELECT EXISTS (SELECT 1 FROM characters WHERE id = ?1 AND deleted_at IS NULL)",
                    params![id_str],
                    |r| r.get(0),
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if !exists {
                return Err(IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Character '{}' not found", existing_id),
                ));
            }

            tx.execute(
                "UPDATE characters SET display_name = ?1, nickname = ?2, species = ?3,
                                       biological_sex = ?4, gender_identity = ?5, gender_expression = ?6
                 WHERE id = ?7",
                params![
                    profile.name.trim(),
                    profile.nickname.as_deref().map(|n| n.trim()),
                    profile.species.trim(),
                    profile.biological_sex.trim(),
                    profile.gender_identity.trim(),
                    profile.gender_expression.trim(),
                    id_str
                ],
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Update character failed: {}", e)))?;

            existing_id
        }
        None => {
            let new_id = Uuid::now_v7();
            let id_str = new_id.to_string();

            // Allocate candidate code
            let base_name = if !profile.name.trim().is_empty() {
                profile.name.trim()
            } else {
                profile.nickname.as_deref().unwrap_or("char")
            };
            let slug = CodeAllocator::slugify_name(base_name);
            let code = allocate_unique_character_code(&tx, &slug)?;

            tx.execute(
                "INSERT INTO characters (id, code, display_name, nickname, species, biological_sex, gender_identity, gender_expression)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    id_str,
                    code,
                    profile.name.trim(),
                    profile.nickname.as_deref().map(|n| n.trim()),
                    profile.species.trim(),
                    profile.biological_sex.trim(),
                    profile.gender_identity.trim(),
                    profile.gender_expression.trim(),
                ],
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Insert character failed: {}", e)))?;

            new_id
        }
    };

    let cid_str = cid.to_string();

    // Link to project if project_id provided
    if let Some(pid) = project_id {
        let pid_str = pid.to_string();
        let pc_id = Uuid::now_v7().to_string();
        tx.execute(
            "INSERT INTO project_characters (id, project_id, character_id, created_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT (project_id, character_id) DO UPDATE SET deleted_at = NULL",
            params![pc_id, pid_str, cid_str, now],
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Link project_characters failed: {}", e),
            )
        })?;
    }

    // Upsert Appearance
    if let Some(ref app) = profile.appearance {
        let hair_opt_id = ensure_appearance_option(&tx, "hair_color", &app.hair)?;
        let eye_opt_id = ensure_appearance_option(&tx, "iris_color", &app.eyes)?;
        let skin_opt_id = ensure_appearance_option(&tx, "skin_tone", &app.skin)?;

        let app_id = Uuid::now_v7().to_string();
        tx.execute(
            "INSERT INTO character_appearances (id, character_id, height, weight,
                                               hair_color_id, iris_color_id, skin_tone_id,
                                               hair_note, eyes_note, skin_note, outfit_style, default_summary)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
             ON CONFLICT (character_id) DO UPDATE SET
                 height = excluded.height,
                 weight = excluded.weight,
                 hair_color_id = excluded.hair_color_id,
                 iris_color_id = excluded.iris_color_id,
                 skin_tone_id = excluded.skin_tone_id,
                 hair_note = excluded.hair_note,
                 eyes_note = excluded.eyes_note,
                 skin_note = excluded.skin_note,
                 outfit_style = excluded.outfit_style,
                 default_summary = excluded.default_summary",
            params![
                app_id,
                cid_str,
                app.height as i64,
                app.weight as i64,
                hair_opt_id,
                eye_opt_id,
                skin_opt_id,
                app.hair_note.trim(),
                app.eyes_note.trim(),
                app.skin_note.trim(),
                app.outfit_style.trim(),
                app.default_summary.trim(),
            ],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Upsert appearance failed: {}", e)))?;
    }

    // Upsert Psychological
    if let Some(ref psych) = profile.psychological {
        let core_val_json = serde_json::to_string(&psych.core_values).unwrap_or_default();
        let triggers_json = serde_json::to_string(&psych.trauma_triggers).unwrap_or_default();
        let quirks_json = serde_json::to_string(&psych.behavioral_quirks).unwrap_or_default();

        tx.execute(
            "INSERT INTO character_psychological (character_id, the_wound, moral_code, stress_trigger, persona_type)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT (character_id) DO UPDATE SET
                 the_wound = excluded.the_wound,
                 moral_code = excluded.moral_code,
                 stress_trigger = excluded.stress_trigger,
                 persona_type = excluded.persona_type",
            params![cid_str, psych.summary.trim(), core_val_json, triggers_json, quirks_json],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Upsert psychological failed: {}", e)))?;
    }

    // Upsert Memories (replace existing)
    if !profile.memories.is_empty() {
        tx.execute(
            "DELETE FROM character_memories WHERE character_id = ?1",
            params![cid_str],
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Delete old memories failed: {}", e),
            )
        })?;

        for mem in &profile.memories {
            let mem_id = if mem.id.is_nil() {
                Uuid::now_v7().to_string()
            } else {
                mem.id.to_string()
            };
            let content_payload = json!({
                "memoryType": mem.memory_type,
                "title": mem.title,
                "description": mem.description,
                "emotionalCharge": mem.emotional_charge,
            })
            .to_string();

            tx.execute(
                "INSERT INTO character_memories (id, character_id, content, created_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![mem_id, cid_str, content_payload, now],
            )
            .map_err(|e| {
                IpcError::new(
                    ErrorCode::DatabaseError,
                    format!("Insert memory failed: {}", e),
                )
            })?;
        }
    }

    // Upsert Competences (replace existing)
    if !profile.competences.is_empty() {
        tx.execute(
            "DELETE FROM character_competences WHERE character_id = ?1",
            params![cid_str],
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Delete old competences failed: {}", e),
            )
        })?;

        for comp in &profile.competences {
            let comp_id = if comp.id.is_nil() {
                Uuid::now_v7().to_string()
            } else {
                comp.id.to_string()
            };
            let domain_id = ensure_competence_domain(&tx, &comp.domain)?;
            let sc_id_str = comp.scene_id.unwrap().to_string();

            tx.execute(
                "INSERT INTO character_competences (id, code, character_id, domain_id, scene_id, display_name)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    comp_id,
                    comp_id.clone(),
                    cid_str,
                    domain_id,
                    sc_id_str,
                    comp.competence_name.trim(),
                ],
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Insert competence failed: {}", e)))?;
        }
    }

    tx.commit().map_err(|e| {
        IpcError::new(
            ErrorCode::DatabaseError,
            format!("Commit character upsert failed: {}", e),
        )
    })?;

    get_character_detail(conn, &cid, project_id.as_ref())
}

// ----------------------------------------------------------------------------
// Internal Helpers for Queries and Sub-tables
// ----------------------------------------------------------------------------

fn fetch_appearance(
    conn: &Connection,
    char_id: &str,
) -> Result<Option<CharacterAppearanceDto>, IpcError> {
    let sql = "SELECT ca.height, ca.weight,
                      COALESCE(hc.display_name, hc.code, ca.hair_note, '') AS hair,
                      COALESCE(ec.display_name, ec.code, ca.eyes_note, '') AS eyes,
                      COALESCE(sc.display_name, sc.code, ca.skin_note, '') AS skin,
                      COALESCE(ca.hair_note, ''),
                      COALESCE(ca.eyes_note, ''),
                      COALESCE(ca.skin_note, ''),
                      COALESCE(ca.outfit_style, ''),
                      COALESCE(ca.default_summary, '')
               FROM character_appearances ca
               LEFT JOIN appearance_options hc ON ca.hair_color_id = hc.id
               LEFT JOIN appearance_options ec ON ca.iris_color_id = ec.id
               LEFT JOIN appearance_options sc ON ca.skin_tone_id = sc.id
               WHERE ca.character_id = ?1";

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut rows = stmt
        .query(params![char_id])
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    if let Some(r) = rows
        .next()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
    {
        let height_int: Option<i64> = r.get(0).ok();
        let weight_int: Option<i64> = r.get(1).ok();
        let hair: String = r.get(2).unwrap_or_default();
        let eyes: String = r.get(3).unwrap_or_default();
        let skin: String = r.get(4).unwrap_or_default();
        let hair_note: String = r.get(5).unwrap_or_default();
        let eyes_note: String = r.get(6).unwrap_or_default();
        let skin_note: String = r.get(7).unwrap_or_default();
        let outfit_style: String = r.get(8).unwrap_or_default();
        let default_summary: String = r.get(9).unwrap_or_default();

        Ok(Some(CharacterAppearanceDto {
            height: height_int.unwrap_or(0) as f32,
            weight: weight_int.unwrap_or(0) as f32,
            hair,
            eyes,
            skin,
            hair_note,
            eyes_note,
            skin_note,
            outfit_style,
            default_summary,
        }))
    } else {
        Ok(None)
    }
}

fn fetch_distinguishing_marks(conn: &Connection, char_id: &str) -> Result<Vec<String>, IpcError> {
    let mut stmt = conn
        .prepare("SELECT content FROM character_distinguishing_marks WHERE character_id = ?1 ORDER BY id ASC")
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![char_id], |r| r.get::<_, String>(0))
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut marks = Vec::new();
    for m in rows {
        marks.push(m.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?);
    }
    Ok(marks)
}

fn fetch_psychological(
    conn: &Connection,
    char_id: &str,
) -> Result<Option<CharacterPsychologicalDto>, IpcError> {
    let sql = "SELECT the_wound, moral_code, stress_trigger, persona_type
               FROM character_psychological
               WHERE character_id = ?1";

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut rows = stmt
        .query(params![char_id])
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    if let Some(r) = rows
        .next()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
    {
        let the_wound: Option<String> = r.get(0).ok();
        let moral_code: Option<String> = r.get(1).ok();
        let stress_trigger: Option<String> = r.get(2).ok();
        let persona_type: Option<String> = r.get(3).ok();

        let core_values = parse_string_list(moral_code);
        let trauma_triggers = parse_string_list(stress_trigger);
        let behavioral_quirks = parse_string_list(persona_type);
        let summary = the_wound.unwrap_or_default();

        Ok(Some(CharacterPsychologicalDto {
            core_values,
            trauma_triggers,
            behavioral_quirks,
            summary,
        }))
    } else {
        Ok(None)
    }
}

fn parse_string_list(raw: Option<String>) -> Vec<String> {
    match raw {
        Some(s) if !s.trim().is_empty() => {
            if let Ok(vec) = serde_json::from_str::<Vec<String>>(&s) {
                vec
            } else {
                s.split(';')
                    .map(|part| part.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect()
            }
        }
        _ => Vec::new(),
    }
}

fn fetch_memories(conn: &Connection, char_id: &str) -> Result<Vec<CharacterMemoryDto>, IpcError> {
    let mut stmt = conn
        .prepare("SELECT id, content FROM character_memories WHERE character_id = ?1 ORDER BY created_at ASC, id ASC")
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![char_id], |r| {
            let id_str: String = r.get(0)?;
            let id = Uuid::parse_str(&id_str).unwrap_or_default();
            let content: String = r.get(1)?;
            Ok((id, content))
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut memories = Vec::new();
    for r in rows {
        let (id, content) =
            r.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
            let memory_type = val["memoryType"].as_str().unwrap_or("EPISODIC").to_string();
            let title = val["title"].as_str().unwrap_or(&content).to_string();
            let description = val["description"].as_str().unwrap_or(&content).to_string();
            let emotional_charge = val["emotionalCharge"]
                .as_str()
                .unwrap_or("NEUTRAL")
                .to_string();
            memories.push(CharacterMemoryDto {
                id,
                memory_type,
                title,
                description,
                emotional_charge,
            });
        } else {
            memories.push(CharacterMemoryDto {
                id,
                memory_type: "EPISODIC".to_string(),
                title: content.clone(),
                description: content,
                emotional_charge: "NEUTRAL".to_string(),
            });
        }
    }
    Ok(memories)
}

fn fetch_competences(
    conn: &Connection,
    char_id: &str,
) -> Result<Vec<CharacterCompetenceDto>, IpcError> {
    let sql = "SELECT cc.id, cd.display_name, cc.display_name, dl.display_name, cc.scene_id
               FROM character_competences cc
               JOIN competence_domains cd ON cc.domain_id = cd.id
               LEFT JOIN domain_level dl ON cc.level_id = dl.id
               WHERE cc.character_id = ?1
               ORDER BY cc.id ASC";

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![char_id], |r| {
            let id_str: String = r.get(0)?;
            let id = Uuid::parse_str(&id_str).unwrap_or_default();
            let domain: String = r.get(1)?;
            let comp_name: String = r.get(2)?;
            let level: Option<String> = r.get(3)?;
            let sc_id_str: String = r.get(4)?;
            let scene_id = Uuid::parse_str(&sc_id_str).ok();

            Ok(CharacterCompetenceDto {
                id,
                domain,
                competence_name: comp_name,
                level: level.unwrap_or_else(|| "DEFAULT".to_string()),
                scene_id,
            })
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut competences = Vec::new();
    for c in rows {
        competences.push(c.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?);
    }
    Ok(competences)
}

fn fetch_relationships(
    conn: &Connection,
    char_id: &str,
) -> Result<Vec<CharacterRelationshipDto>, IpcError> {
    let sql = "SELECT r.to_character_id, COALESCE(rc.category, 'SOCIAL'),
                      COALESCE(rs.trust, 'MODERATE'), COALESCE(cm.description, '')
               FROM relationships r
               LEFT JOIN relationship_roles rr ON rr.relationship_id = r.id AND rr.is_primary = 1
               LEFT JOIN relationship_categories rc ON rc.id = rr.category_id
               LEFT JOIN relationship_states rs ON rs.relationship_id = r.id
               LEFT JOIN character_motifs cm ON cm.character_id = r.from_character_id
               WHERE r.from_character_id = ?1
               ORDER BY r.id ASC";

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![char_id], |r| {
            let to_char_str: String = r.get(0)?;
            let to_id = Uuid::parse_str(&to_char_str).unwrap_or_default();
            let rel_type: String = r.get(1)?;
            let intensity: String = r.get(2)?;
            let desc: String = r.get(3)?;

            Ok(CharacterRelationshipDto {
                target_character_id: to_id,
                relationship_type: rel_type,
                intensity,
                description: desc,
            })
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut rels = Vec::new();
    for r in rows {
        rels.push(r.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?);
    }
    Ok(rels)
}

fn ensure_appearance_option(
    conn: &Connection,
    group_code: &str,
    val: &str,
) -> Result<Option<String>, IpcError> {
    let val_trimmed = val.trim();
    if val_trimmed.is_empty() {
        return Ok(None);
    }

    // Check if matching code or display_name exists
    let existing_opt: Option<String> = conn
        .query_row(
            "SELECT id FROM appearance_options WHERE code = ?1 OR display_name = ?1 LIMIT 1",
            params![val_trimmed],
            |r| r.get(0),
        )
        .ok();

    if let Some(opt_id) = existing_opt {
        return Ok(Some(opt_id));
    }

    // Ensure group exists
    let group_id: String = conn
        .query_row(
            "SELECT id FROM appearance_option_groups WHERE code = ?1",
            params![group_code],
            |r| r.get(0),
        )
        .or_else(|_| {
            let gid = Uuid::now_v7().to_string();
            conn.execute(
                "INSERT INTO appearance_option_groups (id, code, display_name) VALUES (?1, ?2, ?3)",
                params![gid, group_code, group_code],
            )?;
            Ok::<String, rusqlite::Error>(gid)
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let new_opt_id = Uuid::now_v7().to_string();
    let opt_code = CodeAllocator::slugify_name(val_trimmed);
    conn.execute(
        "INSERT INTO appearance_options (id, group_id, code, display_name) VALUES (?1, ?2, ?3, ?4)",
        params![new_opt_id, group_id, opt_code, val_trimmed],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    Ok(Some(new_opt_id))
}

fn ensure_competence_domain(conn: &Connection, domain_name: &str) -> Result<String, IpcError> {
    let d_name = domain_name.trim();
    let d_code = CodeAllocator::slugify_name(d_name);

    let existing: Option<String> = conn
        .query_row(
            "SELECT id FROM competence_domains WHERE code = ?1 OR display_name = ?2 LIMIT 1",
            params![d_code, d_name],
            |r| r.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    let new_id = Uuid::now_v7().to_string();
    conn.execute(
        "INSERT INTO competence_domains (id, code, display_name, description) VALUES (?1, ?2, ?3, ?4)",
        params![new_id, d_code, d_name, d_name],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    Ok(new_id)
}

fn allocate_unique_character_code(conn: &Connection, base_slug: &str) -> Result<String, IpcError> {
    let base = if base_slug.is_empty() {
        "char"
    } else {
        base_slug
    };
    let mut candidate = base.to_string();
    let mut suffix = 2;

    loop {
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS (SELECT 1 FROM characters WHERE code = ?1)",
                params![candidate],
                |r| r.get(0),
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if !exists {
            return Ok(candidate);
        }
        candidate = format!("{base}_{suffix}");
        suffix += 1;
    }
}
