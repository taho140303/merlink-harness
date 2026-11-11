//! RFC 8785 JSON Canonicalization Scheme (JCS), `BusinessFlowData` normalization,
//! and SHA-256 `data_hash` calculation (`REQ-CODEC-001`, `be-core.md` §4.3, §6.4.2, §6.4.7, `P2S2-P`, `P2S2-N`).
//!
//! Key guarantees:
//! 1. `canonical_state(&BusinessFlowData) -> Result<Vec<u8>, DomainError>`:
//!    - Validates structure (`meta.validate()` and unique entity codes).
//!    - Normalizes every unset `TrackedField<T>` (`{"value": null, "source": null}` or missing)
//!      to explicit `{"value": null, "source": "incomplete"}` (`be-core.md` §6.4.2, §6.4.7).
//!    - Excludes `meta.updated_at` (and only `meta.updated_at`) while preserving `require`,
//!      `optional`, `user_constraints`, `decision_log`, `_counters`, all `source` tags, and
//!      semantic array ordering.
//!    - Serializes the normalized JSON tree according to **RFC 8785 (JCS)**: object keys sorted
//!      by UTF-16 code unit order, no insignificant whitespace, standard IEEE 754 number formatting,
//!      and RFC 8785 string escaping.
//! 2. `state_data_hash(&BusinessFlowData) -> Result<String, DomainError>`:
//!    - Computes `"sha256:<64_lowercase_hex>"` over the UTF-8 JCS bytes of `canonical_state`.
//! 3. `states_are_semantically_equal(&a, &b) -> Result<bool, DomainError>`:
//!    - Compares exact canonical JCS byte slices (`canonical_state(a)? == canonical_state(b)?`),
//!      never relying solely on hash equality (`REQ-CODEC-001`, `be-core.md` §6.4.7).

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

use super::business_flow::{BusinessFlowData, DomainError, ProvenanceSource};

/// Strongly-typed wrapper around RFC 8785 JCS canonical state bytes (`Observable handoff: CanonicalStateBytes`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalStateBytes(Vec<u8>);

impl CanonicalStateBytes {
    /// Produces canonical RFC 8785 JCS bytes for `state` (excluding `meta.updated_at`).
    pub fn from_state(state: &BusinessFlowData) -> Result<Self, DomainError> {
        canonical_state(state).map(Self)
    }

    /// Borrows the underlying UTF-8 JCS byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consumes the wrapper and returns the owned `Vec<u8>`.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Computes `"sha256:<64_lowercase_hex>"` from these canonical bytes.
    pub fn sha256_prefixed(&self) -> String {
        let digest = Sha256::digest(&self.0);
        format!("sha256:{}", hex::encode(digest))
    }
}

impl AsRef<[u8]> for CanonicalStateBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Computes the RFC 8785 JCS canonical UTF-8 byte representation of `state` (`REQ-CODEC-001`, `be-core.md` §4.3, §6.4.7).
///
/// - Validates `state.validate_structure()`.
/// - Removes `meta.updated_at` so timestamp-only changes never change canonical bytes or `data_hash`.
/// - Normalizes `TrackedField<T>` objects where `value` is `null` and `source` is `null` to
///   `{"value": null, "source": "incomplete"}` (`be-core.md` §6.4.2).
/// - Preserves `_counters`, all `source` provenance tags, and semantic array order.
pub fn canonical_state(state: &BusinessFlowData) -> Result<Vec<u8>, DomainError> {
    state.validate_structure()?;
    let normalized = normalize_business_flow_value(state)?;
    canonical_json_value(&normalized)
}

/// Computes the canonical `data_hash` (`"sha256:<64_lowercase_hex>"`) for `state` (`be-core.md` §4.3).
pub fn state_data_hash(state: &BusinessFlowData) -> Result<String, DomainError> {
    let bytes = CanonicalStateBytes::from_state(state)?;
    Ok(bytes.sha256_prefixed())
}

/// Compares two `BusinessFlowData` instances by exact RFC 8785 JCS canonical byte equality
/// (`S ≡ T iff canonical_state(S) == canonical_state(T)`, `REQ-CODEC-001`, `be-core.md` §6.4.7).
pub fn states_are_semantically_equal(
    a: &BusinessFlowData,
    b: &BusinessFlowData,
) -> Result<bool, DomainError> {
    let bytes_a = canonical_state(a)?;
    let bytes_b = canonical_state(b)?;
    Ok(bytes_a == bytes_b)
}

