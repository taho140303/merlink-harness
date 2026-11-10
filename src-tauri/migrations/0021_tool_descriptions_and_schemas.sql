-- 0021: Mọi tool được cấp cho prompt phase phải có description.
--
-- `PromptComposer::get_tools_by_prompt` (chat_orchestrator.rs) loại bỏ bất kỳ tool nào có
-- description rỗng. Trước migration này chỉ 12/87 tool có description, nên dù
-- `prompt_block_tools` cấp 75 tool cho `planning_phase_0`, model chỉ nhận được 12 tool:
-- không có add_arc/add_theme/add_theme_argument/add_relationship_role/add_scene_config/get_state...
-- Hệ quả: model không đọc được dàn ý Arc, không biết schema của role/quan hệ và tự bịa summary.
--
-- Migration bổ sung description cho toàn bộ tool còn thiếu, đồng thời khai báo enum cho các
-- trường dễ sai (role, answer_mode, hierarchy, các chiều quan hệ, is_pov).

UPDATE tools SET description = CASE code
  -- Arc
  WHEN 'add_arc' THEN 'Thêm một Arc vào optional.overview_phase.arcs[]. Tham số: code?, display_name, summary, position, declared_sources.'
  WHEN 'set_arc' THEN 'Cập nhật Arc đã có theo code. Tham số: code, display_name?, summary?, position?, declared_sources.'
  WHEN 'delete_arc' THEN 'Xóa Arc theo code; cascade chương và cảnh thuộc Arc sau khi kiểm tra source.'
  WHEN 'set_arc_motif' THEN 'Gán motif xung đột cho Arc. Tham số: code, motif_code?, note?, declared_sources.'
  WHEN 'add_arc_event_block' THEN 'Thêm khối biến cố cho Arc. Tham số: arc_code, code?, event_block_code?, start_chapter?, end_chapter?, summary?, declared_sources.'
  WHEN 'delete_arc_event_block' THEN 'Xóa một khối biến cố của Arc theo code.'
  -- Theme
  WHEN 'add_theme' THEN 'Thêm chủ đề cho Arc. Tham số: arc_code, code?, parent_theme_code?, question?, resolution_type?, resolution?, message?, declared_sources.'
  WHEN 'set_theme' THEN 'Cập nhật chủ đề của Arc theo code. Tham số: code, question?, resolution?, message?, declared_sources.'
  WHEN 'delete_theme' THEN 'Xóa chủ đề của Arc theo code.'
  WHEN 'add_theme_argument' THEN 'Thêm lập trường nhân vật cho chủ đề. Tham số: theme_code, character_code?, role (PRIMARY|ANTITHESIS|CAUTIONARY|WITNESS), answer_mode (VERBAL|ACTION|OUTCOME|IMPLICIT), note?, declared_sources.'
  WHEN 'delete_theme_argument' THEN 'Xóa một lập trường của chủ đề theo code.'
  -- Chương, cảnh
  WHEN 'add_chapter' THEN 'Thêm chương vào Arc. Tham số: arc_code, code?, display_name, position, summary, declared_sources. position là thứ tự toàn truyện.'
  WHEN 'set_chapter' THEN 'Cập nhật chương theo code. Tham số: code, display_name?, position?, summary?, declared_sources.'
  WHEN 'delete_chapter' THEN 'Xóa chương theo code; cascade cảnh thuộc chương.'
  WHEN 'add_scene' THEN 'Thêm cảnh thô vào chương. Tham số: chapter_code, code?, display_name, position, summary, declared_sources.'
  WHEN 'set_scene' THEN 'Cập nhật cảnh theo code. Tham số: code, display_name?, position?, summary?, declared_sources.'
  WHEN 'delete_scene' THEN 'Xóa cảnh theo code; cascade các liên kết nhân vật và lore của cảnh.'
  WHEN 'set_scene_config' THEN 'Cấu hình kể chuyện cho cảnh. Tham số: scene_code, style_preset_code?, narrator_config_code?, world_setting_code?, declared_sources.'
  WHEN 'add_scene_character' THEN 'Gắn nhân vật vào cảnh. Tham số: scene_code, character_code, is_pov?, declared_sources.'
  WHEN 'delete_scene_character' THEN 'Gỡ nhân vật khỏi cảnh. Tham số: scene_code, character_code, declared_source.'
  WHEN 'add_scene_lore_entry' THEN 'Gắn lore vào cảnh. Tham số: scene_code, lore_entry_code, usage?, reveal_to_reader?, reveal_to_pov?, declared_sources.'
  WHEN 'set_scene_lore_entry' THEN 'Cập nhật liên kết lore của cảnh (usage, mức tiết lộ).'
  WHEN 'delete_scene_lore_entry' THEN 'Gỡ liên kết lore khỏi cảnh. Tham số: scene_code, lore_entry_code, declared_source.'
  -- Bối cảnh, lore
  WHEN 'add_world_setting' THEN 'Thêm nút cây bối cảnh. Tham số: code?, parent_code?, display_name, type (REALM|WORLD|REGION|COUNTRY|CITY|TOWN|FACILITY|STREET|ROOM), description?, access?, declared_sources.'
  WHEN 'set_world_setting' THEN 'Cập nhật nút bối cảnh theo code. Tham số: code, display_name?, type?, description?, access?, parent_code?, declared_sources.'
  WHEN 'delete_world_setting' THEN 'Xóa nút bối cảnh; bị chặn nếu còn cảnh tham chiếu (referenced_entity_in_use).'
  WHEN 'add_lore_entry' THEN 'Thêm mục tri thức gắn bối cảnh. Tham số: code?, world_setting_code, display_name, type (CULTURE|HISTORY|FACTION|ITEM), description?, data?, declared_sources.'
  WHEN 'set_lore_entry' THEN 'Cập nhật mục tri thức theo code. Tham số: code, display_name?, type?, description?, data?, declared_sources.'
  WHEN 'delete_lore_entry' THEN 'Xóa mục tri thức; cascade liên kết scene_lore_entries sau khi kiểm tra source.'
  -- Nhân vật
  WHEN 'add_character' THEN 'Thêm nhân vật vào require.character[] và gắn dự án. Tham số: display_name?, nickname?, biological_sex?, motif_code?, project_role?, project_notes?, declared_sources.'
  WHEN 'set_character' THEN 'Cập nhật nhân vật theo code. Tham số: code, display_name?, nickname?, species?, birth?, gender_identity?, gender_expression?, declared_sources.'
  WHEN 'delete_character' THEN 'Xóa nhân vật; bị chặn nếu còn quan hệ tham chiếu (referenced_entity_in_use).'
  WHEN 'set_project_character' THEN 'Cập nhật liên kết nhân vật với dự án (vai trò, ghi chú).'
  WHEN 'add_character_motif' THEN 'Gán motif cho nhân vật. Tham số: character_code, motif_code, declared_source.'
  WHEN 'delete_character_motif' THEN 'Gỡ motif khỏi nhân vật. Tham số: character_code, motif_code, declared_source.'
  WHEN 'add_character_appearance' THEN 'Thêm hồ sơ ngoại hình (5.1). Tham số: character_code, species?, height?, weight?, hair_color_code?, skin_tone_code?, outfit_style?, default_summary?, declared_sources.'
  WHEN 'set_character_appearance' THEN 'Cập nhật hồ sơ ngoại hình theo character_code.'
  WHEN 'add_character_distinguishing_mark' THEN 'Thêm dấu hiệu nhận dạng. Tham số: character_code, location?, content, declared_source.'
  WHEN 'delete_character_distinguishing_mark' THEN 'Xóa dấu hiệu nhận dạng của nhân vật.'
  WHEN 'add_character_psychological' THEN 'Thêm hồ sơ tâm lý (5.2): the_wound, the_lie, fundamental_fear, moral_alignment, external_want, internal_need, internal_conflict, social_mask, defense_mechanisms; declared_sources.'
  WHEN 'set_character_psychological' THEN 'Cập nhật hồ sơ tâm lý theo character_code.'
  WHEN 'add_character_memory' THEN 'Thêm ký ức nhân vật (5.3). Tham số: character_code, code?, display_name?, summary?, source_scene_code?, declared_sources.'
  WHEN 'set_character_memory' THEN 'Cập nhật ký ức theo code.'
  WHEN 'delete_character_memory' THEN 'Xóa ký ức của nhân vật.'
  WHEN 'add_character_competence' THEN 'Thêm năng lực nhân vật (5.3). Tham số: character_code, code?, domain_code?, level_code?, scene_code (bắt buộc), capabilities?, limitations?, declared_sources.'
  WHEN 'set_character_competence' THEN 'Cập nhật năng lực theo code.'
  WHEN 'delete_character_competence' THEN 'Xóa năng lực của nhân vật.'
  WHEN 'add_competence_capability' THEN 'Thêm khả năng cho một năng lực.'
  WHEN 'delete_competence_capability' THEN 'Xóa khả năng của năng lực.'
  WHEN 'add_competence_limitation' THEN 'Thêm giới hạn cho một năng lực.'
  WHEN 'delete_competence_limitation' THEN 'Xóa giới hạn của năng lực.'
  -- Quan hệ
  WHEN 'add_relationship' THEN 'Tạo quan hệ có hướng. Tham số: code?, from_code, to_code, motif_code?, declared_sources.'
  WHEN 'set_relationship' THEN 'Cập nhật quan hệ theo code.'
  WHEN 'delete_relationship' THEN 'Xóa quan hệ theo code.'
  WHEN 'add_relationship_role' THEN 'Thêm vai trò xã hội cho quan hệ. Tham số: relationship_code, category_code?, hierarchy (SUPERIOR|EQUAL|INFERIOR), is_primary?, declared_sources.'
  WHEN 'delete_relationship_role' THEN 'Xóa vai trò của quan hệ.'
  WHEN 'set_relationship_state' THEN 'Ghi trạng thái quan hệ tại cảnh. Tham số: relationship_code, scene_code, trust?, affection?, hostility?, fear?, respect?, familiarity?, dependency? (NONE|LOW|MODERATE|HIGH|ABSOLUTE), declared_sources.'
  -- Đọc, tra cứu
  WHEN 'get_state' THEN 'Đọc State bản nháp theo thực thể hoặc toàn bộ; trả kèm source và patch đang chờ. Tham số: target_type?, code?, fields?.'
  WHEN 'get_tool_definition' THEN 'Đọc mô tả và input_schema của một tool đã được cấp. Tham số: tool_name.'
  WHEN 'get_field_null' THEN 'Trả tối đa 3 trường còn null kèm câu hỏi cố định. Tham số: limit?.'
  WHEN 'get_array_element_fields' THEN 'Trả trường còn thiếu của phần tử mảng hoặc trường cần tạo phần tử mới. Tham số: array_path, code?, limit?.'
  WHEN 'lookup_outline' THEN 'Tra cứu mục lục chương/cảnh theo trang. Tham số: cursor?, limit?.'
  WHEN 'search_character' THEN 'Tìm nhân vật theo tên hoặc biệt danh. Tham số: query, limit?.'
  WHEN 'search_genre' THEN 'Tra cứu thể loại theo mã hoặc tên hiển thị trong danh mục; dùng mã trả về khi gắn thể loại.'
  WHEN 'add_project_genre' THEN 'Gắn thể loại đã xác minh vào dự án; genre_code phải là mã có thật, declared_sources.genre_code khai báo nguồn.'
  WHEN 'delete_project_genre' THEN 'Gỡ thể loại khỏi dự án. Tham số: genre_code, declared_source.'
  WHEN 'search_motif' THEN 'Tra cứu mô-típ theo mã, tên hoặc nhóm; dùng mã đã tra cứu khi gán mô-típ.'
  WHEN 'search_event_block' THEN 'Tra cứu khối biến cố theo mã hoặc tên.'
  WHEN 'search_style_preset' THEN 'Tra cứu phong cách diễn đạt theo mã hoặc tên.'
  WHEN 'search_narrator_config' THEN 'Tra cứu cấu hình người kể chuyện theo mã hoặc tên.'
  WHEN 'search_appearance_option' THEN 'Tra cứu lựa chọn ngoại hình (màu tóc, màu da, mắt) theo nhóm.'
  WHEN 'search_relationship_category' THEN 'Tra cứu nhóm quan hệ theo mã hoặc tên.'
  WHEN 'search_competence_domain' THEN 'Tra cứu lĩnh vực năng lực theo mã hoặc tên.'
  WHEN 'add_project' THEN 'Thêm dự án mới vào bản nháp. Tham số: display_name?, summary?, language?, target?, declared_sources.'
  WHEN 'set_project' THEN 'Cập nhật thông tin dự án. Tham số: display_name?, summary?, language?, target?, declared_sources.'
  ELSE NULL
