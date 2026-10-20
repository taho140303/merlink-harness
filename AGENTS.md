# Agent memory — merlink-harness

This file is the project-scoped operating memory for coding agents. Read it before
planning or delegating work in this repository.

## 1. Project facts

- Stack: Tauri 2, React 19, TypeScript, Rust (edition 2024), Vite.
- Data root: `MERLINK_HOME` -> `HOME` -> `USERPROFILE` -> `HOMEDRIVE` +
  `HOMEPATH`; default directory is `.merlink`.
- Persistence: `config.json` (providers, 0600 on Unix), `merlink.db` (SQLite:
  projects/sessions/messages plus `outline_runs`/`outline_items`),
  `projects/<id>/super.json` (the committed project state) and
  `projects/<id>/content/<chapter>/<scene>.md` (phase 3 prose).
- Agent loop: model -> tools -> model, **maximum 5 tool rounds**.
- Super JSON: **81 writable paths** (13 project leaves + 34 tables × count/rows), **12 required / 35 optional units**, 70/30 progress. Schema is **v3** (phase-2 fields added, no new table).
- Draft refs: `_json_index` and `<field>_json_index` are synthetic row columns (not paths);
  `super_json_add_row`/`super_json_add_field` complete the chat-agent surface, which now has **8 tools** (`outline` is the 8th).
- Phase 2 (outline): `outline_runs`/`outline_items`, stage machine `seed → arcs → chapters → cast + world → scenes`, `STATE_ORDER` of 26 entities in `outline/stage.rs`. Subagents are backend fan-out calls, never tool loops; only `apply` writes, only accepted proposals, only in that order. `require.project.*` is read-only; required table rows are writable.
- Phase 3 (writing): one file per scene, `scenes.content` stays null, `content_path`/`word_count`/`status` live in state. Prose must contain no emoji and no en/em dash; `writing::violations` rejects, retries once, then sanitises.
- Catalogues: `docs/schema/catalog/*.json` are generated from `docs/schema/seed/*.sql` by `node scripts/gen-catalog.mjs`; `--check` must stay clean in review.
- IPC: Tauri lower-camel-cases Rust parameter names. JS sends `sessionId`,
  `projectId`, `providerId`; never snake_case. Register commands only through
  `merlink_handlers!`.
- Verification: `cargo test --manifest-path src-tauri/Cargo.toml`,
  `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`,
  `pnpm build` and `node scripts/gen-catalog.mjs --check`.
  Baseline after Sprint 17.7: **230 passed, 3 ignored**.
- No new dependency without explicit user approval.
- Do not commit `.agents/`, `dist/`, `node_modules/`, `src-tauri/target/`,
  secrets, or real `~/.merlink` data.

## 2. Git identity and workflow

- Repository owner/account: **taho140303**.
- Active `gh` account must be `taho140303` for pushes.
- Commits must use:
  `git -c user.name=taho140303 -c user.email="202986530+taho140303@users.noreply.github.com" commit ...`
- Branch per sprint, push the branch. Never commit directly to `main`, never
  force-push, never rewrite published history.
- Only the root/orchestrator merges a sprint branch into `main` after verifying
  `cargo test` and `pnpm build` again.

## 3. Model routing for subagents

- Default: `provider=openai-codex`, `model=gpt-6-sol`, `reasoning_effort=high`.
- If GPT is unavailable or out of quota: `provider=commandcode`,
  `model=deepseek/deepseek-v4.1-flash`, `reasoning_effort=high` (`max` only for
  unusually risky work).
- Record the route actually used in the sprint handoff.

## 4. Mandatory subagent call format

The orchestrator **must** call every sprint subagent with this structure. Fill
the bracketed parts; do not omit sections. Add the repo path and model route in
`CONTEXT`/`TASKS` if the child does not inherit them.

```text
═══════════════════════════════════════════════════════
SPRINT [N] — [TÊN]
═══════════════════════════════════════════════════════

CONTEXT:
[2-3 dòng mô tả state hiện tại]

GOAL:
[1 câu duy nhất, measurable]

TASKS:
[numbered list, mỗi task 1 action cụ thể]

ACCEPTANCE:
[checklist verify được, KHÔNG subjective]

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Phát hiện task cần design decision không có trong spec → DỪNG, hỏi
- Cần thêm dependency ngoài list đã cho → DỪNG, hỏi
- Task vượt quá 30 phút ước tính → DỪNG, chia nhỏ lại

───────────────────────────────────────────────────────
🔁 RETRY BUDGET (giới hạn số lần thử)
───────────────────────────────────────────────────────
- Mỗi task: tối đa 2 lần thử
- Mỗi lần thử: phải thay đổi approach, KHÔNG được lặp y hệt
- Sau lần 2 fail: DỪNG task đó, chuyển sang task khác
- Cuối sprint: list ra các task chưa xong + lý do

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS (khi nào hỏi user)
───────────────────────────────────────────────────────
DỪNG và hỏi user khi:
1. Lỗi không rõ nguyên nhân sau 2 lần debug
2. Cần chọn giữa 2 approach (A vs B) mà spec không nói
3. Cần thay đổi kiến trúc hiện tại (folder structure, store shape)
4. Phát hiện bug ở sprint trước ảnh hưởng sprint này
5. Cần user test thủ công (visual, UX, performance)

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- KHÔNG refactor code ngoài file list ở TASKS
- KHÔNG thêm feature ngoài GOAL
- KHÔNG đổi UI/UX ngoài spec
- KHÔNG optimize premature (chỉ làm khi ACCEPTANCE yêu cầu)
- KHÔNG viết test (trừ khi task yêu cầu rõ)
- KHÔNG update README/docs (trừ khi task yêu cầu)

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: [N]

✅ COMPLETED:
- [task 1]: [file changed]
- [task 2]: [file changed]

❌ FAILED (nếu có):
- [task]: [lỗi gì] | [đã thử gì] | [cần gì từ user]

🤔 NEED_INPUT (nếu có):
- Câu hỏi 1: [mô tả] | Option A: [...] | Option B: [...]
- Câu hỏi 2: ...

📋 NEXT ACTION:
- Nếu DONE → "Ready for sprint N+1, paste next prompt"
- Nếu BLOCKED → "Waiting for user response on X"
- Nếu PARTIAL → "Continue task Y after user confirms"

═══════════════════════════════════════════════════════
```

### Orchestrator checklist

1. Read the sprint spec and the current source before filling the template.
2. Fill `GOAL` with one measurable sentence; every `TASK` must name files or an
   observable action.
3. Make `ACCEPTANCE` executable (commands, counts, paths), never "looks good".
4. Put the exact file allow-list in `TASKS` and repeat the non-goals in
   `OUT OF SCOPE`.
5. Pass `provider`, `model`, and `reasoning_effort` explicitly.
6. After the handoff: verify the branch, rerun tests/build, merge with `--no-ff`,
   update the sprint status, and push `main`.

## 5. Refactor programme

- Phase 1 reviews/synthesis/sprints: `docs/refactor/`.
- Phase 2 module-decomposition plan and execution log:
  `docs/refactor/phase-2/README.md`. Phase 2 is complete; production modules are
  split, but `runtime/agent/tests.rs` is 1,505 test-only lines. Splitting test
  modules is a future task, not a behavior change.
- A module move is a refactor: **no behavior change**, and public call sites stay
  stable through `mod.rs` re-exports.
- Audit `include_str!` paths whenever a file containing one moves; relative paths
  are compile-time and depth-sensitive (`runtime/` and `super_json/` both have
  one).
- Move tests with the code they exercise; do not rewrite assertions.
