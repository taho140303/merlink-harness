#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Acceptance and contract tests for `phase3_sprint3_deep0`
//! ("Validate and encode Phase-3 artifacts").
//!
//! Per the backend module wiring rule in `docs/plan/phase3_sprint3_deep0.md`:
//! - This sprint does not edit `src-tauri/src/lib.rs` or `src-tauri/src/domain/mod.rs`.
//! - Instead, this test target loads the owned implementation module directly via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `P3S3-P`: `consumedInput` fixture byte-matches consumer input and hash;
//!   canonical JCS byte-identity across key re-ordering; deterministic `probeRefs` sorting by `taskIdentity`.
//! - `P3S3-N`: Missing runtime budget rejects with `ArtifactBudgetUnavailable`;
//!   oversized payload rejects with `ArtifactBudgetExceeded`; unknown schema/codec rejects with fail-closed errors.
//! - `P3S3-B`: Restore-diverged `probeRef` cannot authorize `world_write`; cross-draft session,
//!   unsealed probe, or hash mismatch are rejected.
//! - `phase3_sprint3_deep0-F0`: Predecessor gate absent or baseline SHA drift returns `CONTRACT_BLOCKED`
//!   before implementation runs.
//! - `phase3_sprint3_deep0-F1`: Injecting absent consumer contract returns `CONTRACT_BLOCKED` (`CI-02`).
//! - `phase3_sprint3_deep0-F2`: Injecting missing artifact or missing budget returns `RequiresReplan`
//!   or budget failure, rejecting round seal.

mod common;

#[path = "../src/domain/phase3_artifact.rs"]
pub mod phase3_artifact;

use phase3_artifact::*;
use serde_json::{json, Value};
use std::path::PathBuf;

pub const BASELINE_COMMIT: &str = "ddda30cd3c842113599c7bfb7d0e221936330428";
pub const PREDECESSOR_P1S3_COMMIT: &str = "1e70d6f6d89b806b2243a1f4669eddce858d0b95";
pub const PREDECESSOR_P2S2_COMMIT: &str = "f69f1ab0e3d3d34853ddf887cac139cde6d64b0d";

/// Golden vector fixture content loaded from `fixtures/phase3_artifacts.json`.
fn load_golden_fixtures() -> Value {
    let fixture_str = include_str!("fixtures/phase3_artifacts.json");
    serde_json::from_str(fixture_str).expect("fixtures/phase3_artifacts.json must be valid JSON")
}

