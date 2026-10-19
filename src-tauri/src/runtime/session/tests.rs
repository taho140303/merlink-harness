use super::*;

use crate::db;
use crate::models::MessageEnvelope;
use crate::store;

/// Online end-to-end for the error path (Sprint 11.5, acceptance): a wrong key
/// Sprint 11.13 · D2: context usage needs a real configured provider, and
/// nothing ships any more, so the test writes one into its temp home.
fn save_openai_fixture() {
    let mut provider = store::catalog_fixture().remove(0);
    provider.api_key = "test-key".into();
    store::save_providers(&[provider]).unwrap();
}

/// must surface the provider's HTTP status instead of panicking or hanging.
#[test]
fn test_context_usage_splits_into_three_parts() {
    let _home = crate::test_support::TempHome::new("context-split");
    save_openai_fixture();
    let usage = context_usage_for_session("context-session", "openai", "gpt-4o").unwrap();
    assert_eq!(
        usage.system_tokens + usage.tool_tokens + usage.message_tokens,
        usage.used_tokens
    );
    assert!(usage.system_tokens > 0 && usage.tool_tokens > 0);
}

#[test]
fn test_context_usage_percent_matches_window() {
    let _home = crate::test_support::TempHome::new("context-percent");
    save_openai_fixture();
    let usage = context_usage_for_session("context-session", "openai", "gpt-4o").unwrap();
    assert_eq!(
        usage.percent,
        (100.0 * usage.used_tokens as f64 / usage.total_tokens as f64).round() as u8
    );
}

#[test]
fn test_context_usage_without_model_is_zero() {
    let _home = crate::test_support::TempHome::new("context-no-model");
    let usage = context_usage_for_session("context-session", "missing", "missing").unwrap();
    assert!(!usage.has_model);
    assert_eq!(
        usage.total_tokens + usage.used_tokens + u64::from(usage.percent),
        0
    );
}

#[test]
fn test_context_usage_grows_after_a_turn() {
    let _home = crate::test_support::TempHome::new("context-grows");
    save_openai_fixture();
    let before = context_usage_for_session("context-session", "openai", "gpt-4o").unwrap();
    let conn = db::open().unwrap();
    db::append_message(
        &conn,
        "context-session",
        &MessageEnvelope::user("m1", "a message after the turn"),
    )
    .unwrap();
    drop(conn);
    let after = context_usage_for_session("context-session", "openai", "gpt-4o").unwrap();
    assert!(after.used_tokens > before.used_tokens);
}
