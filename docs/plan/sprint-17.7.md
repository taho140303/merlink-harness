# SPRINT 17.7 — PHASE 3 UI + HARDENING

> **Trạng thái: DONE.** Branch `sprint-17.7-writing-ui`, base `main` tại `e1bd9bf`.
> Kết quả: `cargo test` **230 passed / 0 failed / 3 ignored**; `cargo clippy --all-targets -- -D warnings`
> sạch; `cargo fmt` sạch; `pnpm build` pass; `node scripts/gen-catalog.mjs --check` sạch.

## KẾT QUẢ

| Hạng mục | Nội dung |
|---|---|
| Tab dự án | `ProjectDetail` có hai tab `Tổng quan` và `Viết`; tab Viết giữ nguyên phần overview phía trên |
| Workspace viết | `components/write/ChapterWorkspace.tsx`: chọn provider/model, danh sách chương (n/ tổng cảnh), danh sách block cảnh của chương đang chọn, ghi chú dùng cho lần viết, nút "Viết lại n cảnh đã chọn" |
| Block cảnh | Mỗi block hiện trạng thái (`PLANNED/WRITTEN/REVISED/LOCKED`), số từ, `content_path`, tóm tắt cảnh; checkbox chọn nhiều cảnh; `Viết cảnh này` (Duyệt), `Khoá/Mở khoá` (Bỏ), `Viết lại` |
| Prompt guard | `runtime/prompt.rs`: tài liệu > 24 KB được nhúng dạng **rút gọn** (project leaves + `count`/`rows` mỗi bảng) kèm nhắc dùng `super_json_get` |
| Docs | `AGENTS.md` cập nhật: 8 tool, schema v3, phase 2/3 invariants, baseline 230 test; `docs/plan/sprint-17.md` đánh dấu hoàn tất |
| Test | `test_large_document_is_embedded_as_a_summary` (prompt rút gọn) |

## QUYẾT ĐỊNH TRONG SPRINT

1. **Không thêm command `project_outline`**: tab Viết đọc state bằng `super_json_read` sẵn có, tránh
   một bề mặt IPC trùng lặp.
2. **Provider/model chọn ngay trong tab Viết**: ProjectDetail chưa có ngữ cảnh provider, nên tab tự
   nạp `load_providers` và mặc định `default_model_id`.
3. **Ngưỡng 24 KB cho prompt**: đo trên chuỗi pretty-printed, vì đó chính là thứ được nhúng; vượt ngưỡng
   thì model buộc phải đọc chi tiết bằng `super_json_get`.
4. **Viết lại nhiều cảnh dùng chung một ghi chú** (đúng yêu cầu "kêu ai viết lại riêng 1 hoặc nhiều
   phần scene được chọn").

## NỢ KỸ THUẬT CÒN LẠI (sau roadmap 17)

- `memories.source_scene_id` vẫn null: cần link pass khi proposal ký ức có tham chiếu cảnh.
- `rewrite_selection` chưa phát event theo từng cảnh ra UI.
- UI chưa có test tự động (repo không có runner); kiểm bằng `pnpm build` + preview.
- `characters.motifs` là field tạm cho motif nhân vật; nếu port DBML thì chuyển sang bảng riêng.
