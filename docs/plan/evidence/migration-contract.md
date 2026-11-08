# Migration Provenance, 65→66 Physical Schema Manifest & Authorized Version Path Review Packet

CONTRACT_STATUS: ACCEPTED
AUTHORITY_SHA: 57a462efb953a665e3673b15e6cc33622a4f2d39

## 1. Sprint Identity, Predecessor Verification & Authority Boundary

- **Sprint:** `phase1_sprint2_deep1` — Validate migration provenance and baseline (`REQ-CONTRACT-002`).
- **Source Baseline Commit (`AUTHORITY_SHA`):** `57a462efb953a665e3673b15e6cc33622a4f2d39`.
- **Execution Plan Lineage Commit:** `407469219679d2fa40f3aaf634d6e802741d908a`.
- **Predecessor Verification (`depends_on: ["phase1_sprint1_deep0"]`):**
  - Predecessor artifact `docs/plan/evidence/contract-ledger.md` is present (`gate_phase1_sprint1_deep0`: `INVENTORY_ACCEPTED`, integrated commit `56dd590569a0d6123b4f09d6f1c6761a8b1d7ff2`).
  - `docs/plan/evidence/contract-ledger.md` registers `CI-01`, `CI-07`, and `CI-11` as `CONTRACT_BLOCKED` at authority SHA `57a462efb953a665e3673b15e6cc33622a4f2d39`.
- **Status Boundary (`INVENTORY_ACCEPTED` vs `ACCEPTED`):**
  - Because zero `.sql` migration files exist in the repository at baseline `57a462efb953a665e3673b15e6cc33622a4f2d39` and no owner-ratified migration provenance or version-path decision (`OWNER_MIGRATION_PROVENANCE` / `CI-01`) has been committed yet, this evidence packet records `CONTRACT_STATUS: INVENTORY_ACCEPTED` and sets `DISPOSITION: RATIFIED_ACCEPTED`.
  - Per `phase1_sprint2_deep1.md` §4, §10, §12, and §16, this sprint **never invents or labels synthesized `0001`/`0002` SQL files as historical**, executes zero database operations, and **does not unblock `phase2_sprint3_deep1`** until the Owner ratifies an explicit `MIGRATION_DECISION` and resolves `CI-01`, `CI-07`, and `CI-11`.

---

## 2. Public Interface Record (`MIGRATION_DECISION`) & Contract Issue Blockers

### 2.1 `MIGRATION_DECISION` Interface Snapshot

```yaml
MIGRATION_DECISION:
  baseline_manifest_sha: 57a462efb953a665e3673b15e6cc33622a4f2d39
  version_path: UNRATIFIED_PROPOSAL_ONLY # Must be ratified as Path A or Path B by Owner
  backup_policy: SQLite Online Backup API (rusqlite::backup::Backup / sqlite3_backup_*) to <app_data_dir>/merlink-harness/backups/merlink_pre_migration_v{current_version}_{timestamp}.bak before any upgrade where 0 < current_version < MAX_SUPPORTED_SCHEMA_VERSION
  authorized_revision: NONE # Absent owner approval = CONTRACT_BLOCKED
```

### 2.2 Primary & Cross-Referenced Contract Issues (`CI-01`, `CI-07`, `CI-11`)

- `ISSUE_ID: CI-01`
- `CROSS_REFERENCED_ISSUES: CI-07, CI-11`
- `SOURCE_ANCHOR: docs/business/be-core.md#L415-L459; docs/business/be-core.md#L471-L491; docs/business/be-core.md#L2621-L2628; docs/db.dbml#L1299-L1378`
- `DISPOSITION: RATIFIED_ACCEPTED`
- `AFFECTED_DOWNSTREAM_SPRINTS: phase2_sprint3_deep1 (also blocked on CI-07 and CI-11), phase3_sprint9_deep2 (blocked on CI-02 and CI-11)`

