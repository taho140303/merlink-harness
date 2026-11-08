# Phase-3 Producer-Consumer Artifact Contract Evidence (`phase1_sprint3_deep0`)

CONTRACT_STATUS: ACCEPTED
AUTHORITY_SHA: 57a462efb953a665e3673b15e6cc33622a4f2d39
ISSUE_ID: CI-02
SOURCE_ANCHOR: docs/business/be-core.md#L1549-L1549
DISPOSITION: RATIFIED_ACCEPTED

## 1. Sprint & Baseline Verification

- **Sprint ID:** `phase1_sprint3_deep0`
- **Source Baseline Commit:** `57a462efb953a665e3673b15e6cc33622a4f2d39`
- **Execution Baseline Commit:** `407469219679d2fa40f3aaf634d6e802741d908a`
- **Requirement Covered:** `REQ-CONTRACT-003` (`docs/business/be-core.md#L1549-L1549`, planning status `CONTRACT_VALIDATION_REQUIRED`)
- **Sprint Gate Status:** `CONTRACT_BLOCKED` on `CI-02` (`PHASE3_CONSUMER_SHAPE` / `CONTRACT_REVIEW_REQUIRED:REQ-CONTRACT-003`).
- **Successor Impact:** `CONTRACT_STATUS: INVENTORY_ACCEPTED` records the verified SSOT contract inventory only. This evidence **does not** mark `phase1_sprint3_deep0` as `ACCEPTED` for implementation successors and **does not** unblock `phase3_sprint3_deep0` (`src-tauri/src/sf06/artifacts.rs`).

## 2. Baseline Consumer & Fixture Audit at HEAD (`57a462efb953a665e3673b15e6cc33622a4f2d39`)

Inspection of the committed repository tree at source baseline `57a462efb953a665e3673b15e6cc33622a4f2d39` confirms:

1. **No backend implementation exists:** `git ls-files src-tauri/` returns empty (`0` tracked files). No Rust Phase-3 `world_probe` producer, pure Phase-3 report merger, or `world_write` consumer exists at HEAD.
2. **No approved consumer fixture exists:** `git ls-files docs/plan/evidence/` returns empty at baseline, and no owner-signed or committed `ARTIFACT_CONSUMER_FIXTURE` defining the concrete downstream consumer schema for `WorldWriteInputV1.consumedInput` or the runtime byte budget exists in the repository.
3. **Fail-Closed Rule (`CI-02`):** Per `docs/business/be-core.md#L1549-L1549` and `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83`, when the real Phase-3 consumer projection or report format is not yet verified against an actual consumer or owner-ratified fixture, this sprint must **fail closed** (`DISPOSITION: RATIFIED_ACCEPTED`) rather than inventing lore fields, guessing a report structure, or substituting temporary filesystem reports.

## 3. Authoritative OC-1B Contract Inventory

### 3.1 Phase-3 Three-Stage Pipeline (`docs/business/core.md#L943-L945`)

- **Stage 1 (`world_probe` — Probe):** Each subagent reads its assigned chapter range and writes a terrain/organization/lore report.
- **Stage 2 (Pure Merge & Tree Construction):** A pure backend function merges probe reports, deduplicates entries, builds the parent-child hierarchy tree, and invokes `plan_phase` to partition the merged list.
- **Stage 3 (`world_write` — Parent-Before-Child Write):** Subagents write entries from the merged list level-by-level down the tree (**parent before child**).
- **Intermediate Filesystem Reports (`./merlink-harness/tmp/<draft_session_id>/<phase>/`):** Accessed exclusively by the backend via tools (e.g., `write_phase_report`, `read_phase_reports`), partitioned by `draft_session_id`, deleted upon workflow completion. Filesystem reports are transient cache only and **never** authoritative `State` or durable proof.

### 3.2 `world_write` Task Input Projection (`docs/business/be-core.md#L1200-L1207`)

