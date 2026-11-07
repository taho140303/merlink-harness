---
sprint_id: phase2_sprint3_deep1
title: Implement disposable SQLite bootstrap and migrations
phase: 2
sprint: 3
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase2_sprint3_deep1 — Implement disposable SQLite bootstrap and migrations

## 1. Sprint identity

Engineering phase 2 (Foundation and frozen interfaces), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement disposable SQLite bootstrap and migrations. Observable handoff: Versioned SQL filenames, DbManager, schema manifest and disposable migration tests.

## 3. Scope boundaries

**IN_SCOPE:** Use official SQLite migration tooling supported by frozen Cargo; author only authorized SQL files.; Exercise fresh, 65→66 legacy fixture, interrupted migration, too-new user_version, JSON1/FK/index/CHECK and byte-preserving backup using disposable DB.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Do not start until P1S2 records owner-authorized baseline/version path. Build target 66 tables from DBML, not nonexistent historical SQL; enumerate all table names with `rg -n "^Table " docs/db.dbml` before DDL.

- Online Backup API before upgrade; transactional table rebuild keeps legacy triplet NULL, `foreign_key_check`, integrity, rollback and explicit no auto-genesis.

- **REQ-DB-001** `docs/business/be-core.md#L361-L382`: WAL, synchronous FULL, foreign_keys ON, busy_timeout 5000 on all connections. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L361: ## §3. SQLite Persistence (Lưu trữ bền vững với SQLite)

  > L363: ### 3.1 Cấu hình Kết nối, WAL Mode, Khóa ngoại & Độ bền (`PRAGMAs`)

  > L365: Mỗi kết nối SQLite được mở bởi `storage::db::DbManager` `MUST` thực thi ngay lập tức khối lệnh cấu hình sau trước khi phục vụ bất kỳ truy vấn nào:

  > L367: ```sql

- **REQ-DB-002** `docs/db.dbml#L1299-L1378`: Materialize the SF-06 B+ header/child/pending target within exactly 66 DBML tables; preserve all other physical tables/indexes/FKs/CHECKs; no unrelated schema. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1299: // B+ SF-06: hợp đồng vật lý đích; chưa phải migration đã chạy. Header bất biến theo protocol trusted backend.

  > L1300: Table workflow_round_receipts {

  > L1301: id uuid [pk, note: 'UUIDv7; commit identity kể cả zero-net']

  > L1302: draft_session_id uuid [ref: > draft_sessions.id, not null]

- **REQ-DB-003** `docs/business/be-core.md#L415-L491`: PRAGMA user_version forward migrations, backup API, rollback, schema/FK/integrity checks. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L415: ### 3.4 Khởi tạo Cơ sở dữ liệu, Quản lý Phiên bản Schema & Giao thức Thực thi Migration (`PRAGMA user_version`)

  > L417: 1. **Cơ chế theo dõi phiên bản Schema duy nhất (`PRAGMA user_version`):**

  > L418: - Để tuân thủ tuyệt đối `docs/db.dbml` (Physical Schema SSOT) và không tự ý đưa thêm bất kỳ bảng metadata nào nằm ngoài Schema SSOT vào cơ sở dữ liệu vật lý, Backend `MUST` sử dụng **`PRAGMA user_version` nguyên bản của SQLite làm cơ chế duy nhất có thẩm quyền (sole authoritative migration-version t

  > L419: - Backend `MUST NOT` tạo hoặc duy trì bảng `_schema_migrations` hay bất kỳ bảng theo dõi lịch sử migration tùy chỉnh nào không được định nghĩa trong `docs/db.dbml`.

- **REQ-DB-004** `docs/business/be-core.md#L1578-L1578`: Preserve legacy rows, nullable SF-06 evidence and phase; fail closed; no automatic genesis. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1578: **Legacy/migration/test:** `isRequireComplete=false, head=NULL` draft chưa ready; `isRequireComplete=true, head=NULL` ⇒ `WorkflowBaselineUnavailable`, không genesis `seq=0`/RAM/backfill; có thể inspect draft, sửa thủ công chỉ khi có nguồn snapshot đáng tin, quyền và kế hoạch backup/đối chiếu, ngoài

- **REQ-SPECTEST-T-DB-01** `docs/business/be-core.md#L2528-L2528`: MUST implement executable BE acceptance vector T-DB-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-MIG-01** `docs/business/be-core.md#L2547-L2547`: MUST implement executable BE acceptance vector MIG-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-MIG-02** `docs/business/be-core.md#L2548-L2548`: MUST implement executable BE acceptance vector MIG-02, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-MIG-03** `docs/business/be-core.md#L2549-L2549`: MUST implement executable BE acceptance vector MIG-03, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-MIG-04** `docs/business/be-core.md#L2550-L2550`: MUST implement executable BE acceptance vector MIG-04, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-MIG-05** `docs/business/be-core.md#L2551-L2551`: MUST implement executable BE acceptance vector MIG-05, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-DB-P01** `docs/business/be-core.md#L2586-L2586`: MUST implement executable BE acceptance vector DB-P01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-DB-P02** `docs/business/be-core.md#L2587-L2587`: MUST implement executable BE acceptance vector DB-P02, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L361-L491` — focus: PRAGMAs, backup, user_version, schema verifier; CLI: `sed -n "361,491p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/db.dbml#L1273-L1378` — focus: Version/header/child/pending target; CLI: `sed -n "1273,1378p" docs/db.dbml` (or `rg -n "<symbol>" docs/db.dbml` for a changed anchor).

- `docs/business/be-core.md#L1549-L1578` — focus: B+ composite constraints and legacy; CLI: `sed -n "1549,1578p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/architecture/decisions/sf06-owner-ratification.md#L56-L58` — focus: No executed migration; CLI: `sed -n "56,58p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase2_sprint1_deep0", "phase1_sprint2_deep1"]`.

- `phase2_sprint1_deep0` [BUILD_DEPENDENCY] produces Compilable Cargo workspace; module-root scaffolding only (no downstream implementation files) and exact dependency lockfile; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint1_deep0`. If absent, do not start.

- `phase1_sprint2_deep1` [MIGRATION_DEPENDENCY] produces docs/plan/evidence/migration-contract.md with authorized version path, 65→66 manifest provenance or blocker; inspect its accepted commit, exported artifact/test and `gate_phase1_sprint2_deep1`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/migrations/*.sql

  - src-tauri/src/storage/db.rs

  - src-tauri/src/storage/migration.rs

  - src-tauri/tests/migration_contract.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/db.dbml

  - docs/architecture/decisions/sf06-owner-ratification.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: DbManager::initialize_and_migrate(app_data_dir)->Result<DbPool,AppError>; verify_schema_and_integrity(conn). Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Use official SQLite migration tooling supported by frozen Cargo; author only authorized SQL files.

- Exercise fresh, 65→66 legacy fixture, interrupted migration, too-new user_version, JSON1/FK/index/CHECK and byte-preserving backup using disposable DB.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Do not start until P1S2 records owner-authorized baseline/version path. Build target 66 tables from DBML, not nonexistent historical SQL; enumerate all table names with `rg -n "^Table " docs/db.dbml` before DDL.

- Online Backup API before upgrade; transactional table rebuild keeps legacy triplet NULL, `foreign_key_check`, integrity, rollback and explicit no auto-genesis.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| No P1S2 authorization | CONTRACT_BLOCKED, no SQL publication; P2S3-N. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint3_deep1-F1 |

| Invalid FK/JSON/version/backup failure | rollback and stop startup, preserve user data; P2S3-C. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint3_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase2_sprint3_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P2S3-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: disposable new DB has exactly 66 target tables, required composite FKs and PRAGMAs. | disposable new DB has exactly 66 target tables, required composite FKs and PRAGMAs. | No unrelated write; preserve exact expected state and listed invariant |

| P2S3-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: child index=N passes SQL but backend gate required; do not overstate CHECK. | child index=N passes SQL but backend gate required; do not overstate CHECK. | No unrelated write; preserve exact expected state and listed invariant |

| P2S3-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: simulated upgrade keeps old receipts byte-identical and new fields NULL. | simulated upgrade keeps old receipts byte-identical and new fields NULL. | No unrelated write; preserve exact expected state and listed invariant |

| P2S3-C | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: interrupted DDL leaves user_version and rows unchanged. | interrupted DDL leaves user_version and rows unchanged. | No unrelated write; preserve exact expected state and listed invariant |

| phase2_sprint3_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject No P1S2 authorization into owned operation | CONTRACT_BLOCKED, no SQL publication; P2S3-N. | No unauthorized commit or external replay |

| phase2_sprint3_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Invalid FK/JSON/version/backup failure into owned operation | rollback and stop startup, preserve user data; P2S3-C. | No unauthorized commit or external replay |

| T-DB-01 | disposable/synthetic fixture described by BE | Khởi tạo DB từ migration lịch sử `0001_initial_schema.sql` rồi chạy chuỗi migration forward-only tới B+ target; kiểm tra `PRAGMA user_version` và danh sách bảng trong `sqlite_master`; thử chèn `characters` có cả `display_name` và `nickname` đều `NULL`; thử chèn `draft_versions` có `phase = 2, subphase = 1`; thử chèn `scene_versions` có `parent_id = NULL`; thử chèn `workflow_task_receipts` có `had_mutations = 0, resulting_version_seq = 5`. | `PRAGMA journal_mode` trả `"wal"`, `foreign_keys` trả `1`, `synchronous` trả `2` (`FULL`), `PRAGMA user_version` bằng đúng `MAX_SUPPORTED_SCHEMA_VERSION`, đủ đúng 66 khai báo `Table` của target B+ `docs/db.dbml` sau forward-only migration (không có bảng metadata ngoài luồng). Tất cả 4 lệnh `INSERT` vi phạm ràng buộc đều bị chính SQLite chặn bằng lỗi `SQLITE_CONSTRAINT`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| MIG-01 | disposable/synthetic fixture described by BE | Mở file SQLite mới hoàn toàn (`PRAGMA user_version == 0`); chạy `run_migrations` và `verify_schema_and_integrity` (§3.4, §3.5). | Toàn bộ migration chạy tuần tự lên `MAX_SUPPORTED_SCHEMA_VERSION`; `PRAGMA user_version` bằng `MAX_SUPPORTED_SCHEMA_VERSION`; tạo đủ đúng 66 khai báo `Table` của target B+ `docs/db.dbml` sau forward-only migration (bao gồm `SF-01`, `SF-02`, `SF-06`, `SF-07`), đầy đủ index/FK/`CHECK` constraints; **không tồn tại bảng `_schema_migrations`**. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| MIG-02 | disposable/synthetic fixture described by BE | Mở file SQLite đang ở `0 < user_version < MAX_SUPPORTED_SCHEMA_VERSION` có sẵn dữ liệu người dùng; chạy `run_migrations` và `verify_schema_and_integrity`. | Tạo file backup vật lý `merlink_pre_migration_v{old}_{timestamp}.bak` trước khi chạy DDL; thực thi tuần tự từng migration trong `BEGIN IMMEDIATE ... COMMIT`; dữ liệu người dùng được bảo toàn 100%; schema sau nâng cấp hội tụ giống hệt schema của `MIG-01`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| MIG-03 | disposable/synthetic fixture described by BE | Giả lập lỗi SQL ở giữa migration lên phiên bản `k` (sau khi phiên bản `k-1` đã commit thành công). | Giao dịch của phiên bản `k` được `ROLLBACK` hoàn toàn; `PRAGMA user_version` giữ nguyên ở `k-1` (**không** tăng lên `k`); không có bảng/cột dở dang của `k`; ứng dụng dừng khởi động với `ErrorCode::MigrationFailed` và giữ nguyên dữ liệu người dùng. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| MIG-04 | disposable/synthetic fixture described by BE | Mở file SQLite có `PRAGMA user_version = MAX_SUPPORTED_SCHEMA_VERSION + 1`. | Backend trả về ngay `ErrorCode::UnsupportedSchemaVersion`, đóng kết nối mà không ghi/sửa/xóa bất kỳ byte nào của DB, và dừng khởi động an toàn. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| MIG-05 | disposable/synthetic fixture described by BE | Khởi tạo DB có `PRAGMA user_version == MAX_SUPPORTED_SCHEMA_VERSION` nhưng cố tình thiếu cột `draft_versions.seal_key` (`SF-01`), thiếu bảng `workflow_task_receipts` (`SF-06`), hoặc có dòng vi phạm khóa ngoại; gọi `verify_schema_and_integrity` (§3.5). | Hàm `verify_schema_and_integrity` phát hiện ngay sự sai lệch qua `sqlite_master` / `PRAGMA table_info` / `PRAGMA index_list` / `PRAGMA foreign_key_check` (không tin mù quáng vào `PRAGMA user_version`) và chặn khởi động với `ErrorCode::MigrationFailed`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| DB-P01 | disposable/synthetic fixture described by BE | Khởi chạy `DbManager::initialize_and_migrate(app_data_dir)` trên môi trường mới. Đối chiếu đường dẫn tệp SQLite được mở tại §3.4 với cây thư mục tại §1.6. | Mở và khởi tạo tệp cơ sở dữ liệu chính tại đúng đường dẫn chuẩn duy nhất `<app_data_dir>/merlink-harness/db/merlink.sqlite3` (cùng cặp tệp phụ trợ `-wal` / `-shm` trong `<app_data_dir>/merlink-harness/db/`); hoàn toàn không tồn tại đường dẫn `<app_data_dir>/merlink-harness/data/merlink.db`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| DB-P02 | disposable/synthetic fixture described by BE | `current_version > 0` và cần nâng cấp. | Dùng SQLite Online Backup API vào `<app_data_dir>/merlink-harness/backups/merlink_pre_migration_v{current_version}_{timestamp}.bak` trước DDL; không dùng `VACUUM INTO` hay convention khác. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase2_sprint3_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test migration_contract  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase2_sprint2_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Versioned SQL filenames, DbManager, schema manifest and disposable migration tests

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase2_sprint3_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Versioned SQL filenames, DbManager, schema manifest and disposable migration tests

- Exact artifacts: src-tauri/migrations/*.sql, src-tauri/src/storage/db.rs, src-tauri/src/storage/migration.rs, src-tauri/tests/migration_contract.rs; accepted integration SHA and `gate_phase2_sprint3_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** OWNER_MIGRATION_PROVENANCE, CI-01, CI-07, CI-11, CONTRACT_REVIEW_REQUIRED:REQ-DB-001, CONTRACT_REVIEW_REQUIRED:REQ-DB-002, CONTRACT_REVIEW_REQUIRED:REQ-DB-003, CONTRACT_REVIEW_REQUIRED:REQ-DB-004.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