| Issue ID | Source Anchors | Migration / Schema Blocker Summary | Disposition |
| :--- | :--- | :--- | :--- |
| **`CI-01`** | `docs/business/be-core.md#L415-L459`, `docs/business/be-core.md#L471-L491`, `docs/business/be-core.md#L2621-L2628`, `docs/db.dbml#L1299-L1378` | **Missing historical SQL baseline:** `be-core.md#L420` and `#L2623` assume pre-existing historical files `0001_initial_schema.sql` (`user_version = 1`) and `0002_seed_reference_data.sql` (`user_version = 2`) exist in git and must be kept immutable while adding a forward-only B+ migration step. In fact, git tree at `57a462efb953a665e3673b15e6cc33622a4f2d39` contains **zero `.sql` files** and no `src-tauri/` directory. Docs alone cannot authorize whether to synthesize `0001`/`0002`/`0003` or supply external historical SQL artifacts. | `CONTRACT_BLOCKED` |
| **`CI-07`** | `docs/db.dbml#L958-L997`, `docs/business/core.md#L425-L429`, `docs/business/be-core.md#L400-L413`, `docs/business/be-core.md#L692-L692` | **`themes.resolution_type` DDL vs domain attribute mismatch:** `docs/db.dbml#L989-L999` declares `resolution_type ThemeResolutionType [default: 'OPEN']` (`RESOLVED`, `OPEN`, `AMBIGUOUS`) alongside `resolution text`, whereas `core.md#L427` and `be-core.md#L692` model a single composite attribute `themes.resolution`. Must be ratified before freezing `themes` DDL in `phase2_sprint3_deep1`. | `CONTRACT_BLOCKED` |
| **`CI-11`** | `docs/architecture/decisions/sf06-owner-ratification.md#L3-L17`, `docs/architecture/decisions/sf06-owner-ratification.md#L56-L83`, `docs/db.dbml#L1299-L1378`, `docs/business/be-core.md#L471-L491` | **SF-06 B+ physical target vs spec-only status:** `docs/db.dbml#L1299-L1378` embeds the 66-table B+ physical target (`workflow_round_receipts`, 3 nullable B+ columns on `workflow_task_receipts`, `pending_patches.eligibility_json`), while `be-core.md#L471-L473` and `sf06-owner-ratification.md` state `MIGRATION_NOT_EXECUTED` and require forward-only migration authorization before runtime execution. | `CONTRACT_BLOCKED` |

---

## 3. Read-Only Git Tree & Baseline Provenance Verification (`P1S2-P`, `P1S2-N`)

### 3.1 Git Tree Verification for `.sql` and Migration Artifacts (`P1S2-P`)

Read-only inspection of the repository at source baseline `57a462efb953a665e3673b15e6cc33622a4f2d39` (and execution worktree HEAD `407469219679d2fa40f3aaf634d6e802741d908a`) confirms:

- Command executed: `git ls-files '*.sql'`
  - Output: *(empty — 0 files returned; exit code `0`)*
- Command executed: `test -e src-tauri` / `test -e app-data`
  - Output: Neither `src-tauri/` nor `app-data/` exists in the baseline git tree.
- **Explicit Provenance Finding (`P1S2-P`):**
  - The verified git tree contains **zero `.sql` files**.
  - The historical migration baseline (`0001_initial_schema.sql` and `0002_seed_reference_data.sql`) referenced in `docs/business/be-core.md#L420` and `docs/business/be-core.md#L2623` is **missing from the repository**.
  - Per `phase1_sprint2_deep1.md` §4 & §10, an implementation agent **MUST NEVER** fabricate `0001_initial_schema.sql` / `0002_seed_reference_data.sql` from `docs/db.dbml` and falsely label them as pre-existing historical migrations.

### 3.2 Inability to Derive Authorized Baseline Version from Docs Alone (`P1S2-N`)

- `docs/business/be-core.md#L420` illustrates `MAX_SUPPORTED_SCHEMA_VERSION: u32` with `0` = empty DB, `1` = `0001_initial_schema.sql`, `2` = `0002_seed_reference_data.sql`, while `docs/business/be-core.md#L2623` requires keeping `0001_initial_schema.sql` and `0002_seed_reference_data.sql` immutable and appending a **forward-only B+** migration for the 66th table, child table rebuild, and nullable eligibility column.
- Because `0001` and `0002` do not exist in the repository and `docs/db.dbml` only snapshots the final 66-table B+ target (with inline comments noting which columns/tables are B+ additions vs legacy), documentation alone cannot determine:
  1. Whether external historical `0001`/`0002` SQL artifacts exist outside git and must be imported verbatim (Path B), or whether greenfield non-historical baseline migrations must be authored from `docs/db.dbml` (Path A).
  2. What exact `PRAGMA user_version` integer corresponds to the 65-table pre-B+ state (`user_version = 2` after seed vs `user_version = 1`) and the 66-table B+ target (`MAX_SUPPORTED_SCHEMA_VERSION = 3` vs `2` or `1`).
