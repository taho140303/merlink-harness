# SPRINT 17.5 — STAGE 2.3: SCENE BINDER

> **Trạng thái: DONE.** Branch `sprint-17.5-stage-23`, base `main` tại `ae3d8bf`.
> Kết quả: `cargo test` **221 passed / 0 failed / 3 ignored**; `cargo clippy --all-targets -- -D warnings`
> sạch; `cargo fmt` sạch; `pnpm build` pass; `node scripts/gen-catalog.mjs --check` sạch.

## KẾT QUẢ

| Hạng mục | Nội dung |
|---|---|
| Contract | `Lane::SceneBinder`: bắt buộc `position/code/name/summarize/world_setting/style_preset_code/narrator_code/target_words/directives/rules/cast`; đúng **một** `is_pov`; `style_preset_code` và `narrator_code` phải có trong catalogue; mọi `directives[].code` phải có trong catalogue, `weight` thuộc `NONE..ABSOLUTE`; `position` liên tục trong payload |
| Prompt | `prompts/outline_scenes.md` + `prompts::scenes_user` (anchor, chương, cast, địa điểm, danh sách code hợp lệ, ngân sách cảnh/từ) |
| Planner | 1 shard/chương (`scenes-ch-<n>`), 5 chương song song; `chapter` do planner gán vào từng proposal để commit không phải tin model |
| Apply | `commit_scenes`: merge theo `(chapter_id_json_index, position)`, sort, ghi `scenes` (status `PLANNED`, `content` null, directives/rules/relationships là JSON), rồi `scene_characters` với `scene_id_json_index` tính sau khi sort; `assert_order([Scenes, SceneCharacters])` |
| Board 2.3 | `ScenesBoard`: nhóm theo chương, mỗi cảnh hiện người kể, địa điểm, preset, narrator, directive chip, rule, độ dài; chọn phạm vi chương và nút sinh |
| Gate | Tab 2.3 mở khi **cast và world đã chốt**; rail dùng `status.stages` |
| Test | 5 test `test_stage_23_*`: lưu proposal theo cảnh, commit scenes + scene_characters (một POV mỗi cảnh, `content` null), từ chối preset lạ, từ chối directive lạ, chặn hai POV |

## QUYẾT ĐỊNH TRONG SPRINT

1. **`schema_rows` phải tìm cả hai group**: trước đó chỉ đọc `optional`, nên `characters` (bảng require)
   không được tìm thấy khi commit scene.
2. **`chapter` do planner gán**, không yêu cầu model trả: model chỉ biết chương qua prompt, còn khoá
   `(chapter, position)` phải chắc chắn.
3. **`scenes.content` luôn null** trong phase 2; văn bản ở phase 3 qua `content_path`.
4. **Directives lưu nguyên mảng JSON** `{code, weight, instruction}` — phase 3 đọc lại để chèn prompt.
5. **Chưa nối `memories.source_scene_id`**: proposal ký ức chưa có tham chiếu cảnh; ghi thành nợ kỹ
   thuật thay vì bịa liên kết.

## NỢ KỸ THUẬT ĐÃ BIẾT

- `memories.source_scene_id` vẫn null; cần một bước "link pass" khi cast proposal có trường `scene`.
- 2.3 chưa có test UI; kiểm bằng `pnpm build` + preview.
- Chưa có phong cách "một preset cho cả chương": binder chọn theo từng cảnh như thiết kế.

## NGOÀI PHẠM VI

Không có văn bản truyện, không có phase 3, không sửa ProjectDetail.
