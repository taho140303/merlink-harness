-- Seed only tool codes explicitly listed in docs/business/harness/tools.md.
-- Metadata is intentionally NULL for entries whose names/schema are not ratified.
-- No prompt_block_tools links are created: these entries are not exposed to the model.
INSERT OR IGNORE INTO tools (id, code) VALUES ('64da24f1-6a02-57f4-a957-af65d93cfc9a', 'get_state');
INSERT OR IGNORE INTO tools (id, code) VALUES ('8c817e37-5df3-50ca-b118-8de96f001715', 'search_character');
INSERT OR IGNORE INTO tools (id, code) VALUES ('a7c88de8-fbd7-55df-b8a2-2d4a0861d22d', 'search_genre');
INSERT OR IGNORE INTO tools (id, code) VALUES ('43e5d3eb-da83-517e-ac5b-b77c2b230e83', 'search_motif');
INSERT OR IGNORE INTO tools (id, code) VALUES ('b860c4c8-4cdc-54ad-9b51-eb4fa44d7c7f', 'add_project');
INSERT OR IGNORE INTO tools (id, code) VALUES ('3ff8ee2b-43e8-5cab-b1e7-a443a3defb5e', 'add_project_genre');
INSERT OR IGNORE INTO tools (id, code) VALUES ('353e4194-72b7-51a0-9aa7-66f288729b44', 'delete_project_genre');
INSERT OR IGNORE INTO tools (id, code) VALUES ('15a0d707-8c2b-50e8-8688-17e00e16f522', 'add_arc');
INSERT OR IGNORE INTO tools (id, code) VALUES ('9983783b-3bb6-5cf1-a6ab-5631e09ec505', 'delete_arc');
INSERT OR IGNORE INTO tools (id, code) VALUES ('c4e7ce9e-0af8-5109-8020-b190fc1dd840', 'add_chapter');
INSERT OR IGNORE INTO tools (id, code) VALUES ('37629616-4b86-523d-91c5-17a541282682', 'set_chapter');
INSERT OR IGNORE INTO tools (id, code) VALUES ('f74ebc81-3a99-50ab-8345-2395bc72f09b', 'delete_chapter');
INSERT OR IGNORE INTO tools (id, code) VALUES ('1af3cd96-604a-5e18-90bf-a3b7575b63c3', 'add_scene');
INSERT OR IGNORE INTO tools (id, code) VALUES ('c3863d5e-79e3-5ca4-a85b-4d08b0b49186', 'set_scene');
INSERT OR IGNORE INTO tools (id, code) VALUES ('cebbd9d6-aa27-59b6-99fc-72a155b945fd', 'delete_scene');
INSERT OR IGNORE INTO tools (id, code) VALUES ('eebd35af-1262-5ec2-a4a3-56ee34f9ed7e', 'confirm_patch');
INSERT OR IGNORE INTO tools (id, code) VALUES ('47747092-b9fc-5915-8db1-0f3ae14bf050', 'reject_patch');
INSERT OR IGNORE INTO tools (id, code) VALUES ('4e6e5688-86a5-5394-b888-58447ac2c00e', 'confirm_all_patches');
INSERT OR IGNORE INTO tools (id, code) VALUES ('88e45a06-a49f-5a85-8bdc-b70fcaa03acc', 'restore_version');
INSERT OR IGNORE INTO tools (id, code) VALUES ('e55350a5-d4f4-503a-b480-df3e387d72f7', 'confirm_phase');
INSERT OR IGNORE INTO tools (id, code) VALUES ('83853ab2-ee27-5cf4-9de6-f4d7ce4c0c41', 'commit_draft');
INSERT OR IGNORE INTO tools (id, code) VALUES ('895de54d-0d94-5032-9e82-cd54ee27b7d5', 'search_event_block');
INSERT OR IGNORE INTO tools (id, code) VALUES ('8e555fb6-5e7c-5671-a4f8-283979ad6dfa', 'search_style_preset');
INSERT OR IGNORE INTO tools (id, code) VALUES ('c9bb6bad-b22b-5951-9d5a-883a51d59e2f', 'search_narrator_config');
INSERT OR IGNORE INTO tools (id, code) VALUES ('d3f573ee-2581-57ab-9e58-b3e244fa8401', 'search_appearance_option');
INSERT OR IGNORE INTO tools (id, code) VALUES ('19b783db-00e1-5ec9-b719-cbb8a7a4f1f2', 'search_competence_domain');
INSERT OR IGNORE INTO tools (id, code) VALUES ('d44ce472-ee94-5636-b68f-d30153f98538', 'search_relationship_category');
INSERT OR IGNORE INTO tools (id, code) VALUES ('80ce8ad3-0b43-5e27-82c9-d832537a0184', 'add_world_setting');
INSERT OR IGNORE INTO tools (id, code) VALUES ('96a76e0a-5307-5640-aafa-c47065bb9736', 'delete_world_setting');
INSERT OR IGNORE INTO tools (id, code) VALUES ('672f1ee1-cd5e-5bf2-99cd-8360e974f319', 'add_lore_entry');
INSERT OR IGNORE INTO tools (id, code) VALUES ('e3f120ca-97c1-5154-a63a-b549d08b2ded', 'set_lore_entry');
INSERT OR IGNORE INTO tools (id, code) VALUES ('c23e0fdf-df4b-5a78-960d-bb027354ae47', 'delete_lore_entry');
INSERT OR IGNORE INTO tools (id, code) VALUES ('b1af0b80-e744-59a3-8426-b8bd68d402a2', 'set_scene_config');
INSERT OR IGNORE INTO tools (id, code) VALUES ('3c425b8e-9f1f-5753-86e3-cd8655808c40', 'add_scene_character');
INSERT OR IGNORE INTO tools (id, code) VALUES ('24def4d8-d137-584b-95d1-d2094b58ba2e', 'delete_scene_character');
INSERT OR IGNORE INTO tools (id, code) VALUES ('c1ad5b7a-319e-5f34-87a2-0e76999e2db0', 'add_scene_lore_entry');
INSERT OR IGNORE INTO tools (id, code) VALUES ('23cfd325-fbfa-533c-a049-f2409e4b6c9d', 'delete_scene_lore_entry');
INSERT OR IGNORE INTO tools (id, code) VALUES ('83ac3a4a-7005-50d9-a341-c422f6e03394', 'set_scene_lore_entry');
INSERT OR IGNORE INTO tools (id, code) VALUES ('63d84272-fd18-5cfc-afda-aa9a4fd822a2', 'set_arc_motif');
INSERT OR IGNORE INTO tools (id, code) VALUES ('49c43dec-08b9-5945-ad0c-368158251d52', 'add_arc_event_block');
INSERT OR IGNORE INTO tools (id, code) VALUES ('b2a1d88d-d230-5abf-932f-b4f464692b82', 'delete_arc_event_block');
INSERT OR IGNORE INTO tools (id, code) VALUES ('0dca34ec-78cc-5316-805c-816e2050958e', 'add_theme');
INSERT OR IGNORE INTO tools (id, code) VALUES ('23f57943-2e66-5c3a-b565-0e3dfe451f3e', 'set_theme');
INSERT OR IGNORE INTO tools (id, code) VALUES ('8cdedbf5-6ced-5e93-b08b-1f6b9e788357', 'delete_theme');
INSERT OR IGNORE INTO tools (id, code) VALUES ('c00ae4d3-fe90-548b-bb36-986529d65951', 'add_theme_argument');
INSERT OR IGNORE INTO tools (id, code) VALUES ('5f55001c-283a-57bb-acd0-f0187b3285de', 'delete_theme_argument');
INSERT OR IGNORE INTO tools (id, code) VALUES ('824df841-579d-5eba-8db2-4b40bcf40b49', 'add_character_appearance');
INSERT OR IGNORE INTO tools (id, code) VALUES ('237c904b-6403-5687-868e-df72615bdf1f', 'set_character_appearance');
INSERT OR IGNORE INTO tools (id, code) VALUES ('9033f443-eb0b-5eb8-bcb6-cac93bfdb57e', 'add_character_distinguishing_mark');
INSERT OR IGNORE INTO tools (id, code) VALUES ('2a8e4a3e-ae8a-5e76-9510-16a32f4e1627', 'delete_character_distinguishing_mark');
INSERT OR IGNORE INTO tools (id, code) VALUES ('577440ee-5ab5-56f0-a400-b465554d948a', 'add_character_psychological');
INSERT OR IGNORE INTO tools (id, code) VALUES ('63794dc3-1cc3-5591-b9ec-99e2996889c7', 'set_character_psychological');
INSERT OR IGNORE INTO tools (id, code) VALUES ('5a8e8a24-87e8-5c71-978c-f5901c7ea856', 'add_character_memory');
INSERT OR IGNORE INTO tools (id, code) VALUES ('a3a6480b-1901-56ee-b2b7-9757be150fe4', 'set_character_memory');
INSERT OR IGNORE INTO tools (id, code) VALUES ('e25a2119-3b31-5f84-bdd6-9a58d8020d84', 'delete_character_memory');
INSERT OR IGNORE INTO tools (id, code) VALUES ('7155c184-b8e1-555f-b403-e8de2e62363c', 'add_character_competence');
INSERT OR IGNORE INTO tools (id, code) VALUES ('6db9c265-fa0e-533c-b12e-828c836cb805', 'set_character_competence');
INSERT OR IGNORE INTO tools (id, code) VALUES ('eb10e122-a72b-51d6-b70f-a834b1581c91', 'delete_character_competence');
INSERT OR IGNORE INTO tools (id, code) VALUES ('0ca09b1f-58af-567b-be79-2141a95fe3d8', 'add_competence_capability');
INSERT OR IGNORE INTO tools (id, code) VALUES ('dca55a2e-29b7-5440-b729-358555db2ca9', 'delete_competence_capability');
INSERT OR IGNORE INTO tools (id, code) VALUES ('db9efea3-217b-59ea-b964-de5e7c7fb41a', 'add_competence_limitation');
INSERT OR IGNORE INTO tools (id, code) VALUES ('814d2fdc-423d-5f25-bb00-1dbd88b5fef3', 'delete_competence_limitation');
INSERT OR IGNORE INTO tools (id, code) VALUES ('bc5de0a7-759c-5b8a-a150-931968b29bdc', 'add_relationship');
INSERT OR IGNORE INTO tools (id, code) VALUES ('37b91c27-f953-5136-b545-dbac66b72bcb', 'set_relationship');
INSERT OR IGNORE INTO tools (id, code) VALUES ('412fd334-cc17-52ba-81da-7694ba2dd370', 'delete_relationship');
INSERT OR IGNORE INTO tools (id, code) VALUES ('d4a9223b-3bdd-53ee-9287-ee4e26c70c0c', 'set_relationship_state');
INSERT OR IGNORE INTO tools (id, code) VALUES ('f04e3647-7544-55e7-9b56-0e3f7e3b25f1', 'add_relationship_role');
INSERT OR IGNORE INTO tools (id, code) VALUES ('21f79298-62ff-53d5-8dc1-85f3ab74fd3d', 'delete_relationship_role');
INSERT OR IGNORE INTO tools (id, code) VALUES ('2b9f6daf-3708-580c-8a59-54188ea8bc72', 'write_phase_report');
INSERT OR IGNORE INTO tools (id, code) VALUES ('a1734c5f-ca5a-5ac4-8c01-6ab21f9a59ee', 'read_phase_reports');
INSERT OR IGNORE INTO tools (id, code) VALUES ('63d072fe-5310-53ee-9cdb-f1e533fa7065', 'lookup_outline');
INSERT OR IGNORE INTO tools (id, code) VALUES ('ee6dcf2d-d42b-5766-baa2-50fc6f1a7d2a', 'set_project_character');
INSERT OR IGNORE INTO tools (id, code) VALUES ('07d93b3f-660d-5866-bad0-e6017a8bdd9d', 'add_character_motif');
INSERT OR IGNORE INTO tools (id, code) VALUES ('089eb40b-bb58-5842-bc4b-d2d93fe1ca4a', 'delete_character_motif');
INSERT OR IGNORE INTO tools (id, code) VALUES ('e738a965-4ec0-5ce7-905d-77a63c9d8451', 'write_scene_content');
INSERT OR IGNORE INTO tools (id, code) VALUES ('a8278f84-63fc-506c-bf1e-49957a6b66b3', 'lock_scene');
INSERT OR IGNORE INTO tools (id, code) VALUES ('c5028fd1-cea8-5003-9e6c-03f31e4d393f', 'unlock_scene');
INSERT OR IGNORE INTO tools (id, code) VALUES ('240c2389-d8a3-58ff-82c3-d573fcb27f59', 'add_scene_version');