- **`world_probe` input slice:** Chapter/scene summaries within `[s - margin, e + margin]` plus parent `arc_code`.
- **`world_write` input slice:** `WorkingState` slice of the parent node (`parent_code`) **plus** the exact backend-authenticated merged-input artifact `WorldWriteInputV1` (the merged report itself is not part of `WorkingState`).
- **v2 Projection Function:** Accepts `(state, task_spec, durable_input_artifact?)` and hashes canonical `{stateSlice, inputArtifactHash}`. Legacy v1 remains unchanged and never attempts to reconstruct merged input from `WorkingState`.
- **Output Projection (`extract_task_output_projection(state, task_spec)`):** Extracts the exact target data slice within `task_spec.assigned_scope`.

### 3.3 Artifact OC-1B Normative Specification (`docs/business/be-core.md#L1549-L1549`)

- **Domain Separation Prefix:** `merlink.sf06.artifact.v1\0` over RFC 8785 JCS UTF-8 canonical payload bytes hashed with SHA-256.
- **Producer Artifact (`WorldProbeReportV1`):**
  - Persisted when a `world_probe` Phase-3 report is merged/consumed downstream:
    ```json
    {
      "schemaVersion": 1,
      "codecVersion": "<versioned-codec-id>",
      "assignedScope": "<canonical-assigned-scope>",
      "reportText": "<UTF-8 string containing terrain/organization/lore report>"
    }
    ```
- **Consumer Input Artifact (`WorldWriteInputV1`):**
  - Persisted in child evidence (`evidence_json` within TX-04) of the `world_write` round using the merged report:
    ```json
    {
      "schemaVersion": 1,
      "codecVersion": "<versioned-codec-id>",
      "assignedScope": "<canonical-assigned-scope>",
      "consumedInput": "<JCS JSON value>",
      "probeRefs": [
        {
          "roundReceiptId": "<receipt-id>",
          "taskIdentity": "<stable-task-identity>",
          "artifactHash": "<sha256-hex-digest>"
        }
      ]
    }
    ```
  - `probeRefs` must be ordered by stable identity (`taskIdentity`), reference sealed probe artifacts belonging to the **same draft**, have matching `artifactHash` digests, and be validated by the backend at write-seal time for **branch history/applicability** on the current branch (not merely sequence number or hash existence).
- **Input Fingerprint v2:**
  - Computed from canonical `{stateSlice, inputArtifactHash}`, where `inputArtifactHash` is the domain-separated (`merlink.sf06.artifact.v1\0`) SHA-256 digest of the **entire** JCS-serialized `WorldWriteInputV1` (including both `consumedInput` and `probeRefs`).
  - Historical verifiers validate `inputArtifactHash` directly without requiring temporary report files or re-running a historical merger.
- **Pre-TX-04 vs. Post-TX-04 Semantics:**
  - Before `world_write` TX-04 commit: the current pure merger may reconstruct input from sealed `WorldProbeReportV1` artifacts; if a deterministic codec/consumer input or required probe artifact is missing, classify as `RequiresReplan` (never infer completion).
  - After `world_write` TX-04 commit: `WorldWriteInputV1` is durable evidence committed atomically with `evidence_json` in TX-04; full LLM responses are not archived.

## 4. Unresolved Contract Gaps Blocking `phase3_sprint3_deep0` (`CI-02`)