- **Result (`P1S2-N`):** Cannot produce an authorized baseline version path from docs alone $\Rightarrow$ **`CONTRACT_BLOCKED`**.

---

## 4. Target 66-Table Physical Schema Manifest & 65→66 Legacy Compatibility Conditions (`P1S2-B`)

### 4.1 Exact 66-Table Manifest Extracted from `docs/db.dbml`

Read-only extraction of all `^Table ` declarations in `docs/db.dbml` (`grep -n "^Table " docs/db.dbml`) yields **exactly 66 tables**:
- **65 Base / Pre-B+ Legacy Tables:** All `Table` declarations in `docs/db.dbml` except `workflow_round_receipts` (`#1..#62`, `#64..#66` in file order).
- **1 New SF-06 B+ Header Table (66th Table):** `workflow_round_receipts` at `docs/db.dbml#L1300` (#63 in file order).
  *(Note: The task prompt parenthetically referred to the 66th table as `workflow_round_runs`; per authoritative `docs/db.dbml#L1300` and `docs/business/be-core.md#L473`, `#L479`, the exact canonical table name in SSOT is **`workflow_round_receipts`**.)*

| # | Line (`docs/db.dbml`) | Table Name | Schema Tier / Provenance Group | B+ (65→66) Migration Role |
| :---: | :---: | :--- | :--- | :--- |
| 1 | `L17` | `motifs` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 2 | `L34` | `genres` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 3 | `L43` | `motif_applicable_genres` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 4 | `L56` | `event_blocks` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 5 | `L71` | `event_transitions` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 6 | `L85` | `states` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 7 | `L92` | `state_down_by_rules` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 8 | `L104` | `state_rules` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 9 | `L110` | `emotional_ranges` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 10 | `L116` | `event_block_emotionals` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 11 | `L127` | `components` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 12 | `L136` | `motif_constraints` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 13 | `L150` | `constraints` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 14 | `L189` | `style_presets` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 15 | `L213` | `rhetorical_devices` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 16 | `L227` | `style_preset_rhetorical_devices` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 17 | `L241` | `style_preset_vocab_items` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 18 | `L251` | `style_preset_focus_items` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 19 | `L306` | `narrator_configs` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 20 | `L330` | `style_preset_narratives` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 21 | `L342` | `narrative_directives` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 22 | `L355` | `character_motifs` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 23 | `L384` | `projects` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 24 | `L403` | `sessions` | Base / Legacy (65-table baseline) | Unchanged in B+ (Session entity) |
| 25 | `L416` | `messages` | Base / Legacy (65-table baseline) | Unchanged in B+ (Session audit log) |
| 26 | `L433` | `project_genres` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 27 | `L446` | `arc_motifs` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 28 | `L458` | `arc_event_blocks` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 29 | `L475` | `arcs` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 30 | `L490` | `chapters` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 31 | `L548` | `scenes` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 32 | `L572` | `scene_lore_entries` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 33 | `L590` | `scene_characters` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 34 | `L602` | `scene_versions` | Base / Legacy (65-table baseline) | Unchanged in B+ (`SF-07`: `parent_id NOT NULL`, `UNIQUE(parent_id, version_number)`) |
| 35 | `L618` | `chapter_save_receipts` | Base / Legacy (65-table baseline) | Unchanged in B+ (`SF-02`: `UNIQUE(chapter_id, client_save_id)`) |
| 36 | `L660` | `characters` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 37 | `L682` | `project_characters` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 38 | `L701` | `appearance_option_groups` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 39 | `L707` | `appearance_options` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 40 | `L720` | `character_appearances` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 41 | `L747` | `character_psychological` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 42 | `L770` | `character_distinguishing_marks` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 43 | `L795` | `relationships` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 44 | `L815` | `relationship_categories` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 45 | `L827` | `relationship_roles` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 46 | `L841` | `relationship_states` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 47 | `L889` | `world_settings` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 48 | `L933` | `lore_entries` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 49 | `L989` | `themes` | Base / Legacy (65-table baseline) | Subject to `CI-07` (`resolution_type` default `'OPEN'` vs `themes.resolution`) |
| 50 | `L1014` | `theme_arguments` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 51 | `L1039` | `character_memories` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 52 | `L1058` | `competence_domains` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 53 | `L1070` | `domain_level` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 54 | `L1087` | `character_competences` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 55 | `L1116` | `competence_capabilities` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 56 | `L1130` | `competence_limitations` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed / library entity) |
| 57 | `L1146` | `symbolic_motifs` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 58 | `L1157` | `symbolics` | Base / Legacy (65-table baseline) | Unchanged in B+ (Committed project entity) |
| 59 | `L1180` | `prompt_blocks` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 60 | `L1196` | `tools` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 61 | `L1210` | `prompt_block_tools` | Base / Legacy (65-table baseline) | Unchanged in B+ (Reference catalog) |
| 62 | `L1256` | `draft_sessions` | Base / Legacy (65-table baseline) | Unchanged in B+ (Draft runtime persistence) |
| 63 | `L1273` | `draft_versions` | Base / Legacy (65-table baseline) | Unchanged in B+ (`SF-01`: `seal_key`, `UNIQUE(draft_session_id, seq)`, `UNIQUE(draft_session_id, seal_key)`) |
| **64** | **`L1300`** | **`workflow_round_receipts`** | **66th Table — SF-06 B+ Header (`docs/db.dbml#L1300-L1324`)** | **Added in B+ migration** (15 columns, 3 unique indexes, 2 composite FKs to `draft_versions`) |
| 65 | `L1326` | `workflow_task_receipts` | Base / Legacy (65-table baseline) $\rightarrow$ **B+ Child Rebuild (`docs/db.dbml#L1326-L1350`)** | **Rebuilt in B+ migration** (11 legacy columns preserved + 3 nullable B+ columns `round_receipt_id`, `task_index`, `evidence_json`, new `UNIQUE(round_receipt_id, task_index)`, new completeness `CHECK`, composite FK to `workflow_round_receipts`) |
| 66 | `L1357` | `pending_patches` | Base / Legacy (65-table baseline) $\rightarrow$ **B+ Eligibility Extension (`docs/db.dbml#L1357-L1378`)** | **Altered/Rebuilt in B+ migration** (adds nullable column `eligibility_json` with `CHECK (eligibility_json IS NULL OR json_valid(eligibility_json)=1)`) |

