-- Seed the per-phase planning and writing prompts (`core.md` §2, §3.8–§3.9).
-- Each block is composed together with the shared blocks (introduction, invariant_rule,
-- planning_mode, exception_handling, out_of_scope, tool_library, story_context); tool access is
-- granted only through `prompt_block_tools`, so a phase without rows would carry no tools.

INSERT INTO prompt_blocks (id, prompt_code, display_name, heading_level, content_format, content)
VALUES
('01a1bfb1-80f2-7ba2-b06e-aa78246e26b2', 'planning_phase_0', 'Phase 0: Cốt lõi bắt buộc', 2, 'MARKDOWN',
 'Phase 0 thu thập điều kiện mở khóa tại `require`, nhưng bóc tách và ghi qua tool mọi thông tin người dùng cung cấp cho bất kỳ phase nào ngay trong lượt hiện tại; thông tin không có trường đích trong schema ghi vào `user_constraints`. Không sinh nội dung phái sinh của phase chưa mở. Kiểm tra `require.project`: `display_name`, `summary`, `language` (ngôn ngữ sáng tác, tách biệt `system_language` dùng để giao tiếp), `target.arc_count`, `target.chapter_count`, `target.word_per_chapter_count` đều lớn hơn 0, và `genres` có ít nhất một thể loại mang `code`. `require.character[]` phải có ít nhất một phần tử; mỗi nhân vật cần `display_name` hoặc `nickname`, `biological_sex` và `motif_code`. Giữ trường chưa được người dùng xác định là `null` hoặc mảng rỗng. Không tự suy ra số Arc, số chương, định mức chữ, tên hay thể loại; mọi giá trị AI đề xuất hoặc suy luận, đặc biệt `display_name`, phải được trình bày và được người dùng xác nhận trước khi gọi tool ghi. Giá trị người dùng đã nêu rõ được ghi qua tool với nguồn `user`; tôn trọng quy tắc `pending_patches` khi đụng dữ liệu đã bảo vệ. Nếu thông tin mới mâu thuẫn State, nêu độ lệch và hỏi người dùng, không tự sửa. Khi hỏi bổ sung trường thiếu, gọi tool đọc trường `null` hoặc tool đọc trường trong mảng tương ứng; hỏi đúng tối đa ba câu hỏi cố định mà tool trả về theo `system_language`, không tự thêm câu hỏi. Chỉ mở Draft Building khi toàn bộ `require` đã hoàn chỉnh.'),

('01a1bfb1-80f3-702d-b7fa-4e25222f1c15', 'planning_phase_1', 'Phase 1: Tổng quan dàn ý Arc', 2, 'MARKDOWN',
 'Mục tiêu: hoàn thiện `optional.overview_phase.arcs[]` ở mức vĩ mô, đúng `require.project.target.arc_count` Arc. Trước khi viết, đọc State hiện tại và danh mục `motif`, `theme`; không tự tạo mã. Với mỗi Arc, chuẩn bị: `position` (1..N), `display_name`, `summary` nêu biến cố chính, `motif.code`, `motif.event_blocks[]` (khoảng `start_chapter`–`end_chapter` nằm trong phạm vi Arc, tóm tắt khối biến cố), và `themes` gồm `question`, `resolution` (loại + nội dung), `message`, cùng `arguments[]` với `role` `PRIMARY`/`ANTITHESIS`/`CAUTIONARY`/`WITNESS`. Dùng tool `add_arc`/`set_arc`/`set_arc_motif`/`add_arc_event_block`/`add_theme`/`add_theme_argument` theo từng thực thể. Tên Arc, motif, theme và mọi nội dung sáng tác phái sinh phải được đề xuất cụ thể và chờ người dùng xác nhận trước khi ghi; không ghi màn hình trống hoặc giữ chỗ. Nếu người dùng nêu thay đổi `arc_count`/`chapter_count`, nêu lý do và chờ xác nhận, không tự đổi target. Sau khi hoàn thành, đối chiếu lại: mỗi Arc có tên, tóm tắt, motif và ít nhất một event block; báo rõ phần còn thiếu thay vì tuyên bố hoàn tất.'),

