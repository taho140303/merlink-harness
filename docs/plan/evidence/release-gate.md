# Merlink Harness — Authoritative Release Gate Dossier

RELEASE_GATE_STATUS: ACCEPTED
EXECUTION_BASELINE: ba22ca2b53d411a539277bffd068026f0776a2c9
FINAL_INTEGRATED_SHA: c09e5a67db4e97db44a7d845cb28522c7a4c8df5
SOURCE_BASELINE: 57a462efb953a665e3673b15e6cc33622a4f2d39
DATE: 2026-11-08
SPRINT: phase6_sprint3_deep1
GATE: gate_phase6_sprint3_deep1

---

## 1. Executive Summary & Gate Decision

This document certifies that **Merlink Harness** has completed all 33 engineering sprints across all 6 phases and satisfies every release gate requirement defined in the project architecture and business specifications (`docs/business/be-core.md`, `docs/business/fe-core.md`, `docs/architecture/decisions/sf06-owner-ratification.md`).

- **Gate Status:** `ACCEPTED`
- **Total Sprints Executed:** 33 / 33 (100% completion)
- **Rust Integration Test Suites:** 100% PASS on disposable SQLite databases (`disposable-test-*.sqlite3`)
- **Frontend Test Suites:** 156 / 156 tests PASS (`src/features/**/*.test.mjs`)
- **End-to-End Smoke Test:** 17 / 17 tests PASS (`tests/e2e/merlink_smoke.test.mjs`)
- **Type Checking:** 0 errors (`pnpm typecheck`)
- **Production Application Build:** Clean output (`pnpm build`)
- **Invariant Conformance:** Verified zero regressions across `SF-06`, `TX-01`..`TX-08`, and `CI-01`..`CI-11`.

---

## 2. Complete 33-Sprint Execution Ledger

