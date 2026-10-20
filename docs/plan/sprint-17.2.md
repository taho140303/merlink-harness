# SPRINT 17.2 — OUTLINE ENGINE CORE

> **Trạng thái: DONE.** Branch `sprint-17.2-outline-engine`, base `main` tại `4bcf453`.
> Kết quả: `cargo test` **204 passed / 0 failed / 3 ignored**; `cargo clippy --all-targets -- -D warnings`
> sạch; `cargo fmt --check` sạch; `pnpm build` pass; `node scripts/gen-catalog.mjs --check` sạch.
> Route dùng: engine `openai-codex / gpt-6-luna / high` (PARTIAL, orchestrator hoàn thiện);
> playbook + design `commandcode / deepseek-v4.1-flash / high` (DONE).

## KẾT QUẢ

| Hạng mục | Nội dung |
|---|---|
| SQLite v2 | `outline_runs`, `outline_items` (+ cột `shard`, index); migrate `1→2` và `0→2`, test `test_open_migrates_v1_database_to_v2` |
| `db/outline.rs` | CRUD run/item, `upsert_items` (ON CONFLICT theo `run_id+stage+key`), `delete_items_for_shard` |
| `outline/stage.rs` | 7 stage + gate (`scenes` cần cả `cast` và `world`), `STATE_ORDER` **26 entity**, `assert_order` |
| `outline/contract.rs` | `Lane` (arc/chapter có implementation), `parse`, `validate` (mảng, code unique, position liên tục **trong payload**), `validate_entity` |
| `outline/fanout.rs` | `JoinSet` + `Semaphore`, concurrency 1..8 (mặc định 5), retry ≤2 khi JSON hỏng, cancel, trả `(RunStats, Vec<ShardResult>)` |
| `outline/seed.rs` | resolve `genre_code` từ catalogue (khớp code/name, fallback substring, bỏ dấu), ghi qua `super_json::set` |
| `outline/apply.rs` | commit theo batch: validate hết trước khi ghi, map proposal → đúng cột của bảng, `arc` → `arc_id_json_index`, merge theo `position`, giữ `require.project.*` bất khả xâm phạm |
| `outline/mod.rs` | facade `start/status/run/items/regenerate/accept/reject/commit/cancel`, `run_with_backend` cho test, chia shard chương 10/chương |
| Tool `outline` | 8 action `status/start/anchor/run/regenerate/items/commit/cancel`; `run`/`regenerate` chạy nền nên không chặn lượt chat |
| IPC | 10 command (2 cũ + 8 mới), `CommandMap`, browser-mock, contract rows; mới nằm trong `RUST_ONLY_COMMANDS` |
| Prompt | `prompts/outline_playbook.md` chèn **một lần** khi đủ 12 unit (`runtime/prompt.rs`), test có/không |
| Docs | `docs/design/phase-2.md` (234 dòng): sơ đồ stage, bảng 3 tầng, 6 lane, STATE_ORDER, nơi lưu dữ liệu, lỗi, nợ kỹ thuật |

## QUYẾT ĐỊNH TRONG SPRINT

1. **`require` bất khả xâm phạm chỉ ở `require.project.*`.** Phase 2 phải mở rộng 6 bảng required
   (`arcs` chỉ có `name`/`position` từ phase 1), nên guard suy bảng từ `schema_table_group` chứ
   không hardcode; test `test_guard_allows_required_table_rows_but_never_project_leaves`.
2. **`outline_items` là đề xuất theo entity** (mỗi arc/chương một dòng) chứ không theo shard, để
   tác giả duyệt/viết lại từng mục; cột `shard` giữ manh mối để `regenerate` dựng lại đúng lô.
3. **`commit` nhận `keys` = sự phê duyệt.** Không có `keys` thì ghi item `accepted`; có `keys` thì
   chính danh sách đó là phê duyệt (chat nói "chốt arc-01" là đủ, không cần accept riêng).
4. **Chương ghi tăng dần.** `missing` liệt kê chương còn thiếu; chỉ khi đủ `plan.chapter.count` mới
   bắt buộc `Σ scene_count == plan.scene.count`. Nhờ vậy "sinh n chương theo yêu cầu" hoạt động.
5. **Tool không tự chạy fan-out đồng bộ**: `run`/`regenerate` spawn nền (không có kênh event), model
   đọc tiến độ bằng `status`; UI 17.3 gọi thẳng command để có event.
6. **Cắt API chưa dùng**: `catalog::{get,contains,count,codes}` và 4 lane chưa implementation được
   bỏ để clippy sạch; chúng trở lại ở 17.4/17.5 cùng consumer thật. `Lane::for_stage` thay cho
   match trùng lặp.
7. **`Entry.data` có `#[allow(dead_code)]`** kèm chú thích: 17.5 đọc trọng số/vocab/devices từ đó.

## NỢ KỸ THUẬT ĐÃ BIẾT

- Prompt cho `arc_smith`/`chapter_smith` còn tối giản (17.3 làm đầy, kèm UI board).
- `RUST_ONLY_COMMANDS` còn 8 command outline; 17.3 gỡ khi `Chat.tsx` gọi thật.
- `cast_smith`/`world_smith`/`bond_smith`/`scene_binder` chưa tồn tại (17.4/17.5).
- Tool `run` chạy nền không phát event; UI phải poll `status` hoặc gọi command.

## NGOÀI PHẠM VI (đúng kế hoạch)

Không có UI outline, không có cast/world/scene lane, không có văn bản truyện.
