-- Every character the author names must exist in require.character before any phase references
-- them. Phase 0 creates them; later phases must add missing ones instead of silently dropping.

UPDATE prompt_blocks
SET content = content || ' Bổ sung bắt buộc: nếu người dùng đã nêu tên bất kỳ nhân vật nào — kể cả nhân vật phụ, người thân, đồng nghiệp, phản diện — thì tạo đủ từng nhân vật bằng `add_character` với `source = user` ngay trong lượt; không chỉ giữ hai nhân vật chính. Với nhân vật chỉ được mô tả mà chưa có tên, đề xuất tên và chờ người dùng xác nhận trước khi ghi; không tự đặt tên. Mỗi nhân vật cần `display_name` hoặc `nickname`, `biological_sex`, `motif_code`; thiếu trường nào thì hỏi đúng trường đó. Liệt kê lại toàn bộ nhân vật đã ghi để người dùng đối chiếu.'
WHERE prompt_code = 'planning_phase_0';

UPDATE prompt_blocks
SET content = content || ' Bổ sung bắt buộc: kiểm tra mọi nhân vật được nhắc trong tóm tắt Arc/theme; nếu chưa có trong `require.character` thì gọi `add_character` trước khi dùng làm luận điểm hoặc tham chiếu. Không bỏ sót nhân vật nào đã được người dùng nêu tên.'
WHERE prompt_code IN ('planning_phase_1', 'planning_phase_2');

UPDATE prompt_blocks
SET content = content || ' Bổ sung bắt buộc: với mỗi cảnh, liệt kê đầy đủ nhân vật có mặt; nếu một nhân vật chưa tồn tại trong `require.character`, gọi `add_character` trước rồi mới gắn vào cảnh. Không bỏ qua nhân vật phụ.'
WHERE prompt_code IN ('planning_phase_4', 'planning_phase_6');
