# Contract Contradiction Ledger & Dependency-Specific Unblock Map

CONTRACT_STATUS: ACCEPTED
AUTHORITY_SHA: 57a462efb953a665e3673b15e6cc33622a4f2d39

## 1. Document Purpose, Authority Hierarchy, and Status Boundary

- **Sprint:** `phase1_sprint1_deep0` — Reconcile blocking SSOT contradictions (`REQ-CONTRACT-001`).
- **Source Baseline Commit (`AUTHORITY_SHA`):** `57a462efb953a665e3673b15e6cc33622a4f2d39`.
- **Execution Plan Lineage Commit:** `407469219679d2fa40f3aaf634d6e802741d908a`.
- **Authority Hierarchy:**
  1. `docs/architecture/decisions/sf06-owner-ratification.md` — Owner ratification & contract closure (`SPECIFICATION_READY_WITH_CONDITIONS`, `IMPLEMENTATION_NOT_VERIFIED`, `MIGRATION_NOT_EXECUTED`).
  2. `docs/business/core.md` — Product and business semantics SSOT.
  3. `docs/db.dbml` — Physical SQLite schema SSOT.
  4. Supporting specifications: `docs/business/be-core.md`, `docs/business/fe-core.md`, `docs/business/harness/tools.md`, `docs/business/prompts/overview.md`, `docs/design-system.md`.
- **Gate & Acceptance Boundary (`INVENTORY_ACCEPTED` vs `ACCEPTED`):**
  - Because no new owner-approved SSOT commit resolving open contradictions `CI-01`..`CI-11` exists beyond `57a462efb953a665e3673b15e6cc33622a4f2d39`, this ledger records `CONTRACT_STATUS: INVENTORY_ACCEPTED` and sets every open issue disposition to `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`.
  - Per `REQ-CONTRACT-001` (`docs/architecture/decisions/sf06-owner-ratification.md#L70-L83`) and `phase1_sprint1_deep0.md` §12/§16, `INVENTORY_ACCEPTED` satisfies the inventory deliverable of `phase1_sprint1_deep0` without inventing business, schema, or IPC policy, and **does not clear any `CI-xx` blocker** on downstream implementation sprints.

---

## 2. Public Contract Evidence Interface (`CONTRACT_EVIDENCE`)

| `issue_id` | `old_authority` | `proposed_disposition` | `owner_approval_sha` | `affected_sprints` |
| :--- | :--- | :--- | :--- | :--- |
| `CI-01` | `docs/business/be-core.md#L415-L491`, `docs/business/be-core.md#L2621-L2628`, `docs/db.dbml#L1299-L1378`, `docs/architecture/decisions/sf06-owner-ratification.md#L56-L58` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase1_sprint2_deep1`, `phase2_sprint3_deep1` |
| `CI-02` | `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83`, `docs/business/be-core.md#L1200-L1207`, `docs/business/be-core.md#L1549-L1549`, `docs/business/core.md#L943-L945` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase1_sprint3_deep0`, `phase3_sprint3_deep0`, `phase3_sprint9_deep2`, `phase3_sprint10_deep4`, `phase4_sprint1_deep0` |
| `CI-03` | `docs/business/be-core.md#L497-L506`, `docs/business/be-core.md#L562-L679`, `docs/business/core.md#L662-L732`, `docs/db.dbml#L1230-L1297`, `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase3_sprint2_deep0` |
| `CI-04` | `docs/business/be-core.md#L175-L221`, `docs/business/be-core.md#L493-L506`, `docs/business/be-core.md#L1530-L1550`, `docs/business/be-core.md#L2110-L2251`, `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase2_sprint4_deep3`, `phase2_sprint5_deep2` |
| `CI-05` | `docs/business/core.md#L734-L768`, `docs/business/be-core.md#L501-L501`, `docs/business/be-core.md#L1582-L1679` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase4_sprint2_deep0` |
| `CI-06` | `docs/business/core.md#L224-L335`, `docs/business/be-core.md#L502-L502`, `docs/business/be-core.md#L1715-L1755`, `docs/business/fe-core.md#L313-L313` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase4_sprint4_deep1` |
| `CI-07` | `docs/db.dbml#L958-L997`, `docs/business/core.md#L425-L429`, `docs/business/be-core.md#L400-L413`, `docs/business/be-core.md#L692-L692` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase2_sprint3_deep1` |
| `CI-08` | `docs/business/core.md#L291-L297`, `docs/business/be-core.md#L2110-L2158`, `docs/business/be-core.md#L2231-L2243`, `docs/business/fe-core.md#L294-L440` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase5_sprint5_deep3` |
| `CI-09` | `docs/business/be-core.md#L990-L1002`, `docs/business/be-core.md#L2169-L2203`, `docs/business/fe-core.md#L78-L80`, `docs/business/fe-core.md#L488-L539` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase5_sprint2_deep1` |
| `CI-10` | `docs/business/be-core.md#L2070-L2098`, `docs/business/be-core.md#L2152-L2155`, `docs/business/be-core.md#L2245-L2251`, `docs/business/fe-core.md#L32-L80` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase2_sprint4_deep3`, `phase5_sprint6_deep1` |
| `CI-11` | `docs/architecture/decisions/sf06-owner-ratification.md#L3-L17`, `docs/architecture/decisions/sf06-owner-ratification.md#L56-L83`, `docs/db.dbml#L1299-L1378`, `docs/business/be-core.md#L385-L491`, `docs/business/be-core.md#L1508-L1578` | `CONTRACT_BLOCKED` | `NONE` (baseline `57a462efb953a665e3673b15e6cc33622a4f2d39`) | `phase2_sprint3_deep1`, `phase3_sprint9_deep2` |

