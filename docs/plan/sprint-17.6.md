# SPRINT 17.6 — PHASE 3 WRITING ENGINE

> **Trạng thái: DONE.** Branch `sprint-17.6-writing-engine`, base `main` tại `a205021`.
> Kết quả: `cargo test` **229 passed / 0 failed / 3 ignored**; `cargo clippy --all-targets -- -D warnings`
> sạch; `cargo fmt` sạch; `pnpm build` pass; `node scripts/gen-catalog.mjs --check` sạch.

## KẾT QUẢ

| Hạng mục | Nội dung |
|---|---|
| Vị trí file | `projects/<id>/content/chapter-<ccc>/scene-<ppp>.md`; bản cũ vào `.../versions/scene-<ppp>-v<n>.md` trước khi ghi đè |
| Bundle | `writing::bundle` dựng prompt từ state: chương (tên, tóm tắt, beats), cảnh (tên, tóm tắt, ngân sách từ), cast + người kể, địa điểm + 5 giác quan, style preset (nhịp, cảm xúc, thủ pháp, từ vựng), narrator, directive + weight + instruction, rule riêng, 2 cảnh trước (600 ký tự đầu) |
| Guardrail | `violations()` phát hiện emoji và `– — ‒ ―`; vi phạm lần 1 → retry kèm mô tả vi phạm; vi phạm lần 2 → `sanitize()` (dấu gạch ngang thành `, `, bỏ emoji) và `clean = false` |
| Ghi state | `content_path`, `word_count`, `status` (`WRITTEN`) qua `super_json::add_field`; `scenes.content` **vẫn null** |
| Báo cáo | `WriteReport {scene, path, word_count, within_budget, clean, violations}`; `within_budget` = lệch ≤ 15% ngân sách từ |
| Khoá | `status = LOCKED` chặn `write_scene` cho tới khi mở khoá (`REVISED`) |
| Command | `writing_status`, `write_scene`, `rewrite_scenes`, `lock_scene` + `CommandMap` + `src/lib/writing.ts` + browser mock + contract rows |
| Test | 8 test `writing::*`: phát hiện vi phạm, sanitize, đường dẫn, tạo file + cập nhật state, retry khi có emoji/`–`, sanitize lần hai, giữ version cũ, cảnh khoá không bị viết lại |

## QUYẾT ĐỊNH TRONG SPRINT

1. **Mỗi cảnh một file** thay vì nhét prose vào `super.json`: ghi song song an toàn, `content` giữ null,
   và state vẫn nhỏ để nhồi prompt.
2. **`clean` = "model có tuân thủ"**, không phải "file cuối sạch": nếu phải sanitize thì `clean=false`
   kèm danh sách vi phạm, dù văn bản đã gửi đi là sạch.
3. **`within_budget` là cờ mềm** (lệch 15%), không chặn ghi: retry vì độ dài sẽ tốn token mà chất lượng
   không chắc tăng.
4. **`rewrite_selection` chạy tuần tự trong một chương** để mạch văn liền, dù backend có thể song song.
5. **Archive trước khi ghi**: nếu ghi file thất bại thì bản cũ vẫn còn.

## NỢ KỸ THUẬT ĐÃ BIẾT

- Chưa có UI phase 3 (17.7): viết từng cảnh, chọn nhiều cảnh để viết lại, khoá/mở khoá.
- `rewrite_selection` chưa phát `OutlineEvent` cho từng cảnh (dùng channel nội bộ rỗng).
- `memories.source_scene_id` vẫn null (nợ từ 17.5).

## NGOÀI PHẠM VI

Không có tab Viết trong ProjectDetail, không sửa UI nào khác.