END
WHERE trim(coalesce(description, '')) = ''
  AND code IN (
    'add_arc','set_arc','delete_arc','set_arc_motif','add_arc_event_block','delete_arc_event_block',
    'add_theme','set_theme','delete_theme','add_theme_argument','delete_theme_argument',
    'add_chapter','set_chapter','delete_chapter','add_scene','set_scene','delete_scene',
    'set_scene_config','add_scene_character','delete_scene_character','add_scene_lore_entry',
    'set_scene_lore_entry','delete_scene_lore_entry','add_world_setting','set_world_setting',
    'delete_world_setting','add_lore_entry','set_lore_entry','delete_lore_entry',
    'add_character','set_character','delete_character','set_project_character','add_character_motif',
    'delete_character_motif','add_character_appearance','set_character_appearance',
    'add_character_distinguishing_mark','delete_character_distinguishing_mark',
    'add_character_psychological','set_character_psychological','add_character_memory',
    'set_character_memory','delete_character_memory','add_character_competence',
    'set_character_competence','delete_character_competence','add_competence_capability',
    'delete_competence_capability','add_competence_limitation','delete_competence_limitation',
    'add_relationship','set_relationship','delete_relationship','add_relationship_role',
    'delete_relationship_role','set_relationship_state','get_state','get_tool_definition',
    'get_field_null','get_array_element_fields','lookup_outline','search_character','search_genre',
    'add_project_genre','delete_project_genre','search_motif','search_event_block',
    'search_style_preset','search_narrator_config','search_appearance_option',
    'search_relationship_category','search_competence_domain','add_project','set_project'
  );