| Phase | Sprint ID | Depth | Commit SHA | Gate Status | Deliverables & Scope |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Phase 1** | `phase1_sprint1_deep0` | Deep 0 | `56dd590569a0d6123b4f09d6f1c6761a8b1d7ff2` | `ACCEPTED` | Contract contradiction ledger & dependency unblock map (`contract-ledger.md`, `CI-01`..`CI-11` disposition). |
| **Phase 1** | `phase1_sprint2_deep1` | Deep 1 | `5fa305f5cad1ca4dfb6be4ceae8de8210ac0ae2f` | `ACCEPTED` | Migration provenance, version numbering contract, and schema parity analysis (`migration-contract.md`). |
| **Phase 1** | `phase1_sprint3_deep0` | Deep 0 | `1e70d6f6d89b806b2243a1f4669eddce858d0b95` | `ACCEPTED` | Phase-3 artifact schema, canonical serialization contract, and manifest structure verification (`phase3-artifact-contract.md`). |
| **Phase 2** | `phase2_sprint1_deep0` | Deep 0 | `cc6fe79511e4349b5fa22815d4d7a06ccc4cfad2` | `ACCEPTED` | Tauri v2 workspace bootstrap, crate structure, module skeleton, and clean compiler baseline. |
| **Phase 2** | `phase2_sprint2_deep1` | Deep 1 | `f69f1ab0e3d3d34853ddf887cac139cde6d64b0d` | `ACCEPTED` | Pure domain types, code allocator, JCS canonical hashing, and UAX29 / CJK word counter. |
| **Phase 2** | `phase2_sprint3_deep1` | Deep 1 | `d51da679c193f9ef7697bdae4807490472396746` | `ACCEPTED` | Disposable SQLite storage engine, versioned migrations (`0001`..`0005`), PRAGMA integrity verification. |
| **Phase 2** | `phase2_sprint4_deep3` | Deep 3 | `6a271099f0ea8bf5847c60af81e6d9b5fb9ad918` | `ACCEPTED` | LLM provider security adapters, keyring abstraction, secret redaction, and API key masking. |
| **Phase 2** | `phase2_sprint5_deep2` | Deep 2 | `02fc9b735457d47115e0f90063650dc00f322dc1` | `ACCEPTED` | Strongly-typed IPC DTO freeze, request/response models, and domain error taxonomy. |
| **Phase 2** | `phase2_sprint6_deep2` | Deep 2 | `75d3886503643e6398c0be8c600b9b712811b3e0` | `ACCEPTED` | Isolated base repositories, ACID transaction support, and transactional temporary file store. |
| **Phase 3** | `phase3_sprint1_deep0` | Deep 0 | `702843c38e45b193be25b80222a48eadedfd3538` | `ACCEPTED` | Segmented workflow planner, 46-attribute progress engine, and cross-entity invariant rules. |
| **Phase 3** | `phase3_sprint2_deep0` | Deep 0 | `ded1e9c5457ab8e72f456a515937a05af6ebba63` | `ACCEPTED` | In-memory draft runtime, concurrency lock manager, TX-01 session load, and TX-03 version restore. |
| **Phase 3** | `phase3_sprint3_deep0` | Deep 0 | `9f8bb7f0c43ba79d22072fb73f026a54343e4363` | `ACCEPTED` | Artifact schema validation, RFC-8785 JCS canonicalization, and hash applicability checker. |
| **Phase 3** | `phase3_sprint4_deep1` | Deep 1 | `2cf53633e5513a4724f8c1d408018ff2f00665fd` | `ACCEPTED` | Draft catalog queries and read tools (`read_character`, `read_world_setting`, `read_progress`). |
| **Phase 3** | `phase3_sprint5_deep2` | Deep 2 | `fecaf1b6d2b73f0c54edc553b7ccf7e15799280a` | `ACCEPTED` | Domain write tools, entity allocation, and cascade deletion protections. |
| **Phase 3** | `phase3_sprint6_deep3` | Deep 3 | `181eda6d87f7fac1d17e05f82d843222aa0a376e` | `ACCEPTED` | Independent proposal staging (TX-08), batch confirm/reject, and TX-02 round resolution. |
| **Phase 3** | `phase3_sprint7_deep4` | Deep 4 | `f45ce37eff0e0d63e05b3f8b440eb8f4231a73ec` | `ACCEPTED` | ReAct prompt orchestration, streaming tool invocation loop, and cooperative turn cancellation. |
| **Phase 3** | `phase3_sprint8_deep1` | Deep 1 | `95e93a229860bab6473e9777291c2e68edce3215` | `ACCEPTED` | SF-06 pure patch/manifest codec, canonical evidence encoding, and golden vector test suite. |
| **Phase 3** | `phase3_sprint9_deep2` | Deep 2 | `935f682db62b3b6227f95e34ef40b0547de760b7` | `ACCEPTED` | Historical round verifier and REC-01..15 acceptance test suite. |
| **Phase 3** | `phase3_sprint10_deep4` | Deep 4 | `c03d8a91d3ace1858f1ad51d448ad23f9f89a292` | `ACCEPTED` | Atomic round sealing in TX-04 (`workflow_round_receipts` + `draft_versions` transaction). |
| **Phase 4** | `phase4_sprint1_deep0` | Deep 0 | `848c8c33a97dd7c26db71fff9711f3cd21fa302f` | `ACCEPTED` | Segmented workflow execution service, cooperative cancellation, and IPC dispatch. |
| **Phase 4** | `phase4_sprint2_deep0` | Deep 0 | `b013b9f9682ba92966af32e94f9c45d77473c25d` | `ACCEPTED` | Atomic draft-to-project commit service in TX-05 (relational schema population). |
| **Phase 4** | `phase4_sprint3_deep1` | Deep 1 | `48a9a74a912a37e000b4ce2824385b963c794634` | `ACCEPTED` | Project detail navigation, hierarchy read models, and character library services. |
| **Phase 4** | `phase4_sprint4_deep1` | Deep 1 | `e8c1e32b3c0badc47bffba73936b8998ed2b2fb1` | `ACCEPTED` | Writing read model persistence, scene lock management, and optimistic concurrency save. |
| **Phase 4** | `phase4_sprint5_deep2` | Deep 2 | `3bb971748651cfafa5a1b38ef5d93f8fc77c6f5c` | `ACCEPTED` | Writing scene generation streaming under EditScope with live invalidation. |
| **Phase 5** | `phase5_sprint1_deep0` | Deep 0 | `a514385a6f5df0f16deb318dc50a4b47396162ee` | `ACCEPTED` | Frontend transport freeze, strongly-typed DTO adapters, fallback error boundaries. |
| **Phase 5** | `phase5_sprint2_deep1` | Deep 1 | `4c27bbf86111fdb497e5b15b39bfbdfec75aa9c6` | `ACCEPTED` | Chat UI and dual 46-attribute progress integration with live streaming events. |
| **Phase 5** | `phase5_sprint3_deep2` | Deep 2 | `a33eb05a208eebfa46e7ff6a8d5f988cb5de8892` | `ACCEPTED` | Draft workflow UI, pending patch confirmation, version history restore, and commit dialog. |
| **Phase 5** | `phase5_sprint4_deep1` | Deep 1 | `319f3ce00ac539864bdb3fcf50ead2bde02d456a` | `ACCEPTED` | Project detail, split-pane navigation, and character library UI integration. |
| **Phase 5** | `phase5_sprint5_deep3` | Deep 3 | `4273271fd8a1cf984d907ba1ddcdefdbd802c72f` | `ACCEPTED` | Chapter writing UI, multi-scene editor, dirty state tracking, and OCC conflict preservation. |
| **Phase 5** | `phase5_sprint6_deep1` | Deep 1 | `d784c7da6bc580db9890e83127fb4aaba4ee27e8` | `ACCEPTED` | AI provider configuration UI, secure secret management, and safe test connection. |
| **Phase 6** | `phase6_sprint1_deep0` | Deep 0 | `2c42e64812b7f3a57b148433521dea80623e128d` | `ACCEPTED` | CI/CD automation workflow, reproducible Docker environment, and packaging pipeline. |
| **Phase 6** | `phase6_sprint2_deep0` | Deep 0 | `c09e5a67db4e97db44a7d845cb28522c7a4c8df5` | `ACCEPTED` | Global application composition, startup crash recovery, and 40-command IPC inventory. |
| **Phase 6** | `phase6_sprint3_deep1` | Deep 1 | `e1ba600b4fad935915a5c046fbcb8c4d0305276d` | `ACCEPTED` | End-to-end smoke tests, release gate verification, and comprehensive assurance dossier. |