#[test]
#[allow(non_snake_case)]
fn P3S3_P_positive_fixture_byte_matching_and_deterministic_hashing() {
    let fixtures = load_golden_fixtures();

    // 1. Verify WorldProbeReportV1 golden vector
    let probe_golden = &fixtures["worldProbeReportGolden"];
    let probe_artifact: WorldProbeReportV1 =
        serde_json::from_value(probe_golden["artifact"].clone()).unwrap();

    let probe_jcs = probe_artifact.canonical_jcs_bytes().unwrap();
    let expected_probe_jcs = probe_golden["canonicalJcs"].as_str().unwrap().as_bytes();
    assert_eq!(
        probe_jcs, expected_probe_jcs,
        "Probe canonical JCS bytes must byte-match golden fixture"
    );

    let probe_hash = probe_artifact.compute_artifact_hash().unwrap();
    let expected_probe_hash = probe_golden["sha256Hash"].as_str().unwrap();
    assert_eq!(
        probe_hash, expected_probe_hash,
        "Probe artifact domain-separated SHA-256 must match golden fixture"
    );

    // 2. Key re-ordering invariance on WorldProbeReportV1
    let reordered_probe_json = json!({
        "reportText": probe_artifact.report_text,
        "assignedScope": probe_artifact.assigned_scope,
        "codecVersion": probe_artifact.codec_version,
        "schemaVersion": probe_artifact.schema_version,
    });
    let reordered_probe: WorldProbeReportV1 = serde_json::from_value(reordered_probe_json).unwrap();
    let reordered_probe_jcs = reordered_probe.canonical_jcs_bytes().unwrap();
    assert_eq!(
        reordered_probe_jcs, probe_jcs,
        "Key re-ordering in JSON input must produce byte-identical canonical JCS bytes"
    );
    assert_eq!(
        reordered_probe.compute_artifact_hash().unwrap(),
        probe_hash,
        "Key re-ordering in JSON input must produce identical domain-separated SHA-256 hash"
    );

    // 3. Verify WorldProbeReportV1 2nd golden vector
    let probe2_golden = &fixtures["worldProbeReportGolden2"];
    let probe2_artifact: WorldProbeReportV1 =
        serde_json::from_value(probe2_golden["artifact"].clone()).unwrap();
    let probe2_jcs = probe2_artifact.canonical_jcs_bytes().unwrap();
    let expected_probe2_jcs = probe2_golden["canonicalJcs"].as_str().unwrap().as_bytes();
    assert_eq!(probe2_jcs, expected_probe2_jcs);
    assert_eq!(
        probe2_artifact.compute_artifact_hash().unwrap(),
        probe2_golden["sha256Hash"].as_str().unwrap()
    );

    // 4. Verify WorldWriteInputV1 golden vector
    let write_golden = &fixtures["worldWriteInputGolden"];
    let write_artifact: WorldWriteInputV1 =
        serde_json::from_value(write_golden["artifact"].clone()).unwrap();

    let write_jcs = write_artifact.canonical_jcs_bytes().unwrap();
    let expected_write_jcs = write_golden["canonicalJcs"].as_str().unwrap().as_bytes();
    assert_eq!(
        write_jcs, expected_write_jcs,
        "WorldWriteInput canonical JCS bytes must byte-match golden fixture"
    );

    let write_hash = write_artifact.compute_artifact_hash().unwrap();
    let expected_write_hash = write_golden["sha256Hash"].as_str().unwrap();
    assert_eq!(
        write_hash, expected_write_hash,
        "WorldWriteInput domain-separated SHA-256 must match golden fixture"
    );

    // 5. Consumed input byte roundtrip losslessness
    let serialized_write = serde_json::to_string(&write_artifact).unwrap();
    let deserialized_write: WorldWriteInputV1 = serde_json::from_str(&serialized_write).unwrap();
    assert_eq!(
        deserialized_write.consumed_input, write_artifact.consumed_input,
        "consumedInput must roundtrip losslessly without schema modification"
    );

    // 6. Deterministic sorting of probeRefs
    assert!(
        write_artifact.is_probe_refs_sorted(),
        "probeRefs in golden fixture must be sorted by taskIdentity"
    );

    // 7. Verify input scope fingerprint v2 calculation
    let state_slice = json!({
        "parentCode": "arc_01",
        "existingWorldSettings": ["ws-iron-citadel"]
    });
    let fp_v2 = compute_input_scope_fingerprint_v2(&state_slice, &write_hash).unwrap();
    assert!(
        fp_v2.starts_with("sf06:v2:"),
        "v2 input scope fingerprint must have prefix 'sf06:v2:', got {fp_v2}"
    );
    assert_eq!(
        fp_v2.len(),
        8 + 64, // "sf06:v2:" is 8 chars, hex digest is 64 chars
        "v2 fingerprint must be exactly 72 characters"
    );
}