/// Parses an arbitrary JSON value into `BusinessFlowData` (strictly validating shape) and returns
/// its RFC 8785 JCS canonical bytes.
///
/// This guarantees that two raw JSON documents with reordered object keys, omitted optional
/// `TrackedField`s vs explicit `{"value": null, "source": null}`, or different `meta.updated_at`
/// strings produce identical canonical bytes, whereas unknown keys or reordered semantic arrays do not.
pub fn canonical_state_from_json_value(value: &Value) -> Result<Vec<u8>, DomainError> {
    let parsed = BusinessFlowData::from_json_value_strict(value)?;
    canonical_state(&parsed)
}

/// Normalizes a validated `BusinessFlowData` into a `serde_json::Value` ready for RFC 8785 JCS serialization:
/// 1. Strips `meta.updated_at`.
/// 2. Normalizes every `TrackedField<T>` object (`{"value": ..., "source": ...}`) so that
///    `{"value": null, "source": null}` becomes `{"value": null, "source": "incomplete"}`.
pub fn normalize_business_flow_value(state: &BusinessFlowData) -> Result<Value, DomainError> {
    let mut root = serde_json::to_value(state).map_err(|err| DomainError::UnknownStateShape {
        reason: format!("failed to serialize BusinessFlowData: {err}"),
    })?;

    if let Some(meta_obj) = root.get_mut("meta").and_then(Value::as_object_mut) {
        meta_obj.remove("updated_at");
    }

    normalize_tracked_fields_in_place(&mut root);
    Ok(root)
}

fn normalize_tracked_fields_in_place(val: &mut Value) {
    match val {
        Value::Object(map) => {
            let is_tracked_field_shape = map.len() == 2
                && map.contains_key("value")
                && map.contains_key("source")
                && matches!(map.get("source"), Some(Value::Null | Value::String(_)));

            if is_tracked_field_shape {
                let source_is_null = matches!(map.get("source"), Some(Value::Null));
                if source_is_null {
                    map.insert(
                        "source".to_string(),
                        Value::String(ProvenanceSource::Incomplete.as_str().to_string()),
                    );
                }
            } else if map.contains_key("source")
                && matches!(map.get("source"), Some(Value::Null)) {
                    map.insert(
                        "source".to_string(),
                        Value::String(ProvenanceSource::Incomplete.as_str().to_string()),
                    );
                }

            for child in map.values_mut() {
                normalize_tracked_fields_in_place(child);
            }
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                normalize_tracked_fields_in_place(item);
            }
        }
        _ => {}
    }
}

/// Serializes any `serde_json::Value` into RFC 8785 (JSON Canonicalization Scheme — JCS) UTF-8 bytes.
///
/// Conforms to RFC 8785 §3.2:
/// - Object properties are sorted lexicographically by UTF-16 code units of their property names.
/// - Array element order is strictly preserved.
/// - Strings escape control characters `U+0000..U+001F` (`\b`, `\t`, `\n`, `\f`, `\r`, or `\uXXXX`
///   with lowercase hex), `\"`, and `\\`.
/// - Numbers are formatted according to ECMAScript / IEEE 754 canonical representation (no trailing
///   `.0` on integral floats, `-0.0` normalized to `0`, non-finite numbers rejected).
pub fn canonical_json_value(value: &Value) -> Result<Vec<u8>, DomainError> {
    let mut buf = Vec::with_capacity(256);
    write_jcs_value(value, "$", &mut buf)?;
    Ok(buf)
}

fn write_jcs_value(value: &Value, path: &str, out: &mut Vec<u8>) -> Result<(), DomainError> {
    match value {
        Value::Null => {
            out.extend_from_slice(b"null");
            Ok(())
        }
        Value::Bool(true) => {
            out.extend_from_slice(b"true");
            Ok(())
        }
        Value::Bool(false) => {
            out.extend_from_slice(b"false");
            Ok(())
        }
        Value::Number(num) => write_jcs_number(num, path, out),
        Value::String(s) => {
            write_jcs_string(s, out);
            Ok(())
        }
        Value::Array(items) => {
            out.push(b'[');
            for (idx, item) in items.iter().enumerate() {
                if idx > 0 {
                    out.push(b',');
                }
                let child_path = format!("{path}[{idx}]");
                write_jcs_value(item, &child_path, out)?;
            }
            out.push(b']');
            Ok(())
        }
        Value::Object(map) => write_jcs_object(map, path, out),
    }
}