---

## 3. Source-Anchored Contradiction Ledger (`CI-01` .. `CI-11`)

### CI-01 — Migration baseline `0001`/`0002` absent; version path unapproved

- `ISSUE_ID: CI-01`
- `SOURCE_ANCHOR: docs/business/be-core.md#L415-L491; docs/business/be-core.md#L2621-L2628; docs/db.dbml#L1299-L1378; docs/architecture/decisions/sf06-owner-ratification.md#L56-L58`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/business/be-core.md#L421-L423`, `docs/business/be-core.md#L2623-L2623`):** Assumes pre-existing historical SQL migration files `0001_initial_schema.sql` (`PRAGMA user_version = 1`) and `0002_seed_reference_data.sql` (`PRAGMA user_version = 2`) exist in the repository and must be kept immutable while adding a forward-only B+ migration step (`0003` / `MAX_SUPPORTED_SCHEMA_VERSION`) for the 66th table (`workflow_round_receipts`), `workflow_task_receipts` rebuild, and nullable `pending_patches.eligibility_json`.
  - **Claim B (`docs/architecture/decisions/sf06-owner-ratification.md#L3-L5`, `docs/architecture/decisions/sf06-owner-ratification.md#L56-L58`, `docs/db.dbml#L1299-L1378`, and Git baseline tree at `57a462efb953a665e3673b15e6cc33622a4f2d39`):** Zero `.sql` files or `src-tauri/` directories exist in the committed repository (`MIGRATION_NOT_EXECUTED`). `docs/db.dbml` already embeds the 66-table SF-06 B+ target schema directly, leaving no committed 65-table SQL baseline artifact or ratified `user_version` numbering path (`1..2` vs `1..3`).
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase1_sprint2_deep1, phase2_sprint3_deep1`
- `REQUIRED_OWNER_DECISION:`
  - Must the greenfield SQLite bootstrap author a 65-table pre-B+ schema in `0001_initial_schema.sql`, reference seeds in `0002_seed_reference_data.sql`, and B+ upgrade DDL in `0003_sf06_bplus.sql` (`MAX_SUPPORTED_SCHEMA_VERSION = 3`), **or** bootstrap directly to the 66-table DBML schema in `0001` (`MAX_SUPPORTED_SCHEMA_VERSION = 2`) with an owner-approved 65-table legacy fixture for `MIG-02` upgrade testing?

### CI-02 — Phase-3 merger/consumer exact projection unproven

- `ISSUE_ID: CI-02`
- `SOURCE_ANCHOR: docs/architecture/decisions/sf06-owner-ratification.md#L81-L83; docs/business/be-core.md#L1200-L1207; docs/business/be-core.md#L1549-L1549; docs/business/core.md#L943-L945`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/business/be-core.md#L1200-L1207`, `docs/business/be-core.md#L1549-L1549`, `docs/business/core.md#L943-L945`):** Requires Phase-3 `world_probe` to persist `WorldProbeReportV1 = {schemaVersion:1, codecVersion, assignedScope, reportText}` and Phase-3 `world_write` to persist `WorldWriteInputV1 = {schemaVersion:1, codecVersion, assignedScope, consumedInput, probeRefs}` inside `workflow_task_receipts.evidence_json` (`TX-04`), where `consumedInput` is the exact JCS JSON value delivered to the downstream consumer after the pure backend merger deduplicates and builds the parent-child tree.
  - **Claim B (`docs/architecture/decisions/sf06-owner-ratification.md#L81-L83`, `docs/business/be-core.md#L1549-L1549`):** Explicitly warns that no Phase-3 merger or `world_write` consumer exists in the repository at `57a462efb953a665e3673b15e6cc33622a4f2d39`, so `WorldProbeReportV1.reportText` and `WorldWriteInputV1.consumedInput` are unproven against an actual consumer; if the consumer requires structured fields or a specific projection, the versioned codec must be ratified before implementation rather than invented by a coding agent.
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase1_sprint3_deep0, phase3_sprint3_deep0, phase3_sprint9_deep2, phase3_sprint10_deep4, phase4_sprint1_deep0`
- `REQUIRED_OWNER_DECISION:`
  - Ratify the exact canonical JSON schema of `WorldProbeReportV1.reportText` (or structured probe payload), the pure Phase-3 merger projection into `WorldWriteInputV1.consumedInput`, and the runtime artifact byte-budget configuration rule (`ArtifactBudgetUnavailable` / `ArtifactBudgetExceeded`).

### CI-03 — `seal_pending_retry`, TX-01 pending expiry, restore idempotency dedupe, and nullable legacy `phase`

- `ISSUE_ID: CI-03`
- `SOURCE_ANCHOR: docs/business/be-core.md#L497-L500; docs/business/be-core.md#L562-L679; docs/business/core.md#L48-L54; docs/business/core.md#L662-L732; docs/db.dbml#L1230-L1297; docs/architecture/decisions/sf06-owner-ratification.md#L80-L83`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Sub-contradiction 3a (`seal_pending_retry` state):** `docs/business/be-core.md#L660-L662` states that when `seal_draft_version` fails twice with `ErrorCode::VersionSealFailed`, the backend must keep `WorkingState` in RAM and mark the draft session in state `seal_pending_retry`. However, `docs/db.dbml#L1230-L1234` (`enum DraftSessionStatus`) and `docs/business/core.md#L725-L725` define only `{editing, committing, committed}` and forbid unapproved status values.
  - **Sub-contradiction 3b (`TX-01` pending expiry):** `docs/business/be-core.md#L497-L497` (`TX-01` table inventory) states `TX-01` reads/writes `draft_sessions`, `draft_versions`, and `pending_patches` (updating `expired` if any), whereas `docs/business/be-core.md#L589-L647` (`seal_draft_version_tx` pseudocode) touches only `draft_versions` and `draft_sessions`, and `docs/business/be-core.md#L504-L504` assigns `pending_patches` insert/dedupe/expire+replace/lazy-expiry to `TX-08` (and `TX-03` on restore).
  - **Sub-contradiction 3c (Restore no-op dedupe vs append-only `TX-03`):** `docs/business/be-core.md#L616-L625` (`seal_draft_version_tx` Step 3) skips creating a new `draft_versions` row when `head.data_hash == req.data_hash && head.phase == req.phase && head.subphase == req.subphase && req.reason != DraftVersionReason::PhaseConfirm` (commenting "trừ phase_confirm và restore sang phase khác"), whereas `docs/business/core.md#L719-L719` and `docs/business/be-core.md#L499-L499`, `docs/business/be-core.md#L664-L679` (`TX-03`) require `restore_draft_version` to append `seq = current_head_seq + 1` (`reason = 'restore'`, `parent_seq = target_seq`) and expire all `pending_patches` in `TX-03`.
  - **Sub-contradiction 3d (Nullable legacy `draft_versions.phase`):** `docs/db.dbml#L1278-L1297` and `docs/architecture/decisions/sf06-owner-ratification.md#L80-L81` note that `draft_versions.phase` is nullable in DBML (`phase int`) and SQLite `CHECK ((phase = 5 AND subphase IN (1, 2, 3)) OR (phase <> 5 AND subphase IS NULL))` evaluates to `UNKNOWN` (passing) when `phase IS NULL`, requiring explicit backend enforcement of `phase IS NOT NULL` on new versions and explicit handling when restoring or verifying legacy rows with `phase IS NULL`.
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase3_sprint2_deep0`
- `REQUIRED_OWNER_DECISION:`
  - (1) Confirm that `seal_pending_retry` is strictly an in-memory `DraftRuntimeEntry` flag (never written to `draft_sessions.status`); (2) clarify whether `TX-01` performs value-comparison lazy expiry on `pending_patches` during seal or delegates all proposal expiry to `TX-08`/`TX-03`/read-time lazy expiry; (3) specify whether `restore_draft_version` (`TX-03`) always appends a new version (`seq = head + 1`) and expires pending patches even when `target_ver.data_hash == head.data_hash` in the same phase; and (4) define the exact fallback or rejection behavior when restoring a legacy `draft_versions` row whose `phase IS NULL`.

### CI-04 — `ErrorCode` variants incomplete for IPC/error contracts and extra durable commands outside TX inventory

- `ISSUE_ID: CI-04`
- `SOURCE_ANCHOR: docs/business/be-core.md#L175-L221; docs/business/be-core.md#L493-L506; docs/business/be-core.md#L1530-L1550; docs/business/be-core.md#L2110-L2251; docs/architecture/decisions/sf06-owner-ratification.md#L81-L83`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Sub-contradiction 4a (Incomplete `ErrorCode` enum surface):** `docs/business/be-core.md#L188-L212` defines `pub enum ErrorCode` with 21 variants (`DraftBusy`, `DraftNotEditable`, `DraftReopenForbidden`, `PendingPatchesExist`, `ValidationFailed`, `CodeCollisionOnCommit`, `VersionSealFailed`, `SealKeyPayloadMismatch`, `IdempotencyKeyPayloadMismatch`, `StaleWorkflowReceipt`, `SceneLocked`, `OutOfEditScope`, `GenerationInvalidatedByLock`, `OccVersionMismatch`, `UnauthorizedCaller`, `EntityNotFound`, `ProviderUnreachable`, `ProviderAuthFailed`, `UnsupportedSchemaVersion`, `MigrationFailed`, `DatabaseError`, `InternalError` — 22 listed). Meanwhile, normative sections in `be-core.md` and `core.md` require structured errors not present in `ErrorCode`: `WorkflowBaselineUnavailable` (`be-core.md#L1578`, `core.md#L987`), `EvidenceCodecUnsupported`, `UnauthorizedTransition`, `PatchPreconditionFailed`, `UnsupportedEvidenceCodec` (`be-core.md#L1530`), `PatchExpired`, `ProposalEligibilityConflict`, `ProposalEligibilityIndeterminate` (`be-core.md#L1542-L1547`), `ArtifactBudgetUnavailable`, `ArtifactBudgetExceeded` (`be-core.md#L1549`), and `WorkflowInvariantViolation` (`be-core.md#L2720`).
  - **Sub-contradiction 4b (Extra durable SQLite-mutating IPC commands outside the 8-TX inventory):** `docs/business/be-core.md#L493-L506` mandates that *every* durable data modification in Merlink Harness `MUST` fit inside one of the **8** atomic transaction boundaries (`TX-01..TX-08`). However, `docs/business/be-core.md#L2118-L2244` specifies durable `BEGIN IMMEDIATE` SQLite writes in multiple IPC commands that are not mapped to any of `TX-01..TX-08`:
    - Command #1 `create_or_load_chat_session` (`be-core.md#L2118`, `#L2167`): `BEGIN IMMEDIATE` inserting `sessions` + `draft_sessions`.
    - Command #3 `send_chat_message` (`be-core.md#L2120`, `#L2169`): durable writes of `user` message and `tool_result` audit rows into `messages` before/during the ReAct loop outside `TX-01`.
    - Command #12 `list_pending_patches` (`be-core.md#L2129`, `#L2193`): lazy-expiry `UPDATE pending_patches SET status = 'expired'`.
    - Command #21 `reopen_committed_draft` (`be-core.md#L2138`, `#L2212`): `BEGIN IMMEDIATE` updating `draft_sessions(status = 'editing', session_id)`.
    - Command #24 `delete_project` (`be-core.md#L2141`, `#L2219`): `BEGIN IMMEDIATE` project-scoped soft-delete (`deleted_at`).
    - Command #27 `create_or_update_library_character` (`be-core.md#L2144`, `#L2228`): `BEGIN IMMEDIATE` upserting across `characters`, `project_characters`, `character_appearances`, `character_psychological`, `character_memories`, `character_competences`.
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase2_sprint4_deep3, phase2_sprint5_deep2`
- `REQUIRED_OWNER_DECISION:`
  - (1) Ratify the complete, closed list of `ErrorCode` enum variants vs command-specific DTO status enums (e.g., whether SF-06/patch/artifact errors are top-level `ErrorCode` variants or fields in `IpcError.details` / command response DTOs); and (2) reconcile `be-core.md` §3.6 (`TX-01..TX-08`) with the durable SQLite transactions in IPC commands `#1`, `#3`, `#12`, `#21`, `#24`, and `#27`.