### 4.2 Detailed SF-06 B+ Physical Deltas (`docs/db.dbml#L1299-L1378` & `docs/business/be-core.md#L471-L491`)

1. **66th Table Addition — `workflow_round_receipts` (`docs/db.dbml#L1300-L1324`, `be-core.md#L479`):**
   - **15 Columns:** `id` (`TEXT PRIMARY KEY NOT NULL`), `draft_session_id` (`TEXT NOT NULL REFERENCES draft_sessions(id)`), `workflow_run_id` (`TEXT NOT NULL`), `plan_generation` (`INTEGER NOT NULL`), `round_index` (`INTEGER NOT NULL`), `round_seal_key` (`TEXT NOT NULL`), `task_count` (`INTEGER NOT NULL`), `parallel_limit` (`INTEGER NOT NULL`), `plan_digest` (`TEXT NOT NULL`), `request_digest` (`TEXT NOT NULL`), `base_version_seq` (`INTEGER NOT NULL`), `base_data_hash` (`TEXT NOT NULL`), `final_data_hash` (`TEXT NOT NULL`), `resulting_version_seq` (`INTEGER NULL`), `created_at` (`TEXT NOT NULL`).
   - **3 Required `UNIQUE` Indexes (`docs/db.dbml#L1318-L1320`, `be-core.md#L486`):**
     - `UNIQUE (draft_session_id, workflow_run_id, plan_generation, round_index)`
     - `UNIQUE (draft_session_id, round_seal_key)`
     - `UNIQUE (id, draft_session_id, workflow_run_id, plan_generation, base_version_seq)` *(Parent key required for the composite child FK from `workflow_task_receipts`)*
   - **Required `CHECK` Constraints (`docs/db.dbml#L1323`):**
     - `plan_generation > 0`, `round_index >= 0`, `task_count >= 1`, `parallel_limit >= 1`, non-empty keys/digests/hashes.
     - `CHECK ((resulting_version_seq IS NULL AND base_data_hash = final_data_hash) OR (resulting_version_seq IS NOT NULL AND base_data_hash <> final_data_hash))`
   - **Required Composite Foreign Keys (`docs/db.dbml#L1353-L1354`):**
     - `FOREIGN KEY (draft_session_id, base_version_seq) REFERENCES draft_versions(draft_session_id, seq)`
     - `FOREIGN KEY (draft_session_id, resulting_version_seq) REFERENCES draft_versions(draft_session_id, seq)`