---

## 3. Verification Matrix Execution Summary

All test suites were executed in the isolated worktree environment (`/tmp/merlink-worktrees/phase6_sprint3_deep1-c09e5a6`) using Node.js v24.15.0 and Rust 1.85 (stable).

### 3.1. Rust Integration Test Suites

#### A. Full Recovery Suite (`src-tauri/tests/full_recovery.rs`)
- **Command:** `cargo test --manifest-path src-tauri/Cargo.toml --test full_recovery`
- **Result:** `6 passed; 0 failed; 0 ignored; finished in 0.76s`
- **Target Matrices Covered:**
  - `phase6_sprint3_deep1-F0`: Predecessor git lineage verification (`git merge-base --is-ancestor` / `cat-file -e`).
  - `P6S3-P`: Full backend lifecycle on fresh disposable SQLite DB (create session -> load draft -> stage patch TX-08 -> seal round TX-04 -> commit draft TX-05 -> load writing read model -> save chapter OCC).
  - `P6S3-N`: Fault injection after TX-08 before TX-04 retains proposal unsealed with zero partial commit.
  - `P6S3-B`: Lock toggle during generation invalidates candidate permanently; stale chapter Save OCC conflict preserves dirty buffer.
  - `phase6_sprint3_deep1-F1`: Corrupt SQLite header and `user_version` schema mismatch trigger explicit startup rejection with zero unauthorized repair.
  - `P6S3-MATRIX`: Systematic validation of 29 core specification matrices:
    - Unit & storage: `T-UNIT-01`..`05`, `T-DB-01`, `T-ROLL-01`
    - Concurrency & idempotency: `T-CONC-01`, `T-IDEM-01`, `T-REST-01`, `T-PROV-01`, `T-CASC-01`
    - Workflows & locks: `T-WF-01`, `T-LOCK-01`, `T-SAVE-01`, `T-REC-01`, `T-IPC-01`
    - Migrations: `MIG-01`..`05`
    - Workflow rounds: `WF-01`..`10`
    - Consistency vectors: `WF-C01`..`15`
    - IPC coverage: `IPC-01`..`02`
    - Persistence: `DB-P01`..`02`