### CI-05 — TX-05 table count and topological insert order differ between Core and BE

- `ISSUE_ID: CI-05`
- `SOURCE_ANCHOR: docs/business/core.md#L734-L768; docs/business/be-core.md#L501-L501; docs/business/be-core.md#L1582-L1679`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/business/core.md#L740-L740`):** Specifies the `BEGIN IMMEDIATE` topological FK insertion order during `commit_draft` as:
    `projects` → `genres` → `characters` → `project_characters` → `arcs/themes` → `chapters` → `world_settings` (parent before child) → `scenes` → `lore` → scene links (`scene_characters`, `scene_lore_entries`) → `appearance/profile/memory/competence` → `relationships`, followed by updating `draft_sessions` and `sessions`. Notice `world_settings` and `scenes` precede `lore` (`lore_entries`).
  - **Claim B (`docs/business/be-core.md#L501-L501`, `docs/business/be-core.md#L1625-L1651`):** Summarizes `TX-05` in §3.6 (`#L501`) as inserting into "21 bảng thực thể nghiệp vụ" plus updating `draft_sessions` and `sessions`, but the §7.2 table (`#L1625-L1651`) lists **22 numbered steps** that actually touch **24 physical business tables** (Step 17 groups `character_appearances` + `character_distinguishing_marks`; Step 20 groups `character_competences` + `competence_capabilities` + `competence_limitations`; Step 21 groups `relationships` + `relationship_roles` + `relationship_states`, plus 18 single-table steps = 24 entity tables + 2 session tables in Step 22 = 26 tables total) and orders Step 12 `world_settings` → Step 13 `lore_entries` → Step 14 `scenes` (placing `lore_entries` *before* `scenes`, contrary to `core.md#L740`).
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase4_sprint2_deep0`
- `REQUIRED_OWNER_DECISION:`
  - Ratify the exact table count (24 business entity tables + `draft_sessions` + `sessions`) and the authoritative topological insertion order between `scenes` and `lore_entries` (note that `lore_entries.world_setting_id` references `world_settings.id`, while `scene_lore_entries` references both `scenes.id` and `lore_entries.id`).

### CI-06 — TX-06 cancel-before-persist failure ordering on scene lock

- `ISSUE_ID: CI-06`
- `SOURCE_ANCHOR: docs/business/core.md#L210-L255; docs/business/be-core.md#L502-L502; docs/business/be-core.md#L1715-L1755; docs/business/fe-core.md#L313-L313`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/business/be-core.md#L1724-L1745`):** In §8.2 Step 2.1 (`#L1726-L1731`), when the user clicks Lock (`desired_locked == true`), the backend *first* cancels the active streaming generation in `WritingRuntimeRegistry` (`generation_cancel_token.cancel()`, setting `InvalidatedByLock` and `authorization_valid = false` permanently) and *then* in Step 2.2 (`#L1732-L1738`) opens `BEGIN IMMEDIATE` (`TX-06`) to execute `UPDATE scenes SET is_locked = ?1`.
  - **Claim B (`docs/business/core.md#L210-L235`, `docs/business/be-core.md#L502-L502`, `docs/business/be-core.md#L1740-L1741`, `docs/business/fe-core.md#L313-L313`):** Requires `TX-06` rollback on SQLite error so the scene remains unlocked and no partial state escapes (`LOCK-01`). If Step 2.1 cancels the stream and permanently invalidates the in-memory generation *before* Step 2.2 commits `TX-06`, a transient SQLite failure in `TX-06` leaves the scene unlocked in SQLite/UI while the active generation stream has already been irreversibly destroyed (`authorization_valid = false`).
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase4_sprint4_deep1`
- `REQUIRED_OWNER_DECISION:`
  - Specify whether `TX-06` (`UPDATE scenes SET is_locked = 1`) must commit in SQLite *before* firing `generation_cancel_token.cancel()` / marking `InvalidatedByLock` (while holding the chapter runtime guard so no concurrent Accept can interleave), or whether cancellation-before-DB-commit is intentional even when `TX-06` rolls back.

### CI-07 — `themes` schema/model mismatch (`themes.resolution_type` default `'OPEN'` vs `core.md` / `be-core.md` 46-attribute paths)

- `ISSUE_ID: CI-07`
- `SOURCE_ANCHOR: docs/db.dbml#L958-L997; docs/business/core.md#L425-L429; docs/business/be-core.md#L400-L413; docs/business/be-core.md#L692-L692`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/db.dbml#L958-L997` & `docs/business/be-core.md#L408-L408`):** `docs/db.dbml#L958-L965` defines `enum ThemeResolutionType { RESOLVED, OPEN, AMBIGUOUS }` and `Table themes` (`#L989-L999`) with two separate columns: `resolution_type ThemeResolutionType [default: 'OPEN']` (`#L997`) and `resolution text` (`#L998`), which `be-core.md#L408` requires translating into an executable SQLite `TEXT DEFAULT 'OPEN' CHECK (resolution_type IN ('RESOLVED', 'OPEN', 'AMBIGUOUS'))` constraint.
  - **Claim B (`docs/business/core.md#L425-L429`, `docs/business/be-core.md#L692-L692`, and `src/features/business-flow/types.ts#L78-L88` / `mockData.ts#L114-L118`):** `core.md#L427` and `be-core.md#L692` (46 Master Attributes) model `optional.overview_phase.arcs.themes` with a single composite attribute `themes.resolution` ("Phương thức giải quyết (loại và nội dung)") rather than separate `resolution_type` and `resolution` attributes, and the existing FE draft prototype (`src/features/business-flow/mockData.ts#L115`) populates `themes.resolution.type = 'action'` (a `ThemeAnswerMode` value rather than `ThemeResolutionType` `'RESOLVED' | 'OPEN' | 'AMBIGUOUS'`). Furthermore, `phase1_sprint1_deep0.md` §4/§10/§12/§16 flags `themes.resolution_type default OPEN not in enum` as an explicit contract verification item requiring owner disposition before freezing SQLite DDL in `phase2_sprint3_deep1`.
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase2_sprint3_deep1`
- `REQUIRED_OWNER_DECISION:`
  - Confirm the exact SQLite `CHECK` and `DEFAULT` definition for `themes.resolution_type` (`DEFAULT 'OPEN' CHECK (resolution_type IS NULL OR resolution_type IN ('RESOLVED', 'OPEN', 'AMBIGUOUS'))`) and confirm how `BusinessFlowData` `themes.resolution` maps to `(themes.resolution_type, themes.resolution)` during validation and `TX-05` commit.

### CI-08 — No authoritative scene-version restore IPC command in the 40-command inventory

- `ISSUE_ID: CI-08`
- `SOURCE_ANCHOR: docs/business/core.md#L291-L297; docs/business/be-core.md#L2110-L2158; docs/business/be-core.md#L2231-L2243; docs/business/fe-core.md#L294-L440`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/business/be-core.md#L2110-L2158`, `docs/business/be-core.md#L2231-L2243`):** Freezes the Backend IPC surface at **exactly 40 commands** (`REQ-IPC-002`). Group 9 (`Writing Mode`, commands `#29..#37`) includes `#35 list_scene_versions` (which returns `SceneVersionItemDto[]`) and `#36 save_chapter` (which accepts `DirtySceneSaveItem { scene_id, expected_version_number, content, scene_source: SceneSource::Restore, summary }`), but **no dedicated `restore_scene_version` IPC command**.
  - **Claim B (`docs/business/core.md#L291-L297`, `docs/business/fe-core.md#L304-L304`, `docs/business/fe-core.md#L345-L345`, `docs/business/fe-core.md#L535-L535`, and `REQ-WRITE-004`):** `core.md#L291-L297` states that selecting a historical scene version loads its `content` into the chapter working state in memory, marks the scene dirty if different from baseline (without persisting immediately), and creates a new `scene_versions` row with `scene_source = RESTORE` upon the next `save_chapter` (`TX-07`). However, `fe-core.md#L535` ("danh sách version scene và restore") and `REQ-WRITE-004` note that it is unconfirmed whether `SceneVersionItemDto` returned by `#35 list_scene_versions` carries full `content` so restore is purely client-side working-state replacement followed by `#36 save_chapter(scene_source = RESTORE)`, or whether a backend restore path is required (while forbidding invention of a 41st IPC command).
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase5_sprint5_deep3`
- `REQUIRED_OWNER_DECISION:`
  - Confirm that `#35 list_scene_versions` (`SceneVersionItemDto`) includes full `content` and that scene-version restore is performed by loading `SceneVersionItemDto.content` into the FE chapter dirty buffer and persisting via `#36 save_chapter` with `sceneSource = "RESTORE"` (requiring zero new IPC commands beyond the frozen 40).

