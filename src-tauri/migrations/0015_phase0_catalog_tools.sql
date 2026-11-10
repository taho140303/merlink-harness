-- Expose Phase 0 reference catalogs and project genre linking to the chat model.
-- Only tools with a valid object schema and a prompt_block_tools link are composed.
UPDATE tools SET
  display_name = 'search_genre',
  description = 'Tra cứu thể loại theo mã hoặc tên hiển thị trong danh mục. Dùng mã code trả về khi gắn thể loại vào dự án.',
  input_schema = '{"type":"object","properties":{"query":{"type":"string","description":"Từ khóa mã hoặc tên thể loại; bỏ qua để liệt kê."},"limit":{"type":"integer","minimum":1,"maximum":50}},"additionalProperties":false}'
WHERE code = 'search_genre';

UPDATE tools SET
  display_name = 'search_motif',
  description = 'Tra cứu mô-típ theo mã hoặc tên hiển thị, có thể lọc theo nhóm. Chỉ dùng mã đã tra cứu khi gán mô-típ nhân vật.',
  input_schema = '{"type":"object","properties":{"query":{"type":"string","description":"Từ khóa mã hoặc tên mô-típ; bỏ qua để liệt kê."},"category":{"type":"string","enum":["character_arc","relationship","symbolic","plot"]},"limit":{"type":"integer","minimum":1,"maximum":50}},"additionalProperties":false}'
WHERE code = 'search_motif';

UPDATE tools SET
  display_name = 'add_project_genre',
  description = 'Gắn thể loại đã xác minh từ search_genre vào dự án; genre_code phải là mã danh mục có thật. declared_sources.genre_code xác định nguồn của liên kết.',
  input_schema = '{"type":"object","properties":{"genre_code":{"type":"string","minLength":1},"is_primary":{"type":"boolean"},"declared_sources":{"type":"object","properties":{"genre_code":{"type":"string","enum":["user","ai_generated"]}},"required":["genre_code"],"additionalProperties":false}},"required":["genre_code","declared_sources"],"additionalProperties":false}'
WHERE code = 'add_project_genre';

INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT '1b5d6e21-7f90-4e86-8c31-9df51c17b001', p.id, t.id
FROM prompt_blocks p JOIN tools t ON t.code = 'search_genre'
WHERE p.prompt_code = 'tool_library';
INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT '1b5d6e21-7f90-4e86-8c31-9df51c17b002', p.id, t.id
FROM prompt_blocks p JOIN tools t ON t.code = 'search_motif'
WHERE p.prompt_code = 'tool_library';
INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT '1b5d6e21-7f90-4e86-8c31-9df51c17b003', p.id, t.id
FROM prompt_blocks p JOIN tools t ON t.code = 'add_project_genre'
WHERE p.prompt_code = 'tool_library';

-- Preserve existing schema properties and only expand the array-path choices.
UPDATE tools SET input_schema = json_set(input_schema,
  '$.properties.array_path.enum', json('["require.character","require.project.genres"]'))
WHERE code = 'get_array_element_fields' AND json_valid(input_schema);

-- Optional target tolerances are nonnegative integers, including zero.
UPDATE tools SET input_schema = json_set(input_schema,
  '$.properties.target.properties.arc_tolerance', json('{"type":"integer","minimum":0,"maximum":4294967295}'),
  '$.properties.target.properties.chapter_tolerance', json('{"type":"integer","minimum":0,"maximum":4294967295}'),
  '$.properties.target.properties.word_per_chapter_tolerance', json('{"type":"integer","minimum":0,"maximum":4294967295}'))
WHERE code = 'set_project' AND json_valid(input_schema);

PRAGMA user_version = 15;