#[test]
#[allow(non_snake_case)]
fn P3S3_N_missing_budget_oversize_and_unknown_codec_reject() {
    let probe = WorldProbeReportV1::new(
        "p3:arc_01:ch=001..050",
        "Terrain: Northern jagged mountains; Organizations: Iron Citadel garrison; Lore: Legend of the Frost King.",
    );

    // 1. Missing budget configuration -> ArtifactBudgetUnavailable
    let err_missing_budget = probe.validate(None).unwrap_err();
    assert!(
        matches!(err_missing_budget, ArtifactError::ArtifactBudgetUnavailable),
        "Missing runtime budget must return ArtifactBudgetUnavailable, got {err_missing_budget:?}"
    );

    let err_encode_missing_budget = probe.encode_and_hash(None).unwrap_err();
    assert!(matches!(
        err_encode_missing_budget,
        ArtifactError::ArtifactBudgetUnavailable
    ));

    // 2. Oversized payload -> ArtifactBudgetExceeded
    let tight_budget = 50; // Much smaller than payload
    let err_oversize = probe.validate(Some(tight_budget)).unwrap_err();
    match err_oversize {
        ArtifactError::ArtifactBudgetExceeded {
            actual_bytes,
            budget_bytes,
        } => {
            assert_eq!(budget_bytes, 50);
            assert!(actual_bytes > 50);
        }
        other => panic!("Expected ArtifactBudgetExceeded, got {other:?}"),
    }

    // 3. WorldWriteInput oversized check with large payload
    let huge_text = "x".repeat(300_000); // 300 KiB > default budget 256 KiB
    let huge_write = WorldWriteInputV1::new(
        "p3:arc_01:world_write:depth=1",
        json!({ "hugePayload": huge_text }),
        vec![],
    );
    let err_huge = huge_write
        .validate(Some(DEFAULT_MAX_ARTIFACT_BUDGET))
        .unwrap_err();
    assert!(matches!(
        err_huge,
        ArtifactError::ArtifactBudgetExceeded { .. }
    ));

    // 4. Unknown schema version -> UnknownSchemaVersion
    let mut invalid_schema_probe = probe.clone();
    invalid_schema_probe.schema_version = 2;
    let err_schema = invalid_schema_probe
        .validate(Some(DEFAULT_MAX_ARTIFACT_BUDGET))
        .unwrap_err();
    assert_eq!(
        err_schema,
        ArtifactError::UnknownSchemaVersion { version: 2 }
    );

    // 5. Unknown codec version -> UnknownCodecVersion
    let mut invalid_codec_probe = probe.clone();
    invalid_codec_probe.codec_version = "v2".to_string();
    let err_codec = invalid_codec_probe
        .validate(Some(DEFAULT_MAX_ARTIFACT_BUDGET))
        .unwrap_err();
    assert_eq!(
        err_codec,
        ArtifactError::UnknownCodecVersion {
            version: "v2".to_string()
        }
    );

    // 6. Empty assigned scope -> Serialization error
    let mut empty_scope_probe = probe.clone();
    empty_scope_probe.assigned_scope = "   ".to_string();
    let err_scope = empty_scope_probe
        .validate(Some(DEFAULT_MAX_ARTIFACT_BUDGET))
        .unwrap_err();
    assert!(matches!(err_scope, ArtifactError::Serialization(_)));

    // 7. Unsorted probeRefs validation failure
    let unsorted_write = WorldWriteInputV1 {
        schema_version: 1,
        codec_version: "v1".to_string(),
        assigned_scope: "p3:arc_01:world_write:depth=1".to_string(),
        consumed_input: json!({"status": "ok"}),
        probe_refs: vec![
            ProbeRefEntry::new("r2", "p3:s0:world_probe:arc=arc_01:ch=051..100", "hash2"),
            ProbeRefEntry::new("r1", "p3:s0:world_probe:arc=arc_01:ch=001..050", "hash1"),
        ],
    };
    let err_unsorted = unsorted_write
        .validate(Some(DEFAULT_MAX_ARTIFACT_BUDGET))
        .unwrap_err();
    assert!(matches!(
        err_unsorted,
        ArtifactError::UnsortedProbeRefs { .. }
    ));
}

