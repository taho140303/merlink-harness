-- The character-coverage addendum must name the real tool parameter; `source` is not accepted.
UPDATE prompt_blocks
SET content = replace(content,
  'tạo đủ từng nhân vật bằng `add_character` với `source = user`',
  'tạo đủ từng nhân vật bằng `add_character` và khai báo `declared_sources` cho từng trường (ví dụ {"display_name":"user","nickname":"user","biological_sex":"user","motif_code":"user"}); không truyền tham số `source`')
WHERE prompt_code = 'planning_phase_0';

-- Phase 6 must create missing characters instead of refusing to.
UPDATE prompt_blocks
SET content = replace(content,
  'Kiểm tra nhân vật và cảnh tham chiếu đã tồn tại trước khi ghi; không tự tạo nhân vật.',
  'Kiểm tra nhân vật và cảnh tham chiếu đã tồn tại trước khi ghi; nếu người dùng đã nêu tên một nhân vật chưa có trong require.character, gọi add_character trước rồi mới ghi quan hệ. Không tự đặt tên mới khi người dùng chưa xác nhận.')
WHERE prompt_code = 'planning_phase_6';