### CI-09 — `system_language` request parameter vs trusted UI settings source precedence

- `ISSUE_ID: CI-09`
- `SOURCE_ANCHOR: docs/business/be-core.md#L990-L1002; docs/business/be-core.md#L2169-L2203; docs/business/fe-core.md#L78-L80; docs/business/fe-core.md#L488-L539`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/business/be-core.md#L996-L999`):** States that `system_language` (`"vi" | "en"`) is obtained from the UI settings boundary to control AI conversational responses and fixed null-field question lookup (`get_field_null` / `get_array_element_fields`), and the AI model never passes a language parameter to tools.
  - **Claim B (`docs/business/be-core.md#L2169-L2169`, `#L2177-L2177`, `#L2180-L2180`, `#L2202-L2202`, and `docs/business/fe-core.md#L78-L78`):** FE currently stores locale only in browser `localStorage` (`merlink.locale`, `fe-core.md#L78`), while backend IPC commands `#3 send_chat_message`, `#6 get_progress_detail`, `#9 regenerate_draft_phase`, and `#16 start_or_resume_workflow` each take an explicit per-request `systemLanguage: "vi" | "en"` field in their request DTO, and `#38/#39 get/update_runtime_and_provider_config` (`be-core.md#L2247-L2249`) do not include `systemLanguage`. The precedence between a backend-stored trusted UI setting vs per-request `systemLanguage` DTO parameter is unspecified.
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase5_sprint2_deep1`
- `REQUIRED_OWNER_DECISION:`
  - Clarify whether the `CallerContext::UiAction` per-request `systemLanguage: "vi" | "en"` field on IPC commands `#3`, `#6`, `#9`, and `#16` *is* the authoritative UI settings boundary passed into the backend for that turn/operation (and locked into the turn/workflow context so the LLM cannot override it), or whether `RuntimeAndProviderConfigDto` must also hold a backend-persisted `systemLanguage` setting.

