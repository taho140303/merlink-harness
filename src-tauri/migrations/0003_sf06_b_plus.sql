-- =============================================================================
-- Migration 0003: SF-06 B+ Physical Target Materialization (65 -> 66 Tables)
-- Authority: docs/db.dbml#L1299-L1378, docs/business/be-core.md (§3.4, §3.5, §12)
-- Target PRAGMA user_version: 3
-- Changes:
--   1. Create table #66: `workflow_round_receipts` (15 columns, composite FKs to draft_versions)
--   2. Rebuild table `workflow_task_receipts` to include 3 nullable B+ columns
--      (round_receipt_id, task_index, evidence_json), triplet check, and composite FK
--   3. Rebuild table `pending_patches` to include nullable eligibility_json column with json_valid check
-- =============================================================================

-- 66. workflow_round_receipts (docs/db.dbml:1300-1325)
CREATE TABLE workflow_round_receipts (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    workflow_run_id TEXT NOT NULL,
    plan_generation INTEGER NOT NULL CHECK (plan_generation > 0),
    round_index INTEGER NOT NULL CHECK (round_index >= 0),
    round_seal_key TEXT NOT NULL CHECK (length(round_seal_key) > 0),
    task_count INTEGER NOT NULL CHECK (task_count >= 0),
    parallel_limit INTEGER NOT NULL CHECK (parallel_limit >= 1),
    plan_digest TEXT NOT NULL CHECK (length(plan_digest) > 0),
    request_digest TEXT NOT NULL CHECK (length(request_digest) > 0),
    base_version_seq INTEGER NOT NULL,
    base_data_hash TEXT NOT NULL CHECK (length(base_data_hash) > 0),
    final_data_hash TEXT NOT NULL CHECK (length(final_data_hash) > 0),
    resulting_version_seq INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (draft_session_id, base_version_seq) REFERENCES draft_versions(draft_session_id, seq),
    FOREIGN KEY (draft_session_id, resulting_version_seq) REFERENCES draft_versions(draft_session_id, seq),
    UNIQUE (draft_session_id, workflow_run_id, plan_generation, round_index),
    UNIQUE (draft_session_id, round_seal_key),
    UNIQUE (id, draft_session_id, workflow_run_id, plan_generation, base_version_seq),
    CHECK ((resulting_version_seq IS NULL AND base_data_hash = final_data_hash) OR (resulting_version_seq IS NOT NULL AND base_data_hash <> final_data_hash))
);

-- Rebuild workflow_task_receipts to add SF-06 B+ columns and constraints (docs/db.dbml:1326-1350)
CREATE TABLE workflow_task_receipts_new (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    workflow_run_id TEXT NOT NULL,
    plan_generation INTEGER NOT NULL,
    task_identity TEXT NOT NULL,
    input_scope_fingerprint TEXT NOT NULL,
    output_scope_fingerprint TEXT NOT NULL,
    base_version_seq INTEGER NOT NULL,
    resulting_version_seq INTEGER,
    had_mutations INTEGER NOT NULL DEFAULT 0 CHECK (had_mutations IN (0, 1)),
    round_receipt_id TEXT REFERENCES workflow_round_receipts(id),
    task_index INTEGER,
    evidence_json TEXT CHECK (evidence_json IS NULL OR json_valid(evidence_json) = 1),
    created_at TEXT,
    FOREIGN KEY (round_receipt_id, draft_session_id, workflow_run_id, plan_generation, base_version_seq)
        REFERENCES workflow_round_receipts(id, draft_session_id, workflow_run_id, plan_generation, base_version_seq),
    UNIQUE (draft_session_id, workflow_run_id, plan_generation, task_identity),
    UNIQUE (round_receipt_id, task_index),
    CHECK ((had_mutations = 1 AND resulting_version_seq IS NOT NULL) OR (had_mutations = 0 AND resulting_version_seq IS NULL)),
    CHECK (
        (round_receipt_id IS NULL AND task_index IS NULL AND evidence_json IS NULL) OR
        (round_receipt_id IS NOT NULL AND task_index IS NOT NULL AND task_index >= 0 AND evidence_json IS NOT NULL AND json_valid(evidence_json) = 1)
    )
);

INSERT INTO workflow_task_receipts_new (
    id, draft_session_id, workflow_run_id, plan_generation, task_identity,
    input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
    resulting_version_seq, had_mutations, round_receipt_id, task_index,
    evidence_json, created_at
)
SELECT
    id, draft_session_id, workflow_run_id, plan_generation, task_identity,
    input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
    resulting_version_seq, had_mutations, NULL, NULL, NULL, created_at
FROM workflow_task_receipts;

DROP TABLE workflow_task_receipts;
ALTER TABLE workflow_task_receipts_new RENAME TO workflow_task_receipts;

CREATE INDEX idx_workflow_task_receipts_session_gen_task ON workflow_task_receipts(draft_session_id, plan_generation, task_identity);
CREATE INDEX idx_workflow_task_receipts_draft_session_id ON workflow_task_receipts(draft_session_id);

-- Rebuild pending_patches to add eligibility_json (docs/db.dbml:1357-1372)
CREATE TABLE pending_patches_new (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    base_version_seq INTEGER,
    target_type TEXT NOT NULL,
    target_code TEXT NOT NULL,
    field_name TEXT,
    operation TEXT NOT NULL CHECK (operation IN ('set', 'delete')),
    previous_value TEXT CHECK (previous_value IS NULL OR json_valid(previous_value) = 1),
    new_value TEXT CHECK (new_value IS NULL OR json_valid(new_value) = 1),
    target_source TEXT CHECK (target_source IS NULL OR target_source IN ('user', 'user_edited', 'ai_generated')),
    eligibility_json TEXT CHECK (eligibility_json IS NULL OR json_valid(eligibility_json) = 1),
    status TEXT DEFAULT 'pending' CHECK (status IS NULL OR status IN ('pending', 'confirmed', 'rejected', 'expired')),
    created_at TEXT,
    resolved_at TEXT
);

INSERT INTO pending_patches_new (
    id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation,
    previous_value, new_value, target_source, eligibility_json, status, created_at, resolved_at
)
SELECT
    id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation,
    previous_value, new_value, target_source, NULL, status, created_at, resolved_at
FROM pending_patches;

DROP TABLE pending_patches;
ALTER TABLE pending_patches_new RENAME TO pending_patches;

CREATE INDEX idx_pending_patches_session_status ON pending_patches(draft_session_id, status);

PRAGMA user_version = 3;