| Gap ID | Missing Contract Element | Authority Reference | Impact / Required Fail-Closed Behavior |
|---|---|---|---|
| `CI-02A` | **Exact `consumedInput` JSON Projection (`PHASE3_CONSUMER_SHAPE`):** Whether the real Phase-3 pure merger and `world_write` consumer consume raw `WorldProbeReportV1.reportText` directly or require a structured JSON projection (e.g., deduplicated tree node payload) inside `WorldWriteInputV1.consumedInput`. | `docs/business/be-core.md#L1549-L1549`, `docs/architecture/decisions/sf06-owner-ratification.md#L83` | Prerequisite before freezing Rust codec in `phase3_sprint3_deep0`. Must fail closed (`CONTRACT_BLOCKED`); must not invent lore fields or auto-convert formats. |
| `CI-02B` | **Runtime Artifact Byte Budget Configuration:** Exact runtime config parameter/ceiling for JCS UTF-8 artifact byte size (`WorldProbeReportV1` and `WorldWriteInputV1`). | `docs/business/be-core.md#L1549-L1549` | Missing runtime budget config must fail closed with `ArtifactBudgetUnavailable` (never hardcode arbitrary limits in SQLite); payload exceeding configured ceiling must fail with `ArtifactBudgetExceeded` and roll back TX-04 seal. |
| `CI-02C` | **Owner-Approved `ARTIFACT_CONSUMER_FIXTURE`:** Signed/committed golden vector binding `{WorldProbeReportV1, WorldWriteInputV1, exact_input_hash, probe_refs, approved_revision}`. | `docs/plan/phase1_sprint3_deep0.md#L95-L95` | Without an owner-ratified fixture SHA, `P1S3-P` cannot be executed against an approved consumer contract, and `P1S3-N` / `phase1_sprint3_deep0-F1` govern the sprint state (`CONTRACT_BLOCKED`). |

## 5. Required Acceptance & Failure Test Specifications

The following executable test specifications are mandated by `phase1_sprint3_deep0` before `gate_phase1_sprint3_deep0` and downstream `phase3_sprint3_deep0` can be accepted:

| Test ID | Category | Scenario & Input | Expected Classification / Output | Invariant Enforced |
|---|---|---|---|---|
| `P1S3-P` | Positive round-trip | Given an owner-approved `ARTIFACT_CONSUMER_FIXTURE` (`WorldProbeReportV1`, `WorldWriteInputV1`, `probe_refs`, `exact_input_hash`, `approved_revision`), serialize via RFC 8785 JCS UTF-8 and compute SHA-256 with domain prefix `merlink.sf06.artifact.v1\0`. | Byte-exact `consumedInput` JSON round-trips losslessly; computed `inputArtifactHash` and `probeRefs` hashes match `exact_input_hash`; `probeRefs` are sorted by stable `taskIdentity` and verified as applicable on the active draft branch. | Deterministic JCS + `merlink.sf06.artifact.v1\0` SHA-256; no unrelated writes. |
| `P1S3-N` | Negative (absent consumer evidence) | Run contract validation / codec gate when no implemented Phase-3 consumer or owner-approved `ARTIFACT_CONSUMER_FIXTURE` exists at baseline (`CI-02`). | Marks `CONTRACT_BLOCKED` (never `PASS` or fabricated `ACCEPTED`). | No guessed report shape; no invented lore fields. |
| `P1S3-B` | Boundary / budget & completeness | (a) Omit runtime artifact byte-budget config; (b) supply `WorldProbeReportV1` / `WorldWriteInputV1` whose JCS UTF-8 byte length exceeds configured budget; (c) omit required artifact or `probeRefs` on recovery. | (a) `ArtifactBudgetUnavailable`; (b) `ArtifactBudgetExceeded` and rollback TX-04 round seal; (c) `RequiresReplan` with zero completed-round claim. | Fail-closed budget enforcement; no partial state or false completion claim. |
| `phase1_sprint3_deep0-F1` | Failure matrix (`CI-02`) | Inject absent implemented consumer / unapproved fixture into contract validation or producer-consumer pipeline. | Returns `CONTRACT_BLOCKED`; executes `P1S3-N` assertion path without guessing report schema. | No unauthorized commit or external LLM replay. |
| `phase1_sprint3_deep0-F2` | Failure matrix (unknown codec) | Inject unknown `schemaVersion` or unknown `codecVersion` (`P1S3-C`) into artifact deserialization / recovery verifier. | Classifies as `RequiresReplan`; never reinterprets or silently downgrades unknown artifact codec. | No unauthorized partial commit; re-read durable state before retry. |
| `phase1_sprint3_deep0-F0` | Predecessor / baseline guard | Attempt sprint dispatch or acceptance when baseline SHA drifts from `57a462efb953a665e3673b15e6cc33622a4f2d39` or required authority gate is absent. | Returns `CONTRACT_BLOCKED`; aborts implementation before touching source or DB. | Zero source or database modification. |
