---
sprint_id: phase2_sprint4_deep3
title: Implement provider security and adapters
phase: 2
sprint: 4
deep: 3
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase2_sprint4_deep3 — Implement provider security and adapters

## 1. Sprint identity

Engineering phase 2 (Foundation and frozen interfaces), within-phase depth 3; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement provider security and adapters. Observable handoff: LlmProvider trait, adapters, credential/redaction APIs and config IPC #38–40.

## 3. Scope boundaries

**IN_SCOPE:** Implement fake-provider integration fixture and official supported client API; no live external calls in tests.; Own config command module only; registration and shared DTO/Cargo belong P6S2/P2S5/P2S1.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Ollama loopback default and OpenCode permission all deny; HTTP timeout/cancel and structured error classes.

- OS Keychain or session-only fallback; no plaintext key in SQLite/localStorage/logs/errors or IPC responses.

- `update_runtime_and_provider_config` is runtime configuration only: persist no provider config unless a committed authoritative source explicitly requires persistence; no implementation may claim settings survive restart without evidence.

- **REQ-PROVIDER-001** `docs/business/be-core.md#L2027-L2070`: Ollama/OpenCode adapters, permission all deny, timeouts/cancellation/error mapping. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L2027: ## §9. Model Providers & Security (Cổng Kết nối Model Provider & Bảo mật)

  > L2029: ### 9.1 Trừu tượng hóa `LlmProvider` & Vòng đời Request / Streaming

  > L2031: `providers::traits::LlmProvider` cung cấp giao diện thống nhất cho cả chế độ gọi có tool (Planning Mode ReAct Loop) và chế độ sinh văn bản streaming (Writing Mode Scene Generation):

  > L2033: ```rust

- **REQ-SEC-001** `docs/business/be-core.md#L2071-L2098`: Keychain/session-only fallback, no plaintext secret in DB/log, redaction, path protection. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L2071: ### 9.2 Lưu trữ Thông tin Xác thực (OS Keychain & Session-Only Fallback)

  > L2073: > **[!CAUTION]**

  > L2074: > **Bất biến Bảo mật Tuyệt đối (`MUST NOT`):**

  > L2075: > 1. **Tuyệt đối không** lưu API key, access token hoặc secret dưới dạng văn bản thuần (plaintext) trong bất kỳ bảng SQLite nào, file cấu hình JSON, hay file log.

- **REQ-CMD-038** `docs/business/be-core.md#L2155-L2155`: MUST implement typed IPC #38 `get_runtime_and_provider_config` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-039** `docs/business/be-core.md#L2156-L2156`: MUST implement typed IPC #39 `update_runtime_and_provider_config` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-040** `docs/business/be-core.md#L2157-L2157`: MUST implement typed IPC #40 `set_provider_api_key` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-IPC-01** `docs/business/be-core.md#L2539-L2539`: MUST implement executable BE acceptance vector T-IPC-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L2027-L2098` — focus: Provider trait, permission deny, timeouts, keychain/redaction; CLI: `sed -n "2027,2098p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2245-L2251` — focus: Config IPC #38–40; CLI: `sed -n "2245,2251p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/fe-core.md#L32-L51` — focus: Existing provider UI mock boundary only; CLI: `sed -n "32,51p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-38 `get_runtime_and_provider_config` — `docs/business/be-core.md#L2249`: **`get_runtime_and_provider_config`**  ·  `{}`  ·  `RuntimeAndProviderConfigDto { subagentConfig: SubagentConfigDto, providers: ProviderStatusDto[], credentialStorageMode: "os_keychain" \ ·  "session_only_fallback" }`  ·  Kiểm tra trạng thái kết nối OS Keychain và cấu hình `subagentConfig` (đồng bộ với `src/features/settings/subagentConfig.ts`).  ·  Read-only.  ·  `InternalError`  ·  Không áp dụng.