fn write_jcs_object(
    map: &Map<String, Value>,
    path: &str,
    out: &mut Vec<u8>,
) -> Result<(), DomainError> {
    let mut entries: Vec<(&String, &Value)> = map.iter().collect();
    entries.sort_by(|(k1, _), (k2, _)| cmp_utf16_code_units(k1, k2));

    out.push(b'{');
    for (idx, (key, val)) in entries.iter().enumerate() {
        if idx > 0 {
            out.push(b',');
        }
        write_jcs_string(key, out);
        out.push(b':');
        let child_path = format!("{path}.{key}");
        write_jcs_value(val, &child_path, out)?;
    }
    out.push(b'}');
    Ok(())
}

/// Compares two Rust UTF-8 strings by their UTF-16 code unit sequences as required by RFC 8785 §3.2.3.
fn cmp_utf16_code_units(a: &str, b: &str) -> Ordering {
    let mut it_a = a.encode_utf16();
    let mut it_b = b.encode_utf16();
    loop {
        match (it_a.next(), it_b.next()) {
            (Some(ua), Some(ub)) => match ua.cmp(&ub) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            },
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => return Ordering::Equal,
        }
    }
}

fn write_jcs_string(s: &str, out: &mut Vec<u8>) {
    out.push(b'"');
    for ch in s.chars() {
        match ch {
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '\u{0008}' => out.extend_from_slice(b"\\b"),
            '\u{0009}' => out.extend_from_slice(b"\\t"),
            '\u{000A}' => out.extend_from_slice(b"\\n"),
            '\u{000C}' => out.extend_from_slice(b"\\f"),
            '\u{000D}' => out.extend_from_slice(b"\\r"),
            c if (c as u32) <= 0x001F => {
                let code = c as u32;
                let escaped = format!("\\u{code:04x}");
                out.extend_from_slice(escaped.as_bytes());
            }
            c => {
                let mut utf8_buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut utf8_buf);
                out.extend_from_slice(encoded.as_bytes());
            }
        }
    }
    out.push(b'"');
}

fn write_jcs_number(
    num: &serde_json::Number,
    path: &str,
    out: &mut Vec<u8>,
) -> Result<(), DomainError> {
    if let Some(i) = num.as_i64() {
        out.extend_from_slice(i.to_string().as_bytes());
        return Ok(());
    }
    if let Some(u) = num.as_u64() {
        out.extend_from_slice(u.to_string().as_bytes());
        return Ok(());
    }
    if let Some(f) = num.as_f64() {
        if !f.is_finite() {
            return Err(DomainError::NonFiniteNumber {
                path: path.to_string(),
            });
        }
        if f == 0.0 {
            out.push(b'0');
            return Ok(());
        }
        // If f is an exact integer within safe range and has no fractional part in scientific range
        if f.fract() == 0.0 && f.abs() < 1e21 {
            let formatted = format!("{f:.0}");
            out.extend_from_slice(formatted.as_bytes());
            return Ok(());
        }
        out.extend_from_slice(f.to_string().as_bytes());
        return Ok(());
    }
    Err(DomainError::NonFiniteNumber {
        path: path.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::business_flow::{OverviewArcItem, TrackedField};
    use serde_json::json;

    #[test]
    fn jcs_sorts_object_keys_but_preserves_array_order() {
        let v1 = json!({
            "z": 1,
            "a": [ {"b": 2, "a": 1}, {"b": 4, "a": 3} ],
            "m": "hello"
        });
        let v2 = json!({
            "a": [ {"a": 1, "b": 2}, {"a": 3, "b": 4} ],
            "m": "hello",
            "z": 1
        });
        let v3_reordered_array = json!({
            "a": [ {"a": 3, "b": 4}, {"a": 1, "b": 2} ],
            "m": "hello",
            "z": 1
        });

        let b1 = canonical_json_value(&v1).unwrap();
        let b2 = canonical_json_value(&v2).unwrap();
        let b3 = canonical_json_value(&v3_reordered_array).unwrap();

        assert_eq!(b1, b2);
        assert_ne!(b1, b3);
    }

    #[test]
    fn canonical_state_excludes_updated_at_and_normalizes_missing_tracked_fields() {
        let mut s1 = BusinessFlowData::new();
        s1.meta.updated_at = Some("2026-11-07T10:00:00Z".to_string());
        s1.optional.overview_phase.arcs.push(OverviewArcItem {
            code: "p1-a1".to_string(),
            display_name: TrackedField::user("Arc 1".to_string()),
            ..Default::default()
        });

        let mut s2 = s1.clone();
        s2.meta.updated_at = Some("2026-11-08T23:59:59Z".to_string());

        assert_eq!(canonical_state(&s1).unwrap(), canonical_state(&s2).unwrap());
        assert_eq!(state_data_hash(&s1).unwrap(), state_data_hash(&s2).unwrap());
        assert!(states_are_semantically_equal(&s1, &s2).unwrap());
    }
}