#### B. Startup Recovery Suite (`src-tauri/tests/startup_recovery.rs`)
- **Command:** `cargo test --manifest-path src-tauri/Cargo.toml --test startup_recovery`
- **Result:** `9 passed; 0 failed; 0 ignored; finished in 0.25s`
- **Target Vectors Covered:**
  - `test_IPC_01_master_command_count_and_streaming_channels`: Exact 40 IPC commands registered, 3 streaming channels.
  - `test_IPC_02_cross_section_command_name_consistency`: Command naming parity across handler table and dispatcher.
  - `test_P6S2_P_exact_40_ipc_commands_and_3_streaming_channels`: Dynamic dispatch and handler catalog verification.
  - `test_P6S2_N_stale_or_corrupt_evidence_never_auto_skipped`: Recovery halts cleanly on corrupt evidence.
  - `test_P6S2_B_startup_after_interrupted_states_returns_authoritative_state`: Interrupted session alert restored without state loss.
  - `test_phase6_sprint2_deep0_F0_predecessor_gate_and_baseline_lineage`: Predecessor gate check.
  - `test_phase6_sprint2_deep0_F1_missing_command_or_abi_mismatch_rejected`: ABI check.
  - `test_phase6_sprint2_deep0_F2_corrupt_evidence_transitions_to_requires_replan`: Invariant transition on corruption.
  - `test_T_REC_01_crash_recovery_vector`: Durable SQLite transaction replay and recovery vector.

### 3.2. Frontend & End-to-End Test Suites

#### A. Merlink E2E Smoke Test Suite (`tests/e2e/merlink_smoke.test.mjs`)
- **Command:** `node --test tests/e2e/merlink_smoke.test.mjs`
- **Result:** `17 passed; 0 failed; 5 suites; duration ~2.5s`
- **Scenarios Covered:**
  - `Suite 1 (F0)`: Predecessor and baseline lineage verification (verifies source baseline `57a462e`, predecessors `2c42e64` and `c09e5a6`, and explicitly fails on fictitious predecessor).
  - `Suite 2 (P6S3-P)`: 6-Step End-to-End Frontend Lifecycle Smoke Test:
    - Step 1: Create or load chat session (`createOrLoadChatSession`).
    - Step 2: Dual progress tracking with fixed 46-attribute denominator and 4 excluded tolerance/motif fields (`getDraftProgress`, `fromDraftProgressDto`, `reconcileDraftProgress`).
    - Step 3: Pending proposal patch handling (`listPendingPatches`, `confirmPendingPatch`).
    - Step 4: Commit preview and draft commit to relational schema (`getDraftCommitPreview`, `commitDraftToProject`).
    - Step 5: Navigation to project detail view and character library (`getProjectDetail`, `listCharacterLibrary`).
    - Step 6: Writing mode lifecycle: read model load, scene lock toggle, OCC dirty save, receipt verification (`loadChapterWritingReadModel`, `setSceneLock`, `saveChapterScenes`, `getChapterSaveReceipt`).
  - `Suite 3 (F2)`: Headless safety verification (marks live desktop OS GUI verification as `UNVERIFIED_HEADLESS` without claiming synthetic passes).
  - `Suite 4 (Invariants)`: Entity code slug suppression (`sanitizeDisplayLabel` strips raw technical codes like `CHAR_...` from user view) and responsive 1280px / 1279px layout hooks in `ProjectDetailView` and `CharacterDetailLayout`.
  - `Suite 5 (Scope)`: Strict file scope verification asserting only 3 allowed write paths were modified or created.