#[test]
#[allow(non_snake_case)]
fn P3S3_B_restore_diverged_and_invalid_probe_refs_cannot_authorize_world_write() {
    let probe1 = WorldProbeReportV1::new(
        "p3:arc_01:ch=001..050",
        "Report 1: Northern mountain ridge.",
    );
    let probe1_hash = probe1.compute_artifact_hash().unwrap();

    let probe2 =
        WorldProbeReportV1::new("p3:arc_01:ch=051..100", "Report 2: Southern valley pass.");
    let probe2_hash = probe2.compute_artifact_hash().unwrap();

    let write_input = WorldWriteInputV1::new(
        "p3:arc_01:world_write:depth=1",
        json!({ "mergeResult": "ok" }),
        vec![
            ProbeRefEntry::new(
                "rcpt-1",
                "p3:s0:world_probe:arc=arc_01:ch=001..050",
                &probe1_hash,
            ),
            ProbeRefEntry::new(
                "rcpt-2",
                "p3:s0:world_probe:arc=arc_01:ch=051..100",
                &probe2_hash,
            ),
        ],
    );

    let draft_session_id = "draft-session-42";
    let active_branch = "branch-feature-w";
    let diverged_branch = "branch-abandoned-restore";

    // 1. Positive case: both probes sealed, same draft, on active branch history -> Success
    let valid_context = DraftBranchContext::new(
        draft_session_id,
        active_branch,
        vec!["main".to_string(), active_branch.to_string()],
    );
    let valid_probes = vec![
        SealedProbeRecord::new(
            "rcpt-1",
            "p3:s0:world_probe:arc=arc_01:ch=001..050",
            draft_session_id,
            active_branch,
            &probe1_hash,
            true,
        ),
        SealedProbeRecord::new(
            "rcpt-2",
            "p3:s0:world_probe:arc=arc_01:ch=051..100",
            draft_session_id,
            "main", // In valid history
            &probe2_hash,
            true,
        ),
    ];
    assert!(
        write_input
            .validate_probe_applicability(&valid_context, &valid_probes)
            .is_ok(),
        "Valid sealed probes on active branch history must authorize world_write"
    );

    // 2. Restore divergence: probe2 resides on diverged/abandoned branch -> RestoreDivergedBranch
    let diverged_probes = vec![
        SealedProbeRecord::new(
            "rcpt-1",
            "p3:s0:world_probe:arc=arc_01:ch=001..050",
            draft_session_id,
            active_branch,
            &probe1_hash,
            true,
        ),
        SealedProbeRecord::new(
            "rcpt-2",
            "p3:s0:world_probe:arc=arc_01:ch=051..100",
            draft_session_id,
            diverged_branch, // NOT in valid history!
            &probe2_hash,
            true,
        ),
    ];
    let err_diverged = write_input
        .validate_probe_applicability(&valid_context, &diverged_probes)
        .unwrap_err();
    assert!(
        matches!(err_diverged, ArtifactError::RestoreDivergedBranch { .. }),
        "Restore-diverged branch probeRef must be rejected with RestoreDivergedBranch, got {err_diverged:?}"
    );

    // 3. Draft session mismatch: probe belongs to different draft session -> DraftSessionMismatch
    let foreign_probes = vec![
        SealedProbeRecord::new(
            "rcpt-1",
            "p3:s0:world_probe:arc=arc_01:ch=001..050",
            "other-draft-999", // Different draft session!
            active_branch,
            &probe1_hash,
            true,
        ),
        SealedProbeRecord::new(
            "rcpt-2",
            "p3:s0:world_probe:arc=arc_01:ch=051..100",
            draft_session_id,
            active_branch,
            &probe2_hash,
            true,
        ),
    ];
    let err_foreign = write_input
        .validate_probe_applicability(&valid_context, &foreign_probes)
        .unwrap_err();
    assert!(
        matches!(err_foreign, ArtifactError::DraftSessionMismatch { .. }),
        "Probe from a different draft session must be rejected with DraftSessionMismatch, got {err_foreign:?}"
    );

    // 4. Hash mismatch: artifact hash modified or forged -> ProbeHashMismatch
    let tampered_probes = vec![
        SealedProbeRecord::new(
            "rcpt-1",
            "p3:s0:world_probe:arc=arc_01:ch=001..050",
            draft_session_id,
            active_branch,
            "0000000000000000000000000000000000000000000000000000000000000000",
            true,
        ),
        SealedProbeRecord::new(
            "rcpt-2",
            "p3:s0:world_probe:arc=arc_01:ch=051..100",
            draft_session_id,
            active_branch,
            &probe2_hash,
            true,
        ),
    ];
    let err_hash = write_input
        .validate_probe_applicability(&valid_context, &tampered_probes)
        .unwrap_err();
    assert!(
        matches!(err_hash, ArtifactError::ProbeHashMismatch { .. }),
        "Mismatched probe artifact hash must be rejected with ProbeHashMismatch, got {err_hash:?}"
    );

    // 5. Unsealed probe artifact -> UnsealedProbeArtifact
    let unsealed_probes = vec![
        SealedProbeRecord::new(
            "rcpt-1",
            "p3:s0:world_probe:arc=arc_01:ch=001..050",
            draft_session_id,
            active_branch,
            &probe1_hash,
            false, // UNSEALED!
        ),
        SealedProbeRecord::new(
            "rcpt-2",
            "p3:s0:world_probe:arc=arc_01:ch=051..100",
            draft_session_id,
            active_branch,
            &probe2_hash,
            true,
        ),
    ];
    let err_unsealed = write_input
        .validate_probe_applicability(&valid_context, &unsealed_probes)
        .unwrap_err();
    assert!(
        matches!(err_unsealed, ArtifactError::UnsealedProbeArtifact { .. }),
        "Unsealed probe artifact cannot authorize world_write, got {err_unsealed:?}"
    );

    // 6. Missing probe reference entirely -> MissingProbeRef
    let incomplete_probes = vec![SealedProbeRecord::new(
        "rcpt-1",
        "p3:s0:world_probe:arc=arc_01:ch=001..050",
        draft_session_id,
        active_branch,
        &probe1_hash,
        true,
    )];
    let err_missing = write_input
        .validate_probe_applicability(&valid_context, &incomplete_probes)
        .unwrap_err();
    assert!(
        matches!(err_missing, ArtifactError::MissingProbeRef { .. }),
        "Missing probe record must return MissingProbeRef, got {err_missing:?}"
    );
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint3_deep0_F0_missing_predecessor_or_baseline_drift_is_contract_blocked() {
    // 1. Missing predecessor gate P1S3 -> CONTRACT_BLOCKED
    let err_p1s3 =
        verify_p3s3_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false, true)
            .unwrap_err();
    assert!(matches!(err_p1s3, ArtifactError::ContractBlocked(_)));
    assert!(err_p1s3.to_string().contains("gate_phase1_sprint3_deep0"));

    // 2. Missing predecessor gate P2S2 -> CONTRACT_BLOCKED
    let err_p2s2 =
        verify_p3s3_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, false)
            .unwrap_err();
    assert!(matches!(err_p2s2, ArtifactError::ContractBlocked(_)));
    assert!(err_p2s2.to_string().contains("gate_phase2_sprint2_deep1"));

    // 3. Baseline SHA drift -> CONTRACT_BLOCKED
    let drifted_baseline = "0000000000000000000000000000000000000000";
    let err_drift =
        verify_p3s3_dispatch_preconditions(BASELINE_COMMIT, drifted_baseline, true, true)
            .unwrap_err();
    assert!(matches!(err_drift, ArtifactError::ContractBlocked(_)));
    assert!(err_drift.to_string().contains("baseline SHA drift"));

    // 4. Positive verification against git repository lineage
    let _repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    // Check baseline is ancestor of HEAD
    assert!(
        common::verify_commit(BASELINE_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        BASELINE_COMMIT
    );

    // Check P1S3 accepted commit exists
    assert!(
        common::verify_commit(PREDECESSOR_P1S3_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P1S3_COMMIT
    );

    // Check P2S2 accepted commit exists
    assert!(
        common::verify_commit(PREDECESSOR_P2S2_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P2S2_COMMIT
    );

    // All valid -> Ok
    assert!(
        verify_p3s3_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, true).is_ok()
    );
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint3_deep0_F1_absent_consumer_contract_is_contract_blocked() {
    // 1. Unratified consumer contract -> CONTRACT_BLOCKED
    let err_unratified =
        verify_consumer_contract_ratified(false, Some("CI-02-APPROVED")).unwrap_err();
    assert!(matches!(err_unratified, ArtifactError::ContractBlocked(_)));
    assert!(err_unratified.to_string().contains("CI-02"));

    // 2. Missing fixture ID -> CONTRACT_BLOCKED
    let err_missing_fixture = verify_consumer_contract_ratified(true, None).unwrap_err();
    assert!(matches!(
        err_missing_fixture,
        ArtifactError::ContractBlocked(_)
    ));
    assert!(err_missing_fixture
        .to_string()
        .contains("absent owner-approved consumer contract"));

    // 3. Ratified contract and valid fixture -> Ok
    assert!(verify_consumer_contract_ratified(true, Some("ARTIFACT_CONSUMER_FIXTURE_V1")).is_ok());
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint3_deep0_F2_missing_artifact_or_budget_on_recovery_requires_replan() {
    let probe = WorldProbeReportV1::new(
        "p3:arc_01:ch=001..050",
        "Report 1: Northern mountain ridge.",
    );
    let probe_hash = probe.compute_artifact_hash().unwrap();

    let valid_write = WorldWriteInputV1::new(
        "p3:arc_01:world_write:depth=1",
        json!({ "summary": "ok" }),
        vec![ProbeRefEntry::new(
            "rcpt-1",
            "p3:s0:world_probe:arc=arc_01:ch=001..050",
            &probe_hash,
        )],
    );

    // 1. Missing budget on recovery -> ArtifactBudgetUnavailable
    let err_budget = verify_recovery_artifacts(Some(&probe), Some(&valid_write), None).unwrap_err();
    assert!(matches!(
        err_budget,
        ArtifactError::ArtifactBudgetUnavailable
    ));

    // 2. Missing write input artifact on recovery -> RequiresReplan (rejects seal, cannot fabricate completion)
    let err_missing_write =
        verify_recovery_artifacts(Some(&probe), None, Some(DEFAULT_MAX_ARTIFACT_BUDGET))
            .unwrap_err();
    assert!(
        matches!(err_missing_write, ArtifactError::RequiresReplan(_)),
        "Missing write input on recovery must require replan, got {err_missing_write:?}"
    );

    // 3. Empty probeRefs on recovery -> RequiresReplan
    let empty_refs_write = WorldWriteInputV1::new(
        "p3:arc_01:world_write:depth=1",
        json!({ "summary": "ok" }),
        vec![],
    );
    let err_empty_refs = verify_recovery_artifacts(
        Some(&probe),
        Some(&empty_refs_write),
        Some(DEFAULT_MAX_ARTIFACT_BUDGET),
    )
    .unwrap_err();
    assert!(
        matches!(err_empty_refs, ArtifactError::RequiresReplan(_)),
        "Empty probeRefs on recovery must require replan, got {err_empty_refs:?}"
    );

    // 4. Oversized payload on recovery -> ArtifactBudgetExceeded
    let err_oversized =
        verify_recovery_artifacts(Some(&probe), Some(&valid_write), Some(10)).unwrap_err();
    assert!(matches!(
        err_oversized,
        ArtifactError::ArtifactBudgetExceeded { .. }
    ));

    // 5. Valid recovery artifacts within budget -> Ok
    assert!(verify_recovery_artifacts(
        Some(&probe),
        Some(&valid_write),
        Some(DEFAULT_MAX_ARTIFACT_BUDGET)
    )
    .is_ok());
}