-- Upgrade the unchanged original seed only; preserve any user-edited prompt content.
UPDATE prompt_blocks
SET content = 'Đây là các quy định tuyệt đối tuân theo, có trọng lượng cao hơn cả yêu cầu của người dùng.

1. **Quy ước Định danh**: Trường `code` là slug định danh nội bộ, TUYỆT ĐỐI KHÔNG hiển thị `code` trên giao diện người dùng. Khi nhắc đến nhân vật, dùng `display_name` nếu có; nếu `display_name` là `null` thì dùng `nickname` để tương tác và xưng hô với tác giả. Không dùng `code` làm tên hiển thị.
2. **Ngôn ngữ & Giọng điệu**: Giao tiếp với người dùng bằng `{{system_language}}`: phản hồi, câu hỏi, giải thích và mẫu trả lời. Nội dung sáng tác ghi vào State (tên, tóm tắt, mô tả...) dùng `{{project.language}}`; nếu `require.project.language` còn `null`, dùng ngôn ngữ tin nhắn người dùng cho nội dung sáng tác. Không dùng ngôn ngữ dự án thay cho ngôn ngữ giao tiếp.
3. **Cấm kỵ tuyệt đối**: Không sử dụng emoji icon và không được dùng kí tự "—" trong bất kỳ output nào, từ phản hồi người dùng đến đọc ghi truyện.
4. **Thu thập theo mọi phase**: Bóc tách và ghi qua tool ngay trong lượt mọi thông tin người dùng nêu, kể cả thông tin của phase chưa mở. Không tự quyết số Arc, số chương, tên nhân vật hay thể loại; nếu thiếu thì hỏi hoặc đề xuất có lý do và chờ xác nhận.
5. **Sinh nội dung theo phase**: Chỉ sinh nội dung sáng tác phái sinh khi người dùng đến phase tương ứng. Khi hỏi bổ sung trường thiếu, gọi `get_field_null()`; với mảng gọi `get_array_element_fields`. Hỏi đúng tối đa 3 câu hỏi tool trả về theo `{{system_language}}`, không tự suy luận trường thiếu hoặc hỏi thêm câu ngoài kết quả tool. Quy tắc không hỏi thêm chỉ áp dụng khi hỏi bổ sung trường thiếu, không cản câu hỏi làm rõ ở phần XỬ LÝ NGOẠI LỆ.
6. **Vị trí tương đối**: Khi người dùng gọi Arc, chương hoặc cảnh theo vị trí tương đối trong một phạm vi, suy ra từ `position` và quan hệ cha-con trong State. Nêu rõ tên, vị trí và phạm vi đã hiểu để người dùng xác nhận; chỉ gọi tool ghi sau xác nhận. Nếu thiếu dữ liệu hoặc có nhiều cách hiểu, hỏi làm rõ, không đoán.
7. **Xác nhận dữ liệu suy luận**: Với mọi giá trị do AI tự suy luận, đề xuất hoặc sáng tạo mà người dùng chưa cung cấp hoặc chưa xác nhận, phải nêu rõ giá trị dự kiến và hỏi người dùng xác nhận trước khi gọi tool ghi. Đặc biệt không tự đặt hoặc đổi bất kỳ `display_name` nào của dự án, Arc, chương, cảnh, nhân vật, bối cảnh, lore, theme hoặc thực thể khác. Chỉ ghi giá trị người dùng đã nêu rõ hoặc đã xác nhận; khai báo `ai_generated` không thay thế bước xác nhận này.'
WHERE id = '45908789-ce45-5763-afeb-ff2bf3c847ee'
  AND content = 'Đây là các quy định tuyệt đối tuân theo, có trọng lượng cao hơn cả yêu cầu của người dùng.