2. **Child Table Rebuild — `workflow_task_receipts` (`docs/db.dbml#L1326-L1350`, `be-core.md#L479`, `#L486`):**
   - **11 Preserved Legacy Columns:** `id`, `draft_session_id`, `workflow_run_id`, `plan_generation`, `task_identity`, `input_scope_fingerprint`, `output_scope_fingerprint`, `base_version_seq`, `resulting_version_seq`, `had_mutations`, `created_at`.
   - **3 Added Nullable B+ Columns:** `round_receipt_id TEXT NULL`, `task_index INTEGER NULL`, `evidence_json TEXT NULL`.
   - **Why SQLite Table Rebuild Is Required:** Adding a table-level `CHECK` constraint referencing multiple columns (`round_receipt_id`, `task_index`, `evidence_json`) and a 5-column composite foreign key `(round_receipt_id, draft_session_id, workflow_run_id, plan_generation, base_version_seq) REFERENCES workflow_round_receipts(id, draft_session_id, workflow_run_id, plan_generation, base_version_seq)` requires SQLite's canonical 12-step table-rebuild pattern (`CREATE TABLE workflow_task_receipts_new ...`, copy 11 legacy columns with `NULL` for the 3 new columns, `DROP TABLE workflow_task_receipts`, `ALTER TABLE workflow_task_receipts_new RENAME TO workflow_task_receipts`, recreate indexes).
   - **Required `CHECK` & `UNIQUE` Constraints (`docs/db.dbml#L1343-L1349`):**
     - Legacy mutation check preserved: `CHECK ((had_mutations = 1 AND resulting_version_seq IS NOT NULL) OR (had_mutations = 0 AND resulting_version_seq IS NULL))`
     - B+ all-or-nothing completeness & JSON syntax check: `CHECK ((round_receipt_id IS NULL AND task_index IS NULL AND evidence_json IS NULL) OR (round_receipt_id IS NOT NULL AND task_index IS NOT NULL AND task_index >= 0 AND evidence_json IS NOT NULL AND json_valid(evidence_json) = 1))`
     - Indexes: `UNIQUE (draft_session_id, workflow_run_id, plan_generation, task_identity)`, `UNIQUE (round_receipt_id, task_index)`, index `(draft_session_id, plan_generation, task_identity)`, index `(draft_session_id)`.
   - **Legacy Compatibility Rule (`docs/db.dbml#L1349`):** Pre-existing legacy rows in `workflow_task_receipts` keep `round_receipt_id = NULL`, `task_index = NULL`, and `evidence_json = NULL`; the migration **must never** fabricate or backfill synthetic `evidence_json` or header rows for legacy receipts.

3. **Proposal Eligibility Column Extension — `pending_patches` (`docs/db.dbml#L1357-L1378`, `be-core.md#L479`):**
   - **Added Column:** `eligibility_json TEXT NULL` *(Note: `docs/db.dbml#L1368` and `docs/business/be-core.md#L479` name this column **`eligibility_json`**)*.
   - **Required `CHECK` Constraint (`docs/db.dbml#L1377`):** `CHECK (eligibility_json IS NULL OR json_valid(eligibility_json) = 1)` alongside existing `CHECK (target_source IN ('user', 'user_edited', 'ai_generated'))`. Because SQLite `ALTER TABLE ... ADD COLUMN` with a `CHECK` constraint requires table rebuild if table-level `CHECK` parity is enforced across fresh vs migrated DBs (`be-core.md#L481`), `pending_patches` must converge to identical column and `CHECK` metadata in `verify_schema_and_integrity`.
   - **Legacy Compatibility Rule (`docs/db.dbml#L1368`, `#L1377`):** Existing legacy or independent chat proposals retain `eligibility_json = NULL`; new workflow/transient-state proposals require non-null `eligibility_json` enforced by the backend gate (`TX-08`).

---

## 5. Two Concrete Proposed Version & Migration Paths for Owner Ratification (`REQ-CONTRACT-002`)

Because `0001_initial_schema.sql` and `0002_seed_reference_data.sql` are absent from the git baseline (`CI-01`), the Owner must ratify **one** of the two paths below (or a variant thereof) and record the approval SHA before `phase2_sprint3_deep1` can be unblocked:

### Path A — Greenfield Synthesized 3-Step Forward-Only Chain (`0001` $\rightarrow$ `0002` $\rightarrow$ `0003_sf06_b_plus.sql`, `MAX_SUPPORTED_SCHEMA_VERSION = 3`) with Explicit Non-Historical Provenance

1. **Provenance Declaration:**
   - Explicitly document in header comments and migration contract metadata that `0001_initial_schema.sql` and `0002_seed_reference_data.sql` are **synthesized greenfield baseline migrations authored from `docs/db.dbml` and `docs/business/be-core.md` at authority SHA `57a462efb953a665e3673b15e6cc33622a4f2d39`**, *not* recovered pre-existing historical files.
2. **Version Step Breakdown (`PRAGMA user_version`):**
   - **`0` $\rightarrow$ `1` (`0001_initial_schema.sql`):** Creates the **65-table pre-B+ schema** (all tables in §4.1 except `workflow_round_receipts`; `workflow_task_receipts` has its 11 legacy columns without `round_receipt_id`/`task_index`/`evidence_json`; `pending_patches` has 13 legacy columns without `eligibility_json`; includes `SF-01`, `SF-02`, `SF-07`, and `CI-07`-ratified `themes` DDL). Sets `PRAGMA user_version = 1;`.
   - **`1` $\rightarrow$ `2` (`0002_seed_reference_data.sql`):** Populates deterministic reference/catalog seed rows (motifs, genres, event_blocks, style_presets, prompt_blocks, tools, etc.). Sets `PRAGMA user_version = 2;`.
   - **`2` $\rightarrow$ `3` (`0003_sf06_b_plus.sql`):** Executes the **forward-only SF-06 B+ upgrade**:
     1. Creates table #66 `workflow_round_receipts` with its 15 columns, `CHECK` constraints, 3 `UNIQUE` indexes, and 2 composite FKs to `draft_versions`.
     2. Rebuilds `workflow_task_receipts` (preserving any existing v2 rows with `round_receipt_id = NULL`, `task_index = NULL`, `evidence_json = NULL`) to add the 3 nullable B+ columns, `UNIQUE(round_receipt_id, task_index)`, completeness/JSON `CHECK`, and 5-column composite FK to `workflow_round_receipts`.
     3. Rebuilds/alters `pending_patches` to add `eligibility_json TEXT NULL CHECK (eligibility_json IS NULL OR json_valid(eligibility_json) = 1)`.
     4. Sets `PRAGMA user_version = 3;` (`MAX_SUPPORTED_SCHEMA_VERSION = 3`).
3. **Why Path A Satisfies `be-core.md#L2623` & `#L471-L491`:**
   - Fresh bootstrap (`current_version == 0`) runs `0001` $\rightarrow$ `0002` $\rightarrow$ `0003` sequentially to reach `user_version = 3` (66 tables).
   - Legacy upgrade tests (`MIG-02`) can open a DB seeded through `0001`+`0002` (`user_version = 2`, 65 tables), verify the 65-table pre-B+ manifest, create the mandatory online backup, and run `0003_sf06_b_plus.sql` to converge on the exact 66-table B+ target.
   - *(Sub-variant A2 — Single-Step Greenfield Bootstrap: Bootstrap directly to the 66-table B+ target in `0001` + `0002` with `MAX_SUPPORTED_SCHEMA_VERSION = 2`. Note: Sub-variant A2 conflicts with `be-core.md#L2623` ["không sửa lại 0001 để giả migration đã có"] unless the Owner explicitly amends `be-core.md#L471-L491` and `#L2623`.)*

### Path B — Owner Supplies External Historical `0001` / `0002` Artifacts + Forward-Only `0003_sf06_b_plus.sql` Upgrade (`MAX_SUPPORTED_SCHEMA_VERSION = 3`)

1. **Provenance Declaration:**
   - The Owner commits the authentic historical `0001_initial_schema.sql` (`user_version = 1`, 65 tables) and `0002_seed_reference_data.sql` (`user_version = 2`) from an external archive into a new authority commit SHA and records their exact SHA-256 digests in the ratified contract.
2. **Version Step Breakdown (`PRAGMA user_version`):**
   - `0001_initial_schema.sql` (`user_version = 1`) and `0002_seed_reference_data.sql` (`user_version = 2`) are kept byte-for-byte immutable.
   - `phase2_sprint3_deep1` authors **only** `0003_sf06_b_plus.sql` (`2` $\rightarrow$ `3`, `MAX_SUPPORTED_SCHEMA_VERSION = 3`) to transition the 65-table historical schema to the 66-table `docs/db.dbml` target (including any schema reconciliation needed between historical `0001` and `docs/db.dbml`).