- IPC-39 `update_runtime_and_provider_config` — `docs/business/be-core.md#L2250`: **`update_runtime_and_provider_config`**  ·  `{ subagentConfig?: SubagentConfigDto, activeProviderId?: String, modelName?: String, endpointUrl?: String }`  ·  `RuntimeAndProviderConfigDto`  ·  Kiểm tra các giới hạn số học của `subagentConfig` (`1 <= maxSubagents <= 6`, `0 <= outlineLookback < outlineCapacity` theo `subagentConfig.ts:20-51` và `core.md` §3.9).  ·  Cập nhật cấu hình; tuyệt đối không ghi API key vào SQLite.  ·  `ValidationFailed`  ·  Không áp dụng.

- IPC-40 `set_provider_api_key` — `docs/business/be-core.md#L2251`: **`set_provider_api_key`**  ·  `{ providerId: String, apiKey: String }`  ·  `{ providerId: String, storageMode: "os_keychain" \ ·  "session_only_fallback" }`  ·  `apiKey.trim()` không rỗng; đăng ký chuỗi khóa vào bộ lọc `redaction` trước khi lưu.  ·  Lưu vào OS Keychain (`keyring`); nếu OS Keychain không khả dụng thì lưu vào `SessionKeyStore` trong RAM và trả `session_only_fallback` (§9.2).  ·  `ValidationFailed`, `InternalError`  ·  Không áp dụng.

## 5. Prerequisites

`depends_on: ["phase2_sprint1_deep0", "phase2_sprint5_deep2", "phase1_sprint1_deep0"]`.

- `phase2_sprint1_deep0` [BUILD_DEPENDENCY] produces Compilable Cargo workspace; module-root scaffolding only (no downstream implementation files) and exact dependency lockfile; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint1_deep0`. If absent, do not start.

- `phase2_sprint5_deep2` [TYPE_DEPENDENCY] produces Frozen DTO/error/adapter Rust API plus generated TS binding path and schema fixture; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint5_deep2`. If absent, do not start.