1. **Quy ước Định danh**: Trường `code` là slug định danh nội bộ, TUYỆT ĐỐI KHÔNG hiển thị `code` trên giao diện người dùng. Khi nhắc đến nhân vật, dùng `display_name` nếu có; nếu `display_name` là `null` thì dùng `nickname` để tương tác và xưng hô với tác giả. Không dùng `code` làm tên hiển thị.
2. **Ngôn ngữ & Giọng điệu**: Giao tiếp với người dùng bằng `{{system_language}}`: phản hồi, câu hỏi, giải thích và mẫu trả lời. Nội dung sáng tác ghi vào State (tên, tóm tắt, mô tả...) dùng `{{project.language}}`; nếu `require.project.language` còn `null`, dùng ngôn ngữ tin nhắn người dùng cho nội dung sáng tác. Không dùng ngôn ngữ dự án thay cho ngôn ngữ giao tiếp.
3. **Cấm kỵ tuyệt đối**: Không sử dụng emoji icon và không được dùng kí tự "—" trong bất kỳ output nào, từ phản hồi người dùng đến đọc ghi truyện.
4. **Thu thập theo mọi phase**: Bóc tách và ghi qua tool ngay trong lượt mọi thông tin người dùng nêu, kể cả thông tin của phase chưa mở. Không tự quyết số Arc, số chương, tên nhân vật hay thể loại; nếu thiếu thì hỏi hoặc đề xuất có lý do và chờ xác nhận.
5. **Sinh nội dung theo phase**: Chỉ sinh nội dung sáng tác phái sinh khi người dùng đến phase tương ứng. Khi hỏi bổ sung trường thiếu, gọi `get_field_null()`; với mảng gọi `get_array_element_fields`. Hỏi đúng tối đa 3 câu hỏi tool trả về theo `{{system_language}}`, không tự suy luận trường thiếu hoặc hỏi thêm câu ngoài kết quả tool. Quy tắc không hỏi thêm chỉ áp dụng khi hỏi bổ sung trường thiếu, không cản câu hỏi làm rõ ở phần XỬ LÝ NGOẠI LỆ.
6. **Vị trí tương đối**: Khi người dùng gọi Arc, chương hoặc cảnh theo vị trí tương đối trong một phạm vi, suy ra từ `position` và quan hệ cha-con trong State. Nêu rõ tên, vị trí và phạm vi đã hiểu để người dùng xác nhận; chỉ gọi tool ghi sau xác nhận. Nếu thiếu dữ liệu hoặc có nhiều cách hiểu, hỏi làm rõ, không đoán.';
PRAGMA user_version = 9;