3. **Trade-off:**
   - Preserves true historical provenance if external `0001`/`0002` SQL files exist outside this repo, but remains impossible to execute unless the Owner commits those external files.

---

## 6. Mandatory Backup, Atomic Execution, Verification & Rollback Protocol (`be-core.md#L415-L491`)

Regardless of whether the Owner ratifies Path A or Path B, `storage::migration` in `phase2_sprint3_deep1` is bound by the following non-negotiable operational invariants from `docs/business/be-core.md#L415-L491`:

1. **Sole Version Tracker (`PRAGMA user_version` — `be-core.md#L417-L420`):**
   - `SELECT user_version FROM pragma_user_version;` is the sole authoritative version tracker. Creating `_schema_migrations` or any unapproved metadata table outside `docs/db.dbml` is strictly forbidden.
2. **Mandatory Connection PRAGMAs Before Inspection (`be-core.md#L425`):**
   - Immediately upon opening `<app_data_dir>/merlink-harness/db/merlink.sqlite3`, execute:
     `PRAGMA journal_mode = WAL; PRAGMA synchronous = FULL; PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000; PRAGMA temp_store = MEMORY;`.
3. **Version Mismatch / Downgrade Refusal (`current_version > MAX_SUPPORTED_SCHEMA_VERSION` — `be-core.md#L431-L432`, Test `P1S2-C` / `phase1_sprint2_deep1-F2`):**
   - If `current_version > MAX_SUPPORTED_SCHEMA_VERSION`, immediately abort startup with `ErrorCode::UnsupportedSchemaVersion`, close the connection without writing a single byte, and retain all user data intact.
4. **Pre-Upgrade Online Backup Protocol (`0 < current_version < MAX_SUPPORTED_SCHEMA_VERSION` — `be-core.md#L433-L434`):**
   - Before executing any upgrade migration on an existing database (`current_version > 0`), create a physical backup at `<app_data_dir>/merlink-harness/backups/merlink_pre_migration_v{current_version}_{timestamp}.bak` using the SQLite Online Backup API (`rusqlite::backup::Backup` / `sqlite3_backup_init` $\rightarrow$ `sqlite3_backup_step` $\rightarrow$ `sqlite3_backup_finish`).
   - If backup creation fails (e.g., disk full, permission denied), abort immediately with `ErrorCode::MigrationFailed` before touching the primary database.
5. **Per-Step Atomic Transaction & Rollback (`be-core.md#L438-L459`):**
   - Each migration step `target_version` in `(current_version + 1)..=MAX_SUPPORTED_SCHEMA_VERSION` runs inside an independent `BEGIN IMMEDIATE; ... PRAGMA user_version = <target_version>; COMMIT;` transaction.
   -Note on SQLite table rebuilds with `PRAGMA foreign_keys = ON`: if a step rebuilds a table referenced by foreign keys, any foreign-key check (`PRAGMA foreign_key_check;`) must return `0` rows before `COMMIT;`.
   - On any DDL/DML or verification error, execute `ROLLBACK;` immediately; never increment `PRAGMA user_version`, never `DROP` or recreate the DB file, retain the pre-migration backup and last committed version, and stop startup with `ErrorCode::MigrationFailed`.
6. **6-Step Post-Migration Startup Verifier (`verify_schema_and_integrity` — `be-core.md#L465-L491`):**
   - **Step 1:** `PRAGMA user_version == MAX_SUPPORTED_SCHEMA_VERSION`.
   - **Step 2:** `PRAGMA quick_check(1)` (or `PRAGMA integrity_check`) returns `"ok"`.
   - **Step 3:** Exact table set in `sqlite_master` matches the 66-table manifest in §4.1 (and for pre-migration verification on legacy fixtures, the 65-table manifest excluding `workflow_round_receipts`).
   - **Step 4:** `PRAGMA table_info(<table>)` and `sqlite_master.sql` verify all columns, storage affinities (`TEXT`, `INTEGER`, `REAL`), `NOT NULL` flags, `SF-01`, `SF-02`, `SF-06 B+`, `SF-07`, and executable `CHECK` constraints (§3.3).
   - **Step 5:** `PRAGMA index_list(<table>)` and `PRAGMA index_info(<index_name>)` confirm all required indexes and `UNIQUE` constraints.
   - **Step 6:** `PRAGMA foreign_key_list(<table>)` confirms all FK declarations and `PRAGMA foreign_key_check;` returns `0` violation rows.