- `phase1_sprint1_deep0` [INTEGRATION_DEPENDENCY] produces docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs; inspect its accepted commit, exported artifact/test and `gate_phase1_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/providers/traits.rs

  - src-tauri/src/providers/ollama.rs

  - src-tauri/src/providers/opencode.rs

  - src-tauri/src/providers/credentials.rs

  - src-tauri/src/providers/redaction.rs

  - src-tauri/src/ipc/config_commands.rs

  - src-tauri/tests/provider_security.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/fe-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PREREQUISITE_INTERFACE: frozen IpcError/DTO. PROPOSED_INTERFACE: LlmProvider::{chat_with_tools,stream_completion}, CredentialManager, Redactor. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement fake-provider integration fixture and official supported client API; no live external calls in tests.

- Own config command module only; registration and shared DTO/Cargo belong P6S2/P2S5/P2S1.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Ollama loopback default and OpenCode permission all deny; HTTP timeout/cancel and structured error classes.

- OS Keychain or session-only fallback; no plaintext key in SQLite/localStorage/logs/errors or IPC responses.

- `update_runtime_and_provider_config` is runtime configuration only: persist no provider config unless a committed authoritative source explicitly requires persistence; no implementation may claim settings survive restart without evidence.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| 401/403 or 429/502/503/504 | map to ProviderAuthFailed(nonretryable) or ProviderUnreachable(retryable) per BE; P2S4-N. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint4_deep3-F1 |

| Keychain unavailable | session-only fallback and redaction; no disk secret; P2S4-C. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint4_deep3-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase2_sprint4_deep3-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P2S4-P | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: fake Ollama/OpenCode requests prove tools and `permission:{"*":"deny"}`. | fake Ollama/OpenCode requests prove tools and `permission:{"*":"deny"}`. | No unrelated write; preserve exact expected state and listed invariant |

| P2S4-N | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: log/error snapshot never contains synthetic secret. | log/error snapshot never contains synthetic secret. | No unrelated write; preserve exact expected state and listed invariant |

| P2S4-B | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: cancellation drops request and returns typed error. | cancellation drops request and returns typed error. | No unrelated write; preserve exact expected state and listed invariant |

| phase2_sprint4_deep3-F1 | accepted prerequisite fixture / isolated worktree | Inject 401/403 or 429/502/503/504 into owned operation | map to ProviderAuthFailed(nonretryable) or ProviderUnreachable(retryable) per BE; P2S4-N. | No unauthorized commit or external replay |

| phase2_sprint4_deep3-F2 | accepted prerequisite fixture / isolated worktree | Inject Keychain unavailable into owned operation | session-only fallback and redaction; no disk secret; P2S4-C. | No unauthorized commit or external replay |

| IPC-CMD-038 | isolated authorized and unauthorized caller fixtures | Invoke `get_runtime_and_provider_config` with valid request and invalid/stale variant; use frozen DTO | /  **`get_runtime_and_provider_config`**  /  `{}`  /  `RuntimeAndProviderConfigDto { subagentConfig: SubagentConfigDto, providers: ProviderStatusDto[], credentialStorageMode: "os_keychain" \ /  "session_only_fallback" }`  /  Kiểm tra trạng thái kết nối OS Keychain và cấu hình `subagentConfig` (đồng bộ với `src/features/settings/subagentConfig.ts`).  /  Read-only.  /  `InternalError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-039 | isolated authorized and unauthorized caller fixtures | Invoke `update_runtime_and_provider_config` with valid request and invalid/stale variant; use frozen DTO | /  **`update_runtime_and_provider_config`**  /  `{ subagentConfig?: SubagentConfigDto, activeProviderId?: String, modelName?: String, endpointUrl?: String }`  /  `RuntimeAndProviderConfigDto`  /  Kiểm tra các giới hạn số học của `subagentConfig` (`1 <= maxSubagents <= 6`, `0 <= outlineLookback < outlineCapacity` theo `subagentConfig.ts:20-51` và `core.md` §3.9).  /  Cập nhật cấu hình; tuyệt đối không ghi API key vào SQLite.  /  `ValidationFailed`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-040 | isolated authorized and unauthorized caller fixtures | Invoke `set_provider_api_key` with valid request and invalid/stale variant; use frozen DTO | /  **`set_provider_api_key`**  /  `{ providerId: String, apiKey: String }`  /  `{ providerId: String, storageMode: "os_keychain" \ /  "session_only_fallback" }`  /  `apiKey.trim()` không rỗng; đăng ký chuỗi khóa vào bộ lọc `redaction` trước khi lưu.  /  Lưu vào OS Keychain (`keyring`); nếu OS Keychain không khả dụng thì lưu vào `SessionKeyStore` trong RAM và trả `session_only_fallback` (§9.2).  /  `ValidationFailed`, `InternalError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| T-IPC-01 | disposable/synthetic fixture described by BE | (1) Giả lập AI gọi `confirm_pending_patch`, `commit_draft`, `set_scene_lock`; (2) Truyền `draft_session_id` chứa `../../etc` vào tool report; (3) Kích hoạt lỗi HTTP chứa `Bearer sk-secret1234567890123456` và kiểm tra log/IPC response. | (1) Bị chặn ngay bởi `UnauthorizedCaller`; (2) Bị chặn bởi kiểm tra kiểu `Uuid`; (3) Chuỗi khóa trong log và `IpcError` bị thay bằng `"[REDACTED]"`, không có khóa plaintext trong SQLite. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase2_sprint4_deep3-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test provider_security  # future target, after bootstrap`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] LlmProvider trait, adapters, credential/redaction APIs and config IPC #38–40

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase2_sprint4_deep3` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- LlmProvider trait, adapters, credential/redaction APIs and config IPC #38–40

- Exact artifacts: src-tauri/src/providers/traits.rs, src-tauri/src/providers/ollama.rs, src-tauri/src/providers/opencode.rs, src-tauri/src/providers/credentials.rs, src-tauri/src/providers/redaction.rs, src-tauri/src/ipc/config_commands.rs, src-tauri/tests/provider_security.rs; accepted integration SHA and `gate_phase2_sprint4_deep3` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CI-04, CI-10.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