-- Mọi tool còn lại (nếu có) nhận mô tả tối thiểu để không bị bộ lọc loại bỏ.
UPDATE tools
SET description = 'Tool ' || code || ': tham số theo tài liệu phase và docs/db.dbml; '
               || 'khai báo declared_sources (hoặc declared_source với tool xóa) cho mọi trường ghi.'
WHERE trim(coalesce(description, '')) = '';

-- Enum tường minh cho các trường hay sai (schema vẫn permissive để backend tự validate).
UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.role', json('{"type":"string","enum":["PRIMARY","ANTITHESIS","CAUTIONARY","WITNESS"],"description":"Vai trò lập trường của nhân vật trong chủ đề"}'),
  '$.properties.answer_mode', json('{"type":"string","enum":["VERBAL","ACTION","OUTCOME","IMPLICIT"],"description":"Cách nhân vật trả lời chủ đề"}'),
  '$.properties.theme_code', json('{"type":"string","minLength":1}'),
  '$.properties.character_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'add_theme_argument' AND json_valid(input_schema);

UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.hierarchy', json('{"type":"string","enum":["SUPERIOR","EQUAL","INFERIOR"],"description":"Thứ bậc quyền lực của vai trò"}'),
  '$.properties.is_primary', json('{"type":"boolean"}'),
  '$.properties.relationship_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'add_relationship_role' AND json_valid(input_schema);

UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.trust', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.affection', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.hostility', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.fear', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.respect', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.familiarity', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.dependency', json('{"type":"string","enum":["NONE","LOW","MODERATE","HIGH","ABSOLUTE"]}'),
  '$.properties.relationship_code', json('{"type":"string","minLength":1}'),
  '$.properties.scene_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'set_relationship_state' AND json_valid(input_schema);

UPDATE tools SET input_schema = json_set(
  input_schema,
  '$.properties.is_pov', json('{"type":"boolean","description":"Nhân vật giữ góc nhìn; ngôi kể giới hạn cần đúng một is_pov"}'),
  '$.properties.scene_code', json('{"type":"string","minLength":1}'),
  '$.properties.character_code', json('{"type":"string","minLength":1}')
)
WHERE code = 'add_scene_character' AND json_valid(input_schema);

PRAGMA user_version = 21;