### CI-10 — Provider configuration persistence semantics missing

- `ISSUE_ID: CI-10`
- `SOURCE_ANCHOR: docs/business/be-core.md#L2070-L2098; docs/business/be-core.md#L2152-L2155; docs/business/be-core.md#L2245-L2251; docs/business/fe-core.md#L32-L80`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/business/be-core.md#L2070-L2098`, `docs/business/be-core.md#L2152-L2155`, `docs/business/be-core.md#L2245-L2251`):** Defines API key persistence strictly via OS Keychain (`keyring` crate) with in-memory `SessionKeyStore` fallback (`#40 set_provider_api_key`), and defines `#38 get_runtime_and_provider_config` and `#39 update_runtime_and_provider_config` (`{ subagentConfig?, activeProviderId?, modelName?, endpointUrl? }`) with `Requires SQLite Transaction = Không` ("Cập nhật cấu hình runtime; không lưu API key vào SQLite").
  - **Claim B (`docs/db.dbml` and `docs/business/fe-core.md#L32-L80`):** Neither `docs/db.dbml` nor `docs/business/be-core.md` defines any SQLite table or config file path for non-secret runtime/provider settings (`subagentConfig`, `activeProviderId`, `modelName`, `endpointUrl`). Consequently, it is unspecified whether `update_runtime_and_provider_config` is strictly in-memory for the current process lifetime or persists non-secret configuration across restarts.
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase2_sprint4_deep3, phase5_sprint6_deep1`
- `REQUIRED_OWNER_DECISION:`
  - Confirm whether `#39 update_runtime_and_provider_config` is strictly runtime/in-memory for V1 (resetting non-secret `subagentConfig`/`activeProviderId`/`modelName`/`endpointUrl` to defaults on app restart while API keys remain in OS Keychain), or ratify an authoritative non-secret config persistence mechanism.