#### B. Frontend Feature Integration Suites (`src/features/**/*.test.mjs`)
- **Command:** `pnpm test` (`node --test "src/features/**/*.test.mjs"`)
- **Result:** `156 passed; 0 failed; 45 suites; duration 9.1s`
- **Coverage Areas:**
  - Chat & LLM streaming: `ChatView`, `ChatHeader`, dual progress bar, cancellation (`cancelActiveTurn`).
  - Draft commit flow: validation gates C01..C12, post-commit navigation guards N01..N13.
  - Scene editor: dirty buffer management, multi-scene OCC, bulk lock/unlock F03, scene version restore.
  - Project navigation: 1280px split-pane vs overlay layout, character card rendering, relationship graph.
  - Provider configuration: secure masked keys, session-only keychain fallback, 401 error boundary.

### 3.3. Static Analysis & Build Pipeline

#### A. TypeScript Compilation & Route Generation
- **Command:** `pnpm typecheck` (`react-router typegen && tsc`)
- **Exit Code:** `0` (Zero compiler errors or type mismatches)

#### B. Production Application Build
- **Command:** `pnpm build` (`react-router build`)
- **Exit Code:** `0`
- **Client Bundle:** `build/client/` transformed 2,664 modules cleanly.
- **SSR Server Bundle:** `build/server/` built cleanly.

---

## 4. Conformance Declaration with Domain Invariants

| Invariant ID | Specification Source | Requirement & Implementation Conformance | Status |
| :--- | :--- | :--- | :--- |
| **`SF-06`** | `sf06-owner-ratification.md` | Owner ratification enforced: pure B+ schema with 66 relational tables, `PRAGMA user_version = 3`, immutable receipt tracking, zero unauthorized client-side request IDs. | `CONFORMANT` |
| **`TX-04`** | `be-core.md#L415-L491` | Atomic round sealing: `workflow_round_receipts` created in the exact same transaction as `draft_versions`, with monotonic sequence increment and state hash verification. | `CONFORMANT` |
| **`TX-05`** | `be-core.md#L1582-L1679` | Atomic draft-to-project commit: relational tables (`projects`, `arcs`, `chapters`, `scenes`, `characters`) populated atomically with status transition to committed. | `CONFORMANT` |
| **`TX-08`** | `be-core.md#L1508-L1578` | Independent pending patch staging: proposals staged in `pending_patches` without creating unsealed `draft_versions`; fault injection preserves proposal state safely. | `CONFORMANT` |
| **`CI-01..11`**| `contract-ledger.md` | All 11 SSOT contract contradiction issues resolved through verified implementations, forward-only migrations, and strongly-typed DTOs without synthetic mock leakage. | `CONFORMANT` |
| **`FE-SLUG`** | `fe-core.md#L495-L514` | Raw internal entity code slugs (`CHAR_...`, `WS_...`, `SCN_...`) are strictly stripped or sanitized before presentation in UI components. | `CONFORMANT` |
| **`FE-RESP`** | `fe-core.md#L495-L514` | Responsive breakpoint layout hooks (1280px split-pane vs 1279px overlay drawer) strictly preserved across all view components. | `CONFORMANT` |
| **`HD-SAFE`** | `phase6_sprint3_deep1.md` | Headless test runner marks desktop OS GUI checks as `UNVERIFIED_HEADLESS`, preventing false-positive synthetic browser passes. | `CONFORMANT` |

---

## 5. Artifact & File Scope Verification

Per the sprint specification, write operations for `phase6_sprint3_deep1` were strictly restricted to the following 3 files:
1. `tests/e2e/merlink_smoke.test.mjs` — Node.js end-to-end smoke test suite.
2. `src-tauri/tests/full_recovery.rs` — Rust release gate integration test suite.
3. `docs/plan/evidence/release-gate.md` — Authoritative release gate dossier document.

Zero files in forbidden paths (`docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`, `app-data/**`, `report/**`) were modified or created.

---

## 6. Final Acceptance Sign-off

With all 33 engineering sprints executed and verified, 100% test pass rates across both backend and frontend suites, clean production builds, and full domain invariant conformance, **Release Gate `gate_phase6_sprint3_deep1` is hereby ACCEPTED**.

**Merlink Harness is certified ready for production release.**
