-- 0025: the loaded business state JSON is the SSOT; verify every write against it.
--
-- The model called add_project_genre, received a rejection, never asked again, and continued as
-- if genres were saved because it trusted its own tool call rather than the state JSON. This rule
-- applies to chat and workflow prompts (the introduction block is shared by both).

UPDATE prompt_blocks
SET content = content || char(10) || char(10) ||
  'Toàn bộ JSON trạng thái nghiệp vụ (business_flow_data/WorkingState) được nạp trong context là nguồn sự thật duy nhất (SSOT). Không được giả định một lần gọi tool là đã ghi thành công: sau mỗi lần ghi, đọc kết quả tool (status/error) và đối chiếu lại state mới nhất. Nếu một trường bắt buộc vẫn null hoặc một mảng bắt buộc vẫn rỗng trong SSOT (ví dụ get_field_null trả empty_arrays/array_questions), phải hỏi lại tác giả hoặc sửa và gọi lại tool trong cùng lượt; không được tuyên bố đã lưu xong rồi chuyển bước.'
WHERE prompt_code = 'introduction';

UPDATE prompt_blocks
SET content = content || char(10) || char(10) ||
  'Nếu kết quả tool là rejected, đọc kỹ trường lỗi, sửa tham số theo schema rồi thử lại; không bỏ qua lỗi và không coi dữ liệu đã được lưu. Chỉ khi SSOT đã có dữ liệu hợp lệ mới được nói với tác giả là đã lưu.'
WHERE prompt_code = 'planning_phase_0';

PRAGMA user_version = 25;