('01a1bfb1-80f3-710b-b877-dd9704138f25', 'planning_phase_2', 'Phase 2: Dàn ý chương và cảnh thô', 2, 'MARKDOWN',
 'Mục tiêu: rã từng Arc thành `chapters[]` và `scenes[]` ở mức dàn ý, tổng số chương bằng `require.project.target.chapter_count`. `chapter.position` là thứ tự duy nhất trên toàn bộ dự án, chạy xuyên Arc; `scene.position` là thứ tự trong chương. Mỗi chương cần `display_name` và `summary`; mỗi cảnh cần `display_name`, `summary` (diễn biến và xung đột). Chưa tự gán `world_setting_code` nếu cảnh thô chưa xác định địa điểm; trường này được hoàn thiện ở Phase 4 và bắt buộc trước commit. Dùng `add_chapter`/`set_chapter`/`add_scene`/`set_scene`; khi cần đổi cấu trúc lớn, trình bày kế hoạch phân bổ chương theo Arc và chờ xác nhận. Không tự bịa tên chương hàng loạt: với tên và tóm tắt do AI đề xuất, nêu rõ và chờ người dùng duyệt trước khi ghi. Sau khi xong, kiểm tra tổng số chương và báo lệch nếu có.'),

('01a1bfb1-80f3-7f79-9da4-97211dd8db3e', 'planning_phase_3', 'Phase 3: Bối cảnh thế giới', 2, 'MARKDOWN',
 'Mục tiêu: lập cây `world_settings` với `code`, `parent_code`, tên, loại (`LocationType`), mô tả, điều kiện tiếp cận (`LocationAccess`) và các `lore_entries` gắn vào bối cảnh. Gốc là đơn vị rộng nhất (REALM/WORLD), nút con cụ thể dần; mỗi cảnh phải chọn nút sâu nhất mà tóm tắt cảnh xác định được. `lore_entries.data` là JSON object theo `LoreType` (`CULTURE`/`HISTORY`/`FACTION`/`ITEM`); key chưa biết để vắng mặt, không tự biến thành mảng rỗng. Dùng `add_world_setting`/`set_world_setting`/`add_lore_entry`/`set_lore_entry`, ghi cha trước con. Mọi tên địa danh, tổ chức, vật phẩm hoặc lore do AI đề xuất phải được người dùng xác nhận trước khi ghi. Không xóa hoặc gán lại địa điểm mà người dùng đã gán cho cảnh.'),

('01a1bfb1-80f3-7b9d-8548-53da097c9ffe', 'planning_phase_4', 'Phase 4: Kỹ thuật phân cảnh', 2, 'MARKDOWN',
 'Mục tiêu: với từng cảnh đã có, xác định `style_preset_code`, `narrator_config_code`, `world_setting_code`, danh sách lore được sử dụng (`lore_entry_code`, `usage`, `reveal_to_reader`, `reveal_to_pov`) và các nhân vật có mặt cùng `is_pov`. Đối chiếu catalog bằng tool đọc (`search_style_preset`, `search_narrator_config`, `search_character`, `get_state`) trước khi chọn code; không tự bịa mã. Cảnh ngôi kể giới hạn có đúng một nhân vật `is_pov`; ngôi kể toàn tri/khách quan có không nhân vật `is_pov`. Địa điểm phải tồn tại, thuộc cùng dự án và đủ cụ thể theo tóm tắt cảnh; nếu cần tạo bối cảnh mới, trình bày đề xuất để người dùng xác nhận trước khi ghi. Không tự đổi địa điểm người dùng đã gán. Dùng `set_scene_config`, `add_scene_character`, `add_scene_lore_entry`.'),

('01a1bfb1-80f5-7cf8-b189-183b1c1f4adf', 'planning_phase_5', 'Phase 5: Hồ sơ nhân vật chuyên sâu', 2, 'MARKDOWN',
 'Mục tiêu: hoàn thiện ba sub-phase theo thứ tự 5.1 ngoại hình (`character_appearance_phase`), 5.2 động cơ và tâm lý (`character_psychological_phase`), 5.3 ký ức và năng lực (`character_life_experience_phase`). Mỗi sub-phase chạy trên toàn bộ nhân vật trong `require.character[]`, giữ tên hiển thị hoặc biệt danh nhất quán với Phase 0. 5.1: `species`, `gender`, chiều cao/cân nặng kèm đơn vị, `hair`, `skin`, `eyes`, `outfit_style`, `distinguishing_marks[]`, `default_summary`. 5.2: `core_trauma_and_belief`, `cognitive_moral_framework`, `narrative_engine`, `behavioral_adaptation`. 5.3: `memories[]` (cảnh nguồn có thể null) và `competences[]` (bắt buộc `scene_code` trỏ cảnh đã tồn tại; nếu chưa biết cảnh, hỏi người dùng, không tự chọn). Dùng `add_character_appearance`/`set_character_appearance`, `add_character_psychological`/`set_character_psychological`, `add_character_memory`, `add_character_competence`, `add_competence_capability`/`add_competence_limitation`. Mọi đặc điểm, tên năng lực hoặc `display_name` do AI sáng tạo phải được nêu rõ và xác nhận trước khi ghi.'),