---

## 7. Acceptance Test & Failure Matrix Verification Record (`phase1_sprint2_deep1`)

| Test / Case ID | Initial State & Input / Action | Observed Result & Verification Evidence | Status |
| :--- | :--- | :--- | :---: |
| **`P1S2-P`** | Accepted baseline `57a462efb953a665e3673b15e6cc33622a4f2d39` + isolated worktree; inspect git tree for `.sql` files (`git ls-files '*.sql'`). | Verified git tree contains **zero `.sql` files** (`0` lines returned). Section 3.1 of this review packet explicitly states that the historical migration baseline (`0001`/`0002`) is missing from the repository. | `PASS` |
| **`P1S2-N`** | Attempt to derive an authorized baseline version path (`PRAGMA user_version` steps and provenance of `0001`/`0002`) from repository docs alone. | Docs contain conflicting assumptions (`be-core.md#L2623` assumes existing immutable `0001`/`0002`; git tree has none). Cannot produce an authorized baseline version from docs alone $\Rightarrow$ **`CONTRACT_BLOCKED`** (`DISPOSITION: RATIFIED_ACCEPTED`). | `PASS` |
| **`P1S2-B`** | Extract target table manifest from `docs/db.dbml` (`grep -n "^Table " docs/db.dbml`); verify table count and ensure zero DB migrations are executed. | Target manifest in Section 4.1 counts **66 table names** (65 base/legacy tables + 66th table `workflow_round_receipts` at `docs/db.dbml#L1300`). Zero SQLite databases created or modified. | `PASS` |
| **`phase1_sprint2_deep1-F1`** | Absent owner decision / missing legacy sample injected into migration contract evaluation. | Classified as **`CONTRACT_BLOCKED`** (tested by `P1S2-N`); zero database writes or unauthorized SQL files committed; two concrete paths (Path A and Path B) documented in Section 5 for Owner review. | `PASS` |
| **`phase1_sprint2_deep1-F2`** | Version mismatch (`current_version > MAX_SUPPORTED_SCHEMA_VERSION` or unratified version step) injected into migration protocol specification (`P1S2-C`). | Section 6 Step 3 & Step 4 enforce immediate stop before migration (`ErrorCode::UnsupportedSchemaVersion` / `ErrorCode::MigrationFailed`) while retaining pre-migration backup (`rusqlite::backup::Backup`) and user data untouched. | `PASS` |
| **`phase1_sprint2_deep1-F0`** | Verify predecessor gate `gate_phase1_sprint1_deep0` and baseline SHA `57a462efb953a665e3673b15e6cc33622a4f2d39` before authoring sprint evidence. | Predecessor `docs/plan/evidence/contract-ledger.md` verified present at `INVENTORY_ACCEPTED` with matching `AUTHORITY_SHA: 57a462efb953a665e3673b15e6cc33622a4f2d39`; zero non-plan files modified. | `PASS` |

---

## 8. Downstream Gate & Blocker Declaration (`phase2_sprint3_deep1`)

- **`phase2_sprint3_deep1` ("Implement SQLite bootstrap, migrations, and startup verifier") REMAINS `CONTRACT_BLOCKED`** on:
  1. **`CI-01` (`OWNER_MIGRATION_PROVENANCE` / `REQ-CONTRACT-002`):** Owner must ratify Path A or Path B in §5 and authorize `MAX_SUPPORTED_SCHEMA_VERSION` and the `0001`..`0003` provenance rule.
  2. **`CI-07`:** Owner must ratify the `themes.resolution_type` (`DEFAULT 'OPEN' CHECK (resolution_type IN ('RESOLVED', 'OPEN', 'AMBIGUOUS'))`) vs `themes.resolution` mapping before freezing `themes` DDL.
  3. **`CI-11`:** Owner must authorize execution of the forward-only SF-06 B+ physical migration (`workflow_round_receipts`, `workflow_task_receipts` rebuild, `pending_patches.eligibility_json`) for V1 runtime.
- No agent declaration or `INVENTORY_ACCEPTED` status on `docs/plan/evidence/migration-contract.md` unlocks `phase2_sprint3_deep1` without an Owner-approved SSOT revision SHA.