### CI-11 — SF-06 B+ target represented as unconditional physical schema vs spec-only authority

- `ISSUE_ID: CI-11`
- `SOURCE_ANCHOR: docs/architecture/decisions/sf06-owner-ratification.md#L3-L17; docs/architecture/decisions/sf06-owner-ratification.md#L56-L83; docs/db.dbml#L1299-L1378; docs/business/be-core.md#L385-L491; docs/business/be-core.md#L1508-L1578`
- `OLD_CONTRACT / CONFLICTING_CLAIMS:`
  - **Claim A (`docs/db.dbml#L1299-L1378`, `docs/business/be-core.md#L470-L491`, `docs/business/be-core.md#L1508-L1578`):** `docs/db.dbml` directly declares the 66-table SF-06 B+ physical schema (`workflow_round_receipts`, 3 new nullable columns on `workflow_task_receipts`, and `pending_patches.eligibility_json`), and `be-core.md` §3.5 (`#L470-L491`) requires startup `verify_schema_and_integrity` to verify all 66 target tables and B+ `CHECK`/composite FK constraints before opening the application.
  - **Claim B (`docs/architecture/decisions/sf06-owner-ratification.md#L3-L17`, `docs/architecture/decisions/sf06-owner-ratification.md#L25-L25`, `docs/architecture/decisions/sf06-owner-ratification.md#L56-L58`, `docs/business/be-core.md#L385-L386`):** Explicitly states `Status: SPECIFICATION_READY_WITH_CONDITIONS; IMPLEMENTATION_NOT_VERIFIED; MIGRATION_NOT_EXECUTED`, notes that B+ is an accepted specification direction ("requires forward-only migration before runtime" and "migration after separate authorization"), and warns in `be-core.md#L472-L473` that pre-B+ databases have 65 tables (`user_version` before B+ migration) and must not be falsely treated as already migrated.
- `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)`
- `OWNER_APPROVAL_SHA: NONE (current authority SHA 57a462efb953a665e3673b15e6cc33622a4f2d39)`
- `AFFECTED_SPRINTS: phase2_sprint3_deep1, phase3_sprint9_deep2`
- `REQUIRED_OWNER_DECISION:`
  - Authorize the execution of forward-only B+ schema migration for V1 runtime (`phase2_sprint3_deep1`) and confirm how `verify_schema_and_integrity` and `receipt_verifier` (`phase3_sprint9_deep2`) handle pre-B+ 65-table databases vs migrated 66-table B+ databases (including legacy rows where `round_receipt_id`, `task_index`, `evidence_json`, and `eligibility_json` are `NULL`).

---

## 4. Cross-Walk of All 11 Sprint Contradiction Topics to `CI-01` .. `CI-11`

Every contradiction topic enumerated in `phase1_sprint1_deep0.md` §4 & §10 is accounted for in the 11 `CI-xx` entries above:

| # | Sprint Prompt Contradiction Topic (`phase1_sprint1_deep0.md` §4 / §10 / §16) | Mapped Issue ID(s) | Primary SSOT Anchors |
| :---: | :--- | :--- | :--- |
| 1 | `seal_pending_retry` (`be-core.md` §4.4.3 vs `db.dbml` `DraftSessionStatus`) | `CI-03` (3a) | `docs/business/be-core.md#L660-L662` vs `docs/db.dbml#L1230-L1234` |
| 2 | `TX01 pending expiry` (`be-core.md` §3.6 TX-01 table vs §4.4.2 pseudocode & TX-08) | `CI-03` (3b) | `docs/business/be-core.md#L497-L497` vs `docs/business/be-core.md#L589-L647` |
| 3 | `restore dedupe/nullable phase` (TX-01 Step 3 hash dedupe vs TX-03 append-only; nullable `phase` CHECK) | `CI-03` (3c, 3d) | `docs/business/be-core.md#L616-L625`, `docs/business/be-core.md#L664-L679`, `docs/db.dbml#L1278-L1297` |
| 4 | `ErrorCode` (22 enum variants in §1.5 vs SF-06/patch/artifact/workflow error variants in §6.4.7/§10.2) | `CI-04` (4a) | `docs/business/be-core.md#L188-L212` vs `docs/business/be-core.md#L1530-L1578` |
| 5 | `TX05 counts/order` (`core.md` `scenes -> lore` vs `be-core.md` 21-table claim / 22 steps / 24 tables `lore_entries -> scenes`) | `CI-05` | `docs/business/core.md#L740-L740` vs `docs/business/be-core.md#L501-L501`, `docs/business/be-core.md#L1625-L1651` |
| 6 | `TX06 failure ordering` (`be-core.md` §8.2 cancel-stream-before-SQLite-update on lock failure) | `CI-06` | `docs/business/be-core.md#L1724-L1745` vs `docs/business/core.md#L210-L235` |
| 7 | `theme OPEN` (`db.dbml` `themes.resolution_type [default: 'OPEN']` vs `core.md`/`be-core.md` 46-attribute `themes.resolution`) | `CI-07` | `docs/db.dbml#L958-L997` vs `docs/business/core.md#L425-L429`, `docs/business/be-core.md#L692-L692` |
| 8 | `scene-version restore IPC` (40 frozen commands have `list_scene_versions` + `save_chapter`, no 41st restore IPC) | `CI-08` | `docs/business/be-core.md#L2110-L2158`, `docs/business/be-core.md#L2231-L2243` vs `docs/business/fe-core.md#L294-L440` |
| 9 | `extra durable commands outside TX inventory` (IPC `#1`, `#3`, `#12`, `#21`, `#24`, `#27` `BEGIN IMMEDIATE` writes vs §3.6 8-TX rule) | `CI-04` (4b) | `docs/business/be-core.md#L493-L506` vs `docs/business/be-core.md#L2118-L2244` |
| 10 | `system_language source` (`be-core.md` §5.2 UI setting boundary vs per-request `systemLanguage` DTO field on `#3`, `#6`, `#9`, `#16`) | `CI-09` | `docs/business/be-core.md#L996-L999` vs `docs/business/be-core.md#L2169-L2203` |
| 11 | `provider configuration persistence` (`#39 update_runtime_and_provider_config` non-secret persistence unspecified) | `CI-10` | `docs/business/be-core.md#L2070-L2098`, `docs/business/be-core.md#L2245-L2251` vs `docs/business/fe-core.md#L32-L80` |
| 12 | `Migration baseline 0001/0002 absent; version path unapproved` | `CI-01` | `docs/business/be-core.md#L415-L491`, `docs/business/be-core.md#L2621-L2628` |
| 13 | `Phase-3 merger/consumer exact projection unproven` | `CI-02` | `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83`, `docs/business/be-core.md#L1549-L1549` |
| 14 | `B+ target represented as unconditional vs spec-only authority` | `CI-11` | `docs/architecture/decisions/sf06-owner-ratification.md#L3-L17`, `docs/db.dbml#L1299-L1378` |

---

## 5. Dependency-Specific Unblock and Owner-Escalation Map

### 5.1 Sprints Directly Blocked by Open Contract Issues (`CI-01` .. `CI-11`)

| Sprint ID | Blocking Issue IDs | Additional Contract Review Gates | Unblock Condition |
| :--- | :--- | :--- | :--- |
| `phase1_sprint2_deep1` | `CI-01` | `OWNER_MIGRATION_PROVENANCE`, `CONTRACT_REVIEW_REQUIRED:REQ-CONTRACT-002` | May run as a contract-inventory sprint after `gate_phase1_sprint1_deep0` (`INVENTORY_ACCEPTED`) to document the missing SQL baseline and propose migration paths; cannot clear `CI-01` for `phase2_sprint3_deep1` without owner-approved SSOT SHA. |
| `phase1_sprint3_deep0` | `CI-02` | `PHASE3_CONSUMER_SHAPE`, `CONTRACT_REVIEW_REQUIRED:REQ-CONTRACT-003` | May run as a contract-inventory sprint to document the unproven Phase-3 consumer projection; cannot clear `CI-02` for `phase3_sprint3_deep0` without owner-approved fixture/SSOT SHA. |
| `phase2_sprint3_deep1` | `CI-01`, `CI-07`, `CI-11` | `OWNER_MIGRATION_PROVENANCE`, `CONTRACT_REVIEW_REQUIRED:REQ-DB-001..004` | Blocked on SQLite implementation until owner ratifies `CI-01` (migration version path), `CI-07` (`themes.resolution_type`), and `CI-11` (B+ DDL execution authorization). |
| `phase2_sprint4_deep3` | `CI-04`, `CI-10` | — | Blocked until owner ratifies `CI-04` (`ErrorCode` surface) and `CI-10` (`update_runtime_and_provider_config` persistence semantics). |
| `phase2_sprint5_deep2` | `CI-04` | `ERROR_ENUM_AUTHORITY`, `CONTRACT_REVIEW_REQUIRED:REQ-IPC-001` | Blocked until owner ratifies `CI-04` (`ErrorCode` variants and 40-command error/TX mapping). |
| `phase3_sprint2_deep0` | `CI-03` | `SEAL_RESTORE_ERROR_CONTRACT`, `CONTRACT_REVIEW_REQUIRED:REQ-DRAFT-001` | Blocked until owner ratifies `CI-03` (`seal_pending_retry`, `TX-01` pending expiry, `TX-03` restore idempotency, nullable `phase`). |
| `phase3_sprint3_deep0` | `CI-02` | `PHASE3_CONSUMER_SHAPE`, `CONTRACT_REVIEW_REQUIRED:REQ-ART-001` | Blocked until owner ratifies `CI-02` (`WorldProbeReportV1` / `WorldWriteInputV1` consumer projection and budget). |
| `phase3_sprint9_deep2` | `CI-02`, `CI-11` | `CONTRACT_REVIEW_REQUIRED:REQ-SF06-001` | Blocked until owner ratifies `CI-02` and `CI-11`. |
| `phase3_sprint10_deep4` | `CI-02` | — | Blocked until owner ratifies `CI-02`. |
| `phase4_sprint1_deep0` | `CI-02` | `CONTRACT_REVIEW_REQUIRED:REQ-WF-003` | Blocked until owner ratifies `CI-02`. |
| `phase4_sprint2_deep0` | `CI-05` | `TX05_MAPPING_AUTHORITY`, `CONTRACT_REVIEW_REQUIRED:REQ-COMMIT-002` | Blocked until owner ratifies `CI-05` (`TX-05` table count and `scenes` vs `lore_entries` order). |
| `phase4_sprint4_deep1` | `CI-06` | `TX06_FAILURE_ORDER`, `CONTRACT_REVIEW_REQUIRED:REQ-WRITE-003` | Blocked until owner ratifies `CI-06` (`TX-06` DB-commit vs stream-cancel failure ordering). |
| `phase5_sprint2_deep1` | `CI-09` | — | Blocked until owner ratifies `CI-09` (`system_language` precedence). |
| `phase5_sprint5_deep3` | `CI-08` | `CONTRACT_REVIEW_REQUIRED:REQ-FE-005`, `CONTRACT_REVIEW_REQUIRED:REQ-WRITE-004` | Blocked until owner ratifies `CI-08` (scene-version restore via `list_scene_versions` + `save_chapter`). |
| `phase5_sprint6_deep1` | `CI-10` | — | Blocked until owner ratifies `CI-10` (provider config persistence boundary). |