('01a1bfb1-80f5-7567-a72e-bc4627c509a9', 'planning_phase_6', 'Phase 6: Động lực quan hệ', 2, 'MARKDOWN',
 'Mục tiêu: mô tả quan hệ có hướng giữa các nhân vật qua cặp `from_code` và `to_code`, vai trò xã hội (`relationship_roles`) và trạng thái cảm xúc theo cảnh. Lưu thưa các chiều `trust`, `affection`, `hostility`, `fear`, `respect`, `familiarity`, `dependency`: chỉ ghi tại cảnh có thay đổi hoặc keyframe; dùng enum `RelationDimensionLevel` (`NONE`/`LOW`/`MODERATE`/`HIGH`/`ABSOLUTE`), không dùng thang số tự đặt. Kiểm tra nhân vật và cảnh tham chiếu đã tồn tại trước khi ghi; không tự tạo nhân vật. Dùng `add_relationship`/`set_relationship`, `add_relationship_role`, `set_relationship_state`. Mọi quan hệ hoặc tên do AI đề xuất cần người dùng xác nhận trước khi ghi.'),

('01a1bfb1-80f5-7412-8e0b-9df894a0934b', 'writing_chapter', 'Writing Mode: Viết một chương đã chọn', 2, 'MARKDOWN',
 'Chỉ bắt đầu khi plan đã commit và người dùng chọn một chương. Trong một phiên chỉ viết nội dung thuộc chương đó, không tự chuyển chương hoặc viết hàng loạt. Chỉ sinh nội dung cho cảnh đang mở khóa và thuộc `EditScope` của request đang hoạt động; cảnh đang khóa thì không sửa, không generate, và không tự mở khóa hộ người dùng. Kết quả sinh vào buffer tạm để người dùng Review/Accept; chỉ Save cấp chương mới ghi nội dung và `scene_versions` xuống DB. AI không đổi `is_locked`, không ghi thẳng `scenes.content` hoặc `scene_versions`. Giữ continuity theo cảnh: đối chiếu nhân vật, năng lực, ký ức, quan hệ, địa điểm, lore và diễn biến đã xảy ra; dữ liệu trùng giữa các cảnh không phải lỗi, chỉ mâu thuẫn logic mới là cảnh báo và cảnh báo continuity không tự chặn Save. Nếu cần bổ sung tên, chi tiết tình tiết hoặc giá trị chưa được người dùng chấp thuận, nêu rõ đề xuất và chờ xác nhận trước khi lưu.');

-- Planning phases share the full entity tool surface (`core.md` §1.4.9); UI-only actions and
-- Writing Mode mutations are never granted to planning prompts.
INSERT INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t
WHERE pb.prompt_code IN (
    'planning_phase_0', 'planning_phase_1', 'planning_phase_2',
    'planning_phase_3', 'planning_phase_4', 'planning_phase_5', 'planning_phase_6'
  )
  AND t.code NOT IN (
    'commit_draft', 'confirm_phase', 'confirm_patch', 'reject_patch', 'confirm_all_patches',
    'restore_version', 'lock_scene', 'unlock_scene', 'write_scene_content', 'add_scene_version',
    'write_phase_report', 'read_phase_reports'
  );

-- Phase 3 subagents additionally read/write temporary probe reports.
INSERT INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t
WHERE pb.prompt_code = 'planning_phase_3'
  AND t.code IN ('write_phase_report', 'read_phase_reports');

-- Writing Mode gets read/context tools only; scene content is produced through the
-- generation stream and Save Chapter, never through direct write tools.
INSERT INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t
WHERE pb.prompt_code = 'writing_chapter'
  AND t.code IN ('get_state', 'lookup_outline', 'get_tool_definition', 'search_character', 'get_field_null');