### 5.2 Sprints Unblocked by `CI-01` .. `CI-11` (Zero Direct `CI-xx` Blockers)

The following sprints have no direct `CI-01`..`CI-11` blocker in `manifest.json` (subject to their normal DAG prerequisite gates):
- **Phase 2:** `phase2_sprint1_deep0` (Bootstrap Tauri workspace and module shells), `phase2_sprint2_deep1` (Freeze pure domain types and canonical state, after `phase2_sprint1_deep0`), `phase2_sprint6_deep2` (after `phase2_sprint2_deep1` + `phase2_sprint3_deep1`).
- **Phase 3:** `phase3_sprint1_deep0` (Planner, progress, and cross-entity validation, after `phase2_sprint2_deep1`).

---

## 6. Acceptance Test & Failure Matrix Verification Record

| Test / Case ID | Scenario | Observed Verification Result at `57a462efb953a665e3673b15e6cc33622a4f2d39` | Status |
| :--- | :--- | :--- | :---: |
| **`P1S1-P`** | Given two conflicting source ranges, ledger lists old/new (conflicting) contract claims, owner authority requirement, and accepted revision SHA (`57a462efb953a665e3673b15e6cc33622a4f2d39`); zero SSOT edits. | All 11 issues (`CI-01`..`CI-11`) and 14 contradiction topics record exact file/line ranges on both sides of each conflict; `git diff --name-only 57a462efb953a665e3673b15e6cc33622a4f2d39` confirms zero edits to `docs/business/**`, `docs/db.dbml`, or `docs/architecture/decisions/**`. | `PASS` |
| **`P1S1-N`** | Unresolved `theme OPEN` (`CI-07`) or `seal_pending_retry` (`CI-03`) yields `CONTRACT_BLOCKED` and prevents only affected successors (`phase2_sprint3_deep1`, `phase3_sprint2_deep0`), not unrelated sprints (`phase2_sprint1_deep0`, `phase2_sprint2_deep1`, `phase3_sprint1_deep0`). | `CI-03` and `CI-07` are explicitly marked `DISPOSITION: RATIFIED_ACCEPTED (awaiting owner authority revision)` with scoped `AFFECTED_SPRINTS` matching `docs/plan/manifest.json`. | `PASS` |
| **`P1S1-B`** | Clean baseline unchanged; evidence file can be validated idempotently from the same SHA (`57a462efb953a665e3673b15e6cc33622a4f2d39`). | `python3 docs/plan/validate_plan.py --contract-evidence docs/plan/evidence/contract-ledger.md` and `git diff --check` succeed deterministically with exit code `0`. | `PASS` |
| **`phase1_sprint1_deep0-F1`** | Missing owner disposition trigger. | Classified as `CONTRACT_BLOCKED` under `CONTRACT_STATUS: INVENTORY_ACCEPTED`; zero implementation writes performed; resumption requires an owner-ratified SSOT SHA. | `PASS` |
| **`phase1_sprint1_deep0-F2`** | Conflicting SSOT revision trigger. | Both claims preserved verbatim with exact line anchors and authority hierarchy noted (`sf06-owner-ratification.md` > `core.md` > `db.dbml` > `be-core.md`/`fe-core.md`); no silent precedence invented. | `PASS` |
| **`phase1_sprint1_deep0-F0`** | Predecessor gate / baseline SHA check. | `depends_on: []`; source baseline `57a462efb953a665e3673b15e6cc33622a4f2d39` is ancestor of worktree HEAD `407469219679d2fa40f3aaf634d6e802741d908a` with zero non-plan modifications. | `PASS` |
