-- =============================================================================
-- Migration 0014: Seed System Reference Catalogs
-- Authority: docs/temp_seed/system/genres.md, motif.md, narrator_config.md, narrative_directive.md
-- Target PRAGMA user_version: 14
-- Contains:
--   1. 50 System Genres & Categories (docs/temp_seed/system/genres.md)
--   2. 19 System Motifs & Structural Components/Events/Transitions/Rules/Constraints (docs/temp_seed/system/motif.md)
--   3. 64 Narrator Configurations (docs/temp_seed/system/narrator_config.md)
--   4. 73 Narrative Directives (docs/temp_seed/system/narrative_directive.md)
-- =============================================================================

-- -----------------------------------------------------------------------------
-- 1. System Genres
-- -----------------------------------------------------------------------------
-- Danh mục tag độc lập; mô tả xác định phạm vi của từng tag.
INSERT INTO genres (id, code, display_name, description) VALUES
('01a1c069-c9f1-7042-9dce-c80bdecf6301', 'xianxia', 'Tiên Hiệp', 'Tiểu thuyết tu chân lấy cảm hứng từ Đạo giáo, Phật giáo và thần thoại Á Đông, tập trung vào hành trình tu luyện nghịch thiên cầu trường sinh, luyện đan luyện khí, độ kiếp phi thăng và lĩnh ngộ thiên địa đại đạo.'),
('01a1c069-c9ee-75e0-9f59-0c4f0c2ff330', 'xuanhuan', 'Huyền Huyễn', 'Tiểu thuyết kỳ ảo phương Đông xây dựng trên nền tảng siêu hình học, tu luyện đấu khí/hồn lực, huyết mạch thần thú và đa tầng vũ trụ, kết hợp tự do các yếu tố thần thoại.'),
('01a1c069-c9f0-7d92-8bab-b56f5f114d46', 'wuxia', 'Kiếm hiệp', 'Tiểu thuyết võ thuật giang hồ và võ lâm Trung Quốc, Hàn Quốc (Murim), xoay quanh môn phái, bí kíp, ân oán chính tà và tinh thần hiệp nghĩa.'),
('01a1c069-c9ff-7ec0-a358-cc9d23d849d7', 'gaowu', 'Cao Võ', 'Thể loại giao thoa nơi võ đạo phát triển đến mức phi phàm dời non lấp bể, phá toái hư không, thường đặt trong bối cảnh toàn dân luyện võ, đô thị tương lai hoặc tinh tế.'),
('01a1c069-c9f2-730d-bc48-eceefe66f172', 'urban_dushi', 'Đô Thị', 'Tag bối cảnh thành thị với đời sống, thiết chế và quan hệ xã hội đô thị; có thể kết hợp với thời đại, kỳ ảo, dị năng hoặc lãng mạn.'),
('01a1c069-c9f4-7bed-b5a1-87aef9bd1895', 'military_junshi', 'Quân Sự', 'Tiểu thuyết chiến tranh, quân đội, chiến dịch quân sự, đặc chủng binh vương và chiến thuật sa trường với tinh thần đồng đội và lòng trung kiên.'),
('01a1c069-c9f9-7d6a-acd6-1247dc538e03', 'supernatural_lingyi', 'Linh Dị', 'Tiểu thuyết huyền bí tâm linh, trừ ma diệt quỷ, phong thủy bí thuật, trinh thám tâm linh và các quy tắc thế giới quỷ dị siêu nhiên.'),
('01a1c069-ca0e-7dd5-ace6-bd991c3f32c0', 'fan_fiction', 'Đồng Nhân', 'Tác phẩm phóng tác viết dựa trên thế giới quan, bối cảnh và tuyến nhân vật sẵn có từ các bộ anime, manga, game, tiểu thuyết hoặc phim ảnh nổi tiếng.'),
('01a1c069-ca56-7d71-a789-f499936dd3b6', 'mat_the', 'Mạt Thế', 'Tiểu thuyết sinh tồn khi xã hội sụp đổ trước đại dịch zombie, biến dị sinh học, thiên tai diệt vong hoặc dị tộc xâm lăng.'),
('01a1c069-ca57-7d54-805b-9344074fe045', 'he_thong', 'Hệ Thống', 'Tiểu thuyết nơi nhân vật sở hữu bàn tay vàng là giao diện thông minh giao nhiệm vụ, tích lũy điểm thưởng, quay thưởng gacha và thăng cấp chỉ số thần tốc.'),
('01a1c069-ca54-7c75-989b-2be9f3172c2b', 'xuyen_khong', 'Xuyên không', 'Thể loại linh hồn hoặc thể xác vượt qua không gian, thời gian sang thế giới khác, thời đại khác hoặc nhập vào nhân vật trong sách.'),
('01a1c069-ca3e-7b16-952b-71bebd7f0e78', 'trong_sinh', 'Trọng Sinh', 'Nhân vật sau khi trải qua bi kịch hoặc cái chết được quay ngược thời gian về quá khứ với toàn bộ ký ức để sửa chữa sai lầm và thay đổi số phận.'),
('01a1c069-ca52-7c5d-af48-07f1de9b3320', 'di_gioi', 'Dị Giới', 'Tiểu thuyết diễn ra tại một thế giới hoàn toàn khác biệt với Trái Đất, nơi tồn tại các quy luật ma thuật, đấu khí và các chủng tộc kỳ ảo.'),
('01a1c069-ca53-771d-b479-3ec33bdf45f6', 'di_nang', 'Dị Năng', 'Tiểu thuyết về cá nhân thức tỉnh siêu năng lực, học cách kiểm soát sức mạnh và đối mặt xung đột xã hội; bao gồm các câu chuyện siêu anh hùng.'),
('01a1c069-ca00-7604-bd37-fa2f5519bd4a', 'danmei', 'Đam Mỹ', 'Tiểu thuyết tình cảm lãng mạn giữa nam và nam (Boys Love / BL) trải rộng từ tiên hiệp, cổ trang, hiện đại cho đến vô hạn lưu và tinh tế.'),
('01a1c069-ca06-7e62-9924-3ebebb88d095', 'baihe', 'Bách Hợp', 'Tiểu thuyết tình cảm lãng mạn giữa nữ và nữ (Girls Love / GL) trong các bối cảnh kỳ ảo, cổ đại hoặc đời thường.'),
('01a1c069-ca04-79b4-a317-f936df64615d', 'vo_han_luu', 'Vô Hạn Lưu', 'Thể loại nơi nhân vật bị kéo vào chiều không gian kỳ bí, liên tục vượt qua các phó bản sinh tồn, giải mã quy tắc tử thần để đổi lấy tích điểm sống còn.'),
('01a1c069-ca30-7a30-8d6c-2d7ef19d392f', 'dien_van', 'Điền Văn & Trồng Trọt', 'Tiểu thuyết nhịp điệu êm đềm tập trung vào đời sống gia đình, làm nông nghiệp, phát triển sản nghiệp, kinh tế gia đình từ hai bàn tay trắng.'),
('01a1c069-ca3d-70c5-926b-58ceffc8ba41', 'hunter_dungeon', 'Hầm Ngục, Thợ Săn', 'Tiểu thuyết về thợ săn thức tỉnh, cổng không gian, chinh phục hầm ngục hoặc tháp thử thách; có thể khai thác chòm sao bảo hộ hay nhân vật điều khiển lõi hầm ngục.'),
('01a1c069-ca1a-7b61-b866-02a46cd502e6', 'shounen', 'Shounen', 'Tác phẩm hướng tới thanh thiếu niên, nhấn mạnh tinh thần đồng đội nỗ lực, tình bạn bền chặt, phiêu lưu vượt thử thách và chiến đấu hết mình.'),
('01a1c069-ca19-7d2d-8f44-a9a115544043', 'seinen', 'Seinen', 'Tác phẩm hướng tới độc giả nam trưởng thành, cốt truyện phức tạp, chiều sâu tâm lý, chính trị hoặc yếu tố hành động chân thực tàn khốc hơn.'),
('01a1c069-ca1b-7d70-9fc7-1b76343f77bb', 'shoujo', 'Shoujo', 'Tác phẩm hướng tới độc giả nữ trẻ tuổi, tập trung vào cảm xúc nội tâm, nét vẽ mộng mơ, rung động tình đầu và các mối quan hệ học đường.'),
('01a1c069-ca1c-71b9-9e57-d8bd67cabf33', 'josei', 'Josei', 'Tác phẩm hướng tới độc giả nữ trưởng thành, miêu tả các vấn đề tình yêu, hôn nhân, gia đình và sự nghiệp dưới góc nhìn chín chắn, thực tế.'),
('01a1c069-ca20-7b16-98fe-9958f05a604f', 'mecha', 'Mecha', 'Thể loại khoa học viễn tưởng xoay quanh các cỗ máy chiến đấu khổng lồ do con người điều khiển trong chiến tranh liên sao.'),
('01a1c069-ca21-7543-bc1f-29d9eb707f76', 'slice_of_life', 'Đời thường', 'Thể loại khắc họa chân thực, nhẹ nhàng những khoảnh khắc đời thường của nhân vật, không tập trung vào xung đột kịch tính lớn.'),
('01a1c069-ca22-7c62-bd6b-d1d96ee55c18', 'harem', 'Harem', 'Thể loại nơi một nhân vật chính được theo đuổi, vây quanh và yêu mến bởi nhiều nhân vật khác giới.'),
('01a1c069-ca11-7da3-8dba-eb84a7f898f3', 'litrpg', 'LitRPG', 'Tiểu thuyết tích hợp chỉ số, cấp độ, nhiệm vụ và cơ chế game vào thế giới truyện, bao gồm võng du, trò chơi trực tuyến thực tế ảo và game hóa hiện thực.'),
('01a1c069-ca5b-7fc3-952d-4e64724cd3f4', 'science_fiction', 'Khoa Học Viễn Tưởng', 'Tiểu thuyết suy đoán về khoa học, công nghệ, trí tuệ nhân tạo và tương lai nhân loại; bao gồm khoa huyễn Á Đông và công nghệ giả tưởng thời đại hơi nước.'),
('01a1c069-ca78-7af1-9c8f-5bf3b55740a8', 'space_opera', 'Space Opera', 'Tiểu thuyết viễn tưởng hoành tráng lấy bối cảnh chiến tranh giữa các đế chế thiên hà, phi thuyền chiến hạm và phiêu lưu đa hành tinh.'),
('01a1c069-ca73-7753-823e-c47097179aa1', 'cyberpunk', 'Cyberpunk', 'Khoa học viễn tưởng tương lai phản địa đàng, nơi công nghệ AI, mạng kết nối và cấy ghép sinh học thống trị bên cạnh sự suy đồi xã hội.'),
('01a1c069-ca5c-75e9-b380-a9824f778356', 'dystopian', 'Phản Địa Đàng', 'Tiểu thuyết lấy bối cảnh xã hội tương lai bị kiểm soát toàn trị, áp bức khắc nghiệt nơi cá nhân vùng lên đòi lại tự do.'),
('01a1c069-ca71-7bce-9c55-9d57b1bebacd', 'alternate_history', 'Alternate History', 'Tiểu thuyết suy đoán điều gì sẽ xảy ra nếu một biến cố lịch sử quan trọng trong quá khứ diễn biến theo chiều hướng ngược lại.'),
('01a1c069-ca5e-70fe-a423-674439619334', 'mystery_detective', 'Trinh thám', 'Tiểu thuyết xoay quanh quá trình điều tra phá án, truy lùng hung thủ của thám tử chuyên nghiệp hoặc nghiệp dư.'),
('01a1c069-ca39-7f7b-b4bf-02552e9a6c2b', 'psychological', 'Tâm Lý', 'Tiểu thuyết đi sâu vào các ngóc ngách tinh thần, sự thao túng tâm lý, chấn thương tiềm thức và nhân cách con người.'),
('01a1c069-ca5f-7739-94a2-ce2531c64875', 'horror', 'Horror', 'Tiểu thuyết khơi gợi cảm giác sợ hãi nguyên thủy trước quái vật, thế lực tà ác siêu nhiên hoặc sự điên loạn tâm thần.'),
('01a1c069-ca65-7144-af38-f78b9746ff74', 'contemporary_fiction', 'Contemporary Fiction', 'Tác phẩm hư cấu phản ánh trực tiếp đời sống, mâu thuẫn xã hội và tâm tư con người trong thế giới hôm nay.'),
('01a1c069-ca66-712f-8229-8c805ff7429f', 'literary_fiction', 'Literary Fiction', 'Tiểu thuyết chú trọng phong cách ngôn từ nghệ thuật, chiều sâu tư tưởng và nhân văn hơn các công thức giải trí.'),
('01a1c069-ca0a-7005-b755-89e4d6b45d16', 'magical_realism', 'Magical Realism (Hiện Thực Huyền Ảo)', 'Tiểu thuyết khắc họa hiện thực đời sống nhưng đan cài các hiện tượng kỳ ảo như một phần tự nhiên của thế giới.'),
('01a1c069-c9f7-752a-9b1e-acd95edd8d84', 'sports', 'Thể Thao Chuyên Nghiệp', 'Tiểu thuyết về đấu trường thể thao, tinh thần rèn luyện và vinh quang đỉnh cao của vận động viên.'),
('019a6f00-0000-7000-8000-000000000001', 'comedy', 'Hài Hước', 'Tác phẩm lấy tiếng cười làm trọng tâm qua tình huống trớ trêu, đối thoại dí dỏm, châm biếm hoặc phi lý; có thể kết hợp với các thể loại khác.'),
('019a6f01-0000-7000-8000-000000000001', 'romance', 'Lãng Mạn', 'Tác phẩm lấy sự hình thành và phát triển tình yêu làm tuyến chính, độc lập với thời đại và bối cảnh.'),
('019a6f01-0000-7000-8000-000000000002', 'fantasy', 'Kỳ Ảo', 'Tác phẩm có ma thuật, thần thoại hoặc quy luật kỳ ảo giữ vai trò quan trọng; không quy định thời đại, địa điểm, mức độ phép thuật hay tuyến tình cảm.'),
('019a6f01-0000-7000-8000-000000000003', 'ancient', 'Cổ Đại', 'Tag bối cảnh xã hội cổ đại hoặc cổ phong hư cấu; không mặc định có tình yêu hay dựa trên một giai đoạn lịch sử có thật.'),
('019a6f01-0000-7000-8000-000000000004', 'modern', 'Hiện Đại', 'Tag bối cảnh thời hiện đại, có thể ở thành thị hoặc nông thôn; không mặc định tuyến tình cảm.'),
('019a6f01-0000-7000-8000-000000000005', 'academy', 'Học Viện', 'Tag bối cảnh trường học hoặc cơ sở đào tạo, gồm đời sống học tập và quan hệ thầy trò; không mặc định ma thuật.'),
('019a6f01-0000-7000-8000-000000000006', 'dark', 'Đen Tối', 'Tag sắc thái u ám, tàn khốc hoặc đạo đức xám; không mặc định nhân vật phải hắc hóa, có yếu tố kinh dị hay quan hệ tình cảm độc hại.'),
('019a6f01-0000-7000-8000-000000000007', 'supernatural', 'Siêu Nhiên', 'Tag về thực thể hoặc hiện tượng vượt quy luật tự nhiên như linh hồn, ma cà rồng và người sói; không mặc định gây sợ hãi hoặc có tình yêu.'),
('019a6f01-0000-7000-8000-000000000008', 'progression', 'Thăng Tiến Năng Lực', 'Tag về trọng tâm rèn luyện và tăng năng lực qua các cấp độ có thể theo dõi; không bắt buộc ma thuật, hệ thống game, xuất phát điểm phế vật hay báo thù.'),
('019a6f01-0000-7000-8000-000000000009', 'action', 'Hành Động', 'Tác phẩm lấy các hành động đối đầu, chiến đấu, truy đuổi và ứng phó hiểm nguy làm trọng tâm.'),
('019a6f01-0000-7000-8000-00000000000a', 'adventure', 'Phiêu Lưu', 'Tác phẩm lấy hành trình khám phá, thám hiểm và trải nghiệm ở những vùng đất hoặc hoàn cảnh mới làm trọng tâm.')
ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name, description = EXCLUDED.description;
DELETE FROM genres WHERE code IN ("sci_fi", "mystery");

-- -----------------------------------------------------------------------------
-- 2. System Motifs & Associated Structural Components
-- -----------------------------------------------------------------------------
-- Tham chiếu nghiên cứu thị trường: + [Reedsy - Hero's Journey: Get a Strong Story Structure in 12 Steps](https://reedsy.com/blog/guide/story-structure/heros-journey/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0001-0000-7000-8000-000000000001', 'heros_journey_12_steps', 'Hành trình Anh hùng 12 Giai đoạn (The Hero''s Journey)', 'plot', '1.0.0', true, 'Cấu trúc kinh điển 12 giai đoạn kế thừa từ Joseph Campbell và Christopher Vogler, dẫn dắt nhân vật từ Thế giới bình thường qua ngọn lửa thử thách và trở về với Thần dược biến đổi.', 24, 120);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000000015', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000000016', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'action'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000002016', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'adventure'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0001-0000-7000-8000-000000000017', '018f0001-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'space_opera'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000029', 'linear_stage_progression', 'Quy tắc tiến trình tuyến tính tuần tự')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000002a', 'threshold_point_of_no_return', 'Quy tắc vượt ngưỡng không thể quay đầu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000065', 'hj_ordinary_world', 'Thế giới Bình thường (The Ordinary World)', 'Nhân vật xuất phát điểm từ cuộc sống đơn điệu, ẩn chứa cảm giác thiếu sót hoặc bất an.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000d2', '018f0001-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000066', 'hj_call_to_adventure', 'Tiếng gọi Lên đường (Call to Adventure)', 'Sự kiện phá vỡ thế cân bằng, đặt ra mục tiêu hoặc nguy cơ đe dọa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000dc', '018f0001-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000067', 'hj_refusal_of_call', 'Từ chối Tiếng gọi (Refusal of the Call)', 'Sự do dự, sợ hãi, ràng buộc gia đình hoặc gánh nặng quá khứ kìm hãm nhân vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000e6', '018f0001-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000068', 'hj_meeting_mentor', 'Gặp gỡ Người dẫn đường (Meeting the Mentor)', 'Nhận được sự chỉ bảo, báu vật hoặc niềm tin từ người thầy thông thái.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000f0', '018f0001-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000069', 'hj_crossing_first_threshold', 'Vượt qua Ngưỡng cửa Đầu tiên (Crossing the First Threshold)', 'Chính thức bước vào Thế giới Đặc biệt đầy hiểm họa và phép thuật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-0000000000fb', '018f0001-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'threshold_point_of_no_return'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006a', 'hj_tests_allies_enemies', 'Thử thách, Đồng minh và Kẻ thù (Tests, Allies, Enemies)', 'Làm quen với luật chơi mới, xây dựng liên minh và đối phó với tay chân phản diện.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000104', '018f0001-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006b', 'hj_approach_inmost_cave', 'Tiếp cận Hang sâu Tận cùng (Approach to the Inmost Cave)', 'Chuẩn bị tâm thế và nguồn lực trước khi chạm trán trung tâm hiểm nguy.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-00000000010e', '018f0001-0000-7000-8000-00000000006b', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006c', 'hj_the_ordeal', 'Đại Thử thách Sinh tử (The Ordeal)', 'Đối diện với cái chết, chạm đáy khủng hoảng để tái sinh với sức mạnh mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000118', '018f0001-0000-7000-8000-00000000006c', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006d', 'hj_reward_seizing_sword', 'Phần thưởng / Đoạt lấy Bảo kiếm (Reward)', 'Chiến thắng thử thách cốt lõi, nắm giữ thần vật hoặc tri thức giác ngộ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000122', '018f0001-0000-7000-8000-00000000006d', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006e', 'hj_the_road_back', 'Đường về Đầy Sóng gió (The Road Back)', 'Sự truy đuổi điên cuồng của phe ác, hậu quả của việc mang bảo vật trở về.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-00000000012c', '018f0001-0000-7000-8000-00000000006e', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000006f', 'hj_resurrection', 'Sự Phục sinh / Khảo nghiệm Cuối cùng (The Resurrection)', 'Trận chiến tối thượng thử nghiệm sự biến đổi toàn diện của người anh hùng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000136', '018f0001-0000-7000-8000-00000000006f', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000070', 'hj_return_with_elixir', 'Trở về cùng Thần dược (Return with the Elixir)', 'Mang theo bình an, tri thức cứu rỗi cộng đồng và hoàn thiện bản thân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0001-0000-7000-8000-000000000140', '018f0001-0000-7000-8000-000000000070', (SELECT id FROM state_rules WHERE code = 'linear_stage_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000012d', 'cmp_status_quo_setup', 'Thiết lập Hiện trạng', 'Khắc họa khát vọng ngầm và bối cảnh sinh sống.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000012e', 'cmp_herald_arrival', 'Sứ giả Báo hiệu', 'Mang đến thông điệp hoặc lời thách thức định mệnh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-00000000012f', 'cmp_hesitation_crisis', 'Khủng hoảng Do dự', 'Nỗi sợ thất bại đối đầu với áp lực nội tại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000130', 'cmp_mentor_bestowal', 'Tôn sư Ban ấn', 'Vũ khí đặc biệt hoặc kim chỉ nam tinh thần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000131', 'cmp_gatekeeper_defeat', 'Hạ Người giữ cổng', 'Hành vi quyết đoán bước qua điểm không thể quay lại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000132', 'cmp_trials_gauntlet', 'Chuỗi Thử thách Tôi rèn', 'Tập hợp đồng đội trung thành và phân hóa kẻ thù.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000133', 'cmp_inmost_advance', 'Tiến công Sào huyệt', 'Thâm nhập hang ổ rồng hoặc lãnh địa bóng tối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000134', 'cmp_death_brush', 'Trải nghiệm Cận tử', 'Hiến tế cái tôi cũ để phá vỡ giới hạn phàm trần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000135', 'cmp_treasure_claim', 'Chiếm đoạt Bảo vật', 'Thu hoạch quả ngọt của lòng dũng cảm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000136', 'cmp_perilous_escape', 'Thoát hiểm Nguy nan', 'Cuộc rượt đuổi tốc độ cao về lại ranh giới quen thuộc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000137', 'cmp_final_cleansing', 'Thanh tẩy Tối hậu', 'Chiến thắng quyết định xóa tan tàn dư ác mộng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0001-0000-7000-8000-000000000138', 'cmp_boon_distribution', 'Phân phát Phúc lành', 'Chữa lành quê hương và thiết lập trật tự tươi sáng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000191', 'emo_monotony', 'Đơn điệu Ngột ngạt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000192', 'emo_shock_disruption', 'Choáng váng Chấn động')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000193', 'emo_apprehension', 'Bồn chồn Lo sợ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000194', 'emo_reassurance', 'An tâm Tin tưởng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000195', 'emo_solemn_resolve', 'Quyết tâm Trang nghiêm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000196', 'emo_camaraderie', 'Tình bạn Đồng cam Cộng khổ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000197', 'emo_suffocating_dread', 'Nỗi kinh hoàng Nghẹt thở')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000198', 'emo_mortal_catharsis', 'Giải thoát Sinh tử')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-000000000199', 'emo_exultation', 'Hân hoan Chiến thắng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000019a', 'emo_racing_urgency', 'Gấp gáp Nguy cấp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000019b', 'emo_sublime_triumph', 'Chiến thắng Siêu phàm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0001-0000-7000-8000-00000000019c', 'emo_transcendent_peace', 'Bình yên Thanh thản')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f5', 'eb_hj_ordinary_world', '018f0001-0000-7000-8000-00000000012d', '018f0001-0000-7000-8000-000000000065', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_monotony'), '018f0001-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f6', 'eb_hj_call', '018f0001-0000-7000-8000-00000000012e', '018f0001-0000-7000-8000-000000000066', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_shock_disruption'), '018f0001-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f7', 'eb_hj_refusal', '018f0001-0000-7000-8000-00000000012f', '018f0001-0000-7000-8000-000000000067', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_apprehension'), '018f0001-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f8', 'eb_hj_mentor', '018f0001-0000-7000-8000-000000000130', '018f0001-0000-7000-8000-000000000068', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_reassurance'), '018f0001-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001f9', 'eb_hj_threshold', '018f0001-0000-7000-8000-000000000131', '018f0001-0000-7000-8000-000000000069', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_solemn_resolve'), '018f0001-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fa', 'eb_hj_trials', '018f0001-0000-7000-8000-000000000132', '018f0001-0000-7000-8000-00000000006a', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_camaraderie'), '018f0001-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fb', 'eb_hj_approach', '018f0001-0000-7000-8000-000000000133', '018f0001-0000-7000-8000-00000000006b', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002a4', (SELECT id FROM emotional_ranges WHERE code = 'emo_suffocating_dread'), '018f0001-0000-7000-8000-0000000001fb')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fc', 'eb_hj_ordeal', '018f0001-0000-7000-8000-000000000134', '018f0001-0000-7000-8000-00000000006c', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002af', (SELECT id FROM emotional_ranges WHERE code = 'emo_mortal_catharsis'), '018f0001-0000-7000-8000-0000000001fc')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fd', 'eb_hj_reward', '018f0001-0000-7000-8000-000000000135', '018f0001-0000-7000-8000-00000000006d', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002ba', (SELECT id FROM emotional_ranges WHERE code = 'emo_exultation'), '018f0001-0000-7000-8000-0000000001fd')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001fe', 'eb_hj_road_back', '018f0001-0000-7000-8000-000000000136', '018f0001-0000-7000-8000-00000000006e', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002c5', (SELECT id FROM emotional_ranges WHERE code = 'emo_racing_urgency'), '018f0001-0000-7000-8000-0000000001fe')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-0000000001ff', 'eb_hj_resurrection', '018f0001-0000-7000-8000-000000000137', '018f0001-0000-7000-8000-00000000006f', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002d0', (SELECT id FROM emotional_ranges WHERE code = 'emo_sublime_triumph'), '018f0001-0000-7000-8000-0000000001ff')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0001-0000-7000-8000-000000000200', 'eb_hj_return', '018f0001-0000-7000-8000-000000000138', '018f0001-0000-7000-8000-000000000070', '018f0001-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0001-0000-7000-8000-0000000002db', (SELECT id FROM emotional_ranges WHERE code = 'emo_transcendent_peace'), '018f0001-0000-7000-8000-000000000200')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002bd', 'Bước qua Tiếng gọi', '018f0001-0000-7000-8000-0000000001f5', '018f0001-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002be', 'Rút lui do Do dự', '018f0001-0000-7000-8000-0000000001f6', '018f0001-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002bf', 'Thức tỉnh nhờ Tiền bối', '018f0001-0000-7000-8000-0000000001f7', '018f0001-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c0', 'Vượt ranh giới Thực tại', '018f0001-0000-7000-8000-0000000001f8', '018f0001-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c1', 'Đối đầu Sa trường', '018f0001-0000-7000-8000-0000000001f9', '018f0001-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c2', 'Thâm nhập Nguyệt động', '018f0001-0000-7000-8000-0000000001fa', '018f0001-0000-7000-8000-0000000001fb', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c3', 'Chạm trán Vực thẳm Hủy diệt', '018f0001-0000-7000-8000-0000000001fb', '018f0001-0000-7000-8000-0000000001fc', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c4', 'Đoạt Chân mệnh Thánh vật', '018f0001-0000-7000-8000-0000000001fc', '018f0001-0000-7000-8000-0000000001fd', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c5', 'Trở về trong Khói lửa', '018f0001-0000-7000-8000-0000000001fd', '018f0001-0000-7000-8000-0000000001fe', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c6', 'Khảo nghiệm Sinh tử Tối cao', '018f0001-0000-7000-8000-0000000001fe', '018f0001-0000-7000-8000-0000000001ff', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0001-0000-7000-8000-0000000002c7', 'Bình định Thế giới', '018f0001-0000-7000-8000-0000000001ff', '018f0001-0000-7000-8000-000000000200', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0001-0000-7000-8000-000000000321', 'hj_mandatory_ordeal', 'Giai đoạn Đại thử thách (The Ordeal) bắt buộc phải diễn ra trước khi nhân vật nhận phần thưởng.', 'Người anh hùng không thể trưởng thành về mặt tâm thức nếu không vượt qua trải nghiệm cận tử.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0001-0000-7000-8000-000000000385', 'heros_journey_12_steps_hj_mandatory_ordeal', (SELECT id FROM constraints WHERE code = 'hj_mandatory_ordeal'), '018f0001-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''heros_journey_12_steps'', ''hj_mandatory_ordeal'')', 'Giai đoạn Đại thử thách (The Ordeal) bắt buộc phải diễn ra trước khi nhân vật nhận phần thưởng.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0001-0000-7000-8000-000000000322', 'hj_elixir_restoration', 'Thần dược (Elixir) phải có khả năng chữa lành hoặc thăng hoa thế giới xuất phát điểm.', 'Bảo đảm tính chu toàn của vòng cung hành trình phụng sự cộng đồng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0001-0000-7000-8000-000000000386', 'heros_journey_12_steps_hj_elixir_restoration', (SELECT id FROM constraints WHERE code = 'hj_elixir_restoration'), '018f0001-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''heros_journey_12_steps'', ''hj_elixir_restoration'')', 'Thần dược (Elixir) phải có khả năng chữa lành hoặc thăng hoa thế giới xuất phát điểm.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Save the Cat! - What Is the Save the Cat Method? 15 Beat Sheet Guide](https://savethecat.com/get-started):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0002-0000-7000-8000-000000000001', 'save_the_cat_15_beats', 'Cấu trúc 15 Nhịp Save the Cat (Blake Snyder Beat Sheet)', 'plot', '1.0.0', true, 'Phương pháp tự sự điện ảnh 15 nhịp phân chia thành 3 Hồi với tỷ lệ nhịp điệu chính xác: từ Opening Image, Catalyst, Break into Two, Midpoint, All is Lost đến Final Image.', 20, 60);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0002-0000-7000-8000-000000000015', '018f0002-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'contemporary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000029', 'three_act_percentage_rule', 'Quy tắc tỷ lệ phân bổ 3 Hồi chuẩn xác')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-00000000002a', 'midpoint_polarity_reversal', 'Quy tắc đảo chiều cực tính tại Điểm giữa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000065', 'stc_opening_image', 'Hình ảnh Mở đầu (Opening Image)', 'Bức tranh chụp nhanh về thế giới của nhân vật trước khi hành trình bắt đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000d2', '018f0002-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000066', 'stc_set_up', 'Thiết lập Bối cảnh & Thiếu sót (Set-Up)', 'Giới thiệu nhân vật chính, các khiếm khuyết nội tâm (Six Things That Need Fixing).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000dc', '018f0002-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000067', 'stc_catalyst', 'Chất xúc tác (Catalyst)', 'Cú hích chấn động phá hủy thế giới cũ, mở ra nhiệm vụ mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000e6', '018f0002-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000068', 'stc_debate', 'Tranh luận / Do dự (Debate)', 'Nhân vật băn khoăn liệu có nên chấp nhận thử thách.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000f0', '018f0002-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000069', 'stc_break_into_two', 'Bước sang Hồi 2 (Break into Two)', 'Nhân vật chính chủ động lựa chọn rời bỏ thế giới cũ bước vào thế giới ngược lại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-0000000000fa', '018f0002-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006a', 'stc_b_story', 'Tuyến truyện B / Mối quan hệ (B Story)', 'Nhân vật hỗ trợ xuất hiện, đại diện cho thông điệp chủ đề và chân lý.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000104', '018f0002-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006b', 'stc_fun_and_games', 'Hứng khởi & Thử nghiệm (Fun and Games)', 'Lời hứa của tiền đề câu chuyện (Promise of the Premise) thể hiện rõ nét nhất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-00000000010e', '018f0002-0000-7000-8000-00000000006b', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006c', 'stc_midpoint', 'Điểm giữa (Midpoint)', 'Đỉnh cao chiến thắng ảo hoặc thất bại ảo; cọc cược được đẩy lên tối đa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000119', '018f0002-0000-7000-8000-00000000006c', (SELECT id FROM state_rules WHERE code = 'midpoint_polarity_reversal'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006d', 'stc_bad_guys_close_in', 'Kẻ xấu Xiết vòng vây (Bad Guys Close In)', 'Thế lực phản diện phản công, nghi kỵ nội bộ xuất hiện.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000122', '018f0002-0000-7000-8000-00000000006d', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006e', 'stc_all_is_lost', 'Mất hết Tất cả (All Is Lost)', 'Đáy vực tuyệt vọng; cái chết tượng trưng hoặc cái chết của người thầy.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-00000000012c', '018f0002-0000-7000-8000-00000000006e', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000006f', 'stc_dark_night_of_the_soul', 'Đêm tối Tâm hồn (Dark Night of the Soul)', 'Sự nghiền ngẫm đau đớn trước khi ngộ ra Chân lý thực sự.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000136', '018f0002-0000-7000-8000-00000000006f', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000070', 'stc_break_into_three', 'Bước sang Hồi 3 (Break into Three)', 'Tia sáng giác ngộ; tìm ra giải pháp mới kết hợp giữa mong muốn và nhu cầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000140', '018f0002-0000-7000-8000-000000000070', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000071', 'stc_finale', 'Đại kết cục (Finale)', 'Áp dụng bài học chân lý đánh bại kẻ thù và thanh lọc thế giới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-00000000014a', '018f0002-0000-7000-8000-000000000071', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000072', 'stc_final_image', 'Hình ảnh Kết thúc (Final Image)', 'Bức tranh đối chiếu trực tiếp với Opening Image, khẳng định sự lột xác vĩnh viễn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0002-0000-7000-8000-000000000154', '018f0002-0000-7000-8000-000000000072', (SELECT id FROM state_rules WHERE code = 'three_act_percentage_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000012d', 'cmp_snap_shot', 'Khắc họa Điểm xuất phát', 'Bộc lộ điểm yếu chết người.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000012e', 'cmp_inciting_shocker', 'Sự cố Bất ngờ Châm ngòi', 'Tin báo tử hoặc mất việc làm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-00000000012f', 'cmp_deliberation_struggle', 'Đấu tranh Tư tưởng', 'Liệu có nên mạo hiểm dấn thân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000130', 'cmp_active_choice_crossover', 'Hành vi Lựa chọn Quyết đoán', 'Tự tay ký hợp đồng hoặc dấn bước.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000131', 'cmp_thematic_mentor_intro', 'Khai mở Tuyến tình cảm', 'Gặp người phản ánh linh hồn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000132', 'cmp_premise_fulfillment', 'Thực hiện Lời hứa Tiền đề', 'Những cảnh hành động/hài hước đặc trưng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000133', 'cmp_false_peak_or_trough', 'Đỉnh cao / Vực sâu Ảo ảnh', 'Đảo chiều cảm xúc kịch liệt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000134', 'cmp_encirclement_escalation', 'Vây hãm Leo thang', 'Áp lực dồn dập từ cả bên trong và bên ngoài.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000135', 'cmp_whiff_of_death', 'Mùi hương của Tử thần', 'Mất đi điểm tựa lớn nhất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000136', 'cmp_mourning_and_epiphany', 'Khóc than & Ngộ đạo', 'Rũ bỏ ảo tưởng để nhìn thấy sự thật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000137', 'cmp_climax_synthesis', 'Tổng hòa Đại chiến', 'Kế hoạch 5 giai đoạn lật ngược tình thế.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0002-0000-7000-8000-000000000138', 'cmp_transformed_contrast', 'Hình chiếu Chuyển hóa Đối nghịch', 'Đối chiếu trực quan với cảnh đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000191', 'emo_stagnant_dissatisfaction', 'Bất mãn Trì trệ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000192', 'emo_abrupt_instability', 'Bất ổn Đột ngột')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000193', 'emo_conflicted_uncertainty', 'Giằng xé Phân vân')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000194', 'emo_novelty_delight', 'Thích thú Khám phá Mới')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000195', 'emo_illusory_confidence', 'Tự tin Hão huyền')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000196', 'emo_creeping_paranoia', 'Hoang mang Hoài nghi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000197', 'emo_abject_despair', 'Tuyệt vọng Hoàn toàn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000198', 'emo_spiritual_rebirth', 'Hồi sinh Tinh thần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-000000000199', 'emo_unshakeable_clarity', 'Sáng suốt Kiên định')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0002-0000-7000-8000-00000000019a', 'emo_transformative_closure', 'Khép lại Trọn vẹn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f5', 'eb_stc_open', '018f0002-0000-7000-8000-00000000012d', '018f0002-0000-7000-8000-000000000065', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_stagnant_dissatisfaction'), '018f0002-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f6', 'eb_stc_catalyst', '018f0002-0000-7000-8000-00000000012e', '018f0002-0000-7000-8000-000000000067', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_abrupt_instability'), '018f0002-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f7', 'eb_stc_debate', '018f0002-0000-7000-8000-00000000012f', '018f0002-0000-7000-8000-000000000068', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_conflicted_uncertainty'), '018f0002-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f8', 'eb_stc_act2', '018f0002-0000-7000-8000-000000000130', '018f0002-0000-7000-8000-000000000069', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_novelty_delight'), '018f0002-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001f9', 'eb_stc_midpoint', '018f0002-0000-7000-8000-000000000133', '018f0002-0000-7000-8000-00000000006c', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_illusory_confidence'), '018f0002-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fa', 'eb_stc_badguys', '018f0002-0000-7000-8000-000000000134', '018f0002-0000-7000-8000-00000000006d', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_creeping_paranoia'), '018f0002-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fb', 'eb_stc_all_lost', '018f0002-0000-7000-8000-000000000135', '018f0002-0000-7000-8000-00000000006e', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002a4', (SELECT id FROM emotional_ranges WHERE code = 'emo_abject_despair'), '018f0002-0000-7000-8000-0000000001fb')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fc', 'eb_stc_dark_night', '018f0002-0000-7000-8000-000000000136', '018f0002-0000-7000-8000-00000000006f', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002af', (SELECT id FROM emotional_ranges WHERE code = 'emo_spiritual_rebirth'), '018f0002-0000-7000-8000-0000000001fc')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fd', 'eb_stc_finale', '018f0002-0000-7000-8000-000000000137', '018f0002-0000-7000-8000-000000000071', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002ba', (SELECT id FROM emotional_ranges WHERE code = 'emo_unshakeable_clarity'), '018f0002-0000-7000-8000-0000000001fd')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0002-0000-7000-8000-0000000001fe', 'eb_stc_final_img', '018f0002-0000-7000-8000-000000000138', '018f0002-0000-7000-8000-000000000072', '018f0002-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0002-0000-7000-8000-0000000002c5', (SELECT id FROM emotional_ranges WHERE code = 'emo_transformative_closure'), '018f0002-0000-7000-8000-0000000001fe')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002bd', 'Sóng gió ập đến', '018f0002-0000-7000-8000-0000000001f5', '018f0002-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002be', 'Phân vân cân nhắc', '018f0002-0000-7000-8000-0000000001f6', '018f0002-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002bf', 'Bước qua điểm xoay', '018f0002-0000-7000-8000-0000000001f7', '018f0002-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c0', 'Tiến lên đỉnh ảo', '018f0002-0000-7000-8000-0000000001f8', '018f0002-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c1', 'Bẫy ngầm kích hoạt', '018f0002-0000-7000-8000-0000000001f9', '018f0002-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c2', 'Đổ vỡ toàn diện', '018f0002-0000-7000-8000-0000000001fa', '018f0002-0000-7000-8000-0000000001fb', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c3', 'Chìm vào bóng tối', '018f0002-0000-7000-8000-0000000001fb', '018f0002-0000-7000-8000-0000000001fc', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c4', 'Phản công quyết liệt', '018f0002-0000-7000-8000-0000000001fc', '018f0002-0000-7000-8000-0000000001fd', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0002-0000-7000-8000-0000000002c5', 'Thiết lập trật tự mới', '018f0002-0000-7000-8000-0000000001fd', '018f0002-0000-7000-8000-0000000001fe', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0002-0000-7000-8000-000000000321', 'stc_mirror_images', 'Opening Image và Final Image phải tương phản trực tiếp nhằm làm nổi bật sự biến đổi nhân vật.', 'Save the Cat coi sự biến đổi hình ảnh trước và sau là thước đo thành công tối thượng của cấu trúc kịch bản.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0002-0000-7000-8000-000000000385', 'save_the_cat_15_beats_stc_mirror_images', (SELECT id FROM constraints WHERE code = 'stc_mirror_images'), '018f0002-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''save_the_cat_15_beats'', ''stc_mirror_images'')', 'Opening Image và Final Image phải tương phản trực tiếp nhằm làm nổi bật sự biến đổi nhân vật.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0002-0000-7000-8000-000000000322', 'stc_all_is_lost_whiff', 'Tại nhịp All Is Lost bắt buộc phải có Whiff of Death (mùi của tử thần).', 'Phải có cái cũ bị hủy diệt thì cái tôi mới mới có thể hồi sinh.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0002-0000-7000-8000-000000000386', 'save_the_cat_15_beats_stc_all_is_lost_whiff', (SELECT id FROM constraints WHERE code = 'stc_all_is_lost_whiff'), '018f0002-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''save_the_cat_15_beats'', ''stc_all_is_lost_whiff'')', 'Tại nhịp All Is Lost bắt buộc phải có Whiff of Death (mùi của tử thần).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Reedsy - Dan Harmon Story Circle: The 8-Step Storytelling Shortcut](https://reedsy.com/blog/guide/story-structure/dan-harmon-story-circle/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0003-0000-7000-8000-000000000001', 'dan_harmon_story_circle', 'Vòng tròn Cốt truyện 8 Bước Dan Harmon (Harmon Story Circle)', 'plot', '1.0.0', true, 'Mô hình rút gọn cô đọng 8 bước của cấu trúc vòng lặp: You (Thế giới quen thuộc) -> Need -> Go (Ranh giới lạ) -> Search -> Find -> Take (Trả giá) -> Return -> Change.', 8, 40);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0003-0000-7000-8000-000000000015', '018f0003-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'comedy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0003-0000-7000-8000-000000000016', '018f0003-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0003-0000-7000-8000-000000000017', '018f0003-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'science_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000029', 'circular_descent_ascent_rule', 'Quy tắc lặn sâu xuống vô thức và nổi lên thực tại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-00000000002a', 'cost_of_change_balance', 'Quy tắc trả giá tương xứng để nhận sự chuyển hóa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000065', 'dh_you', '1. You (Nhân vật trong Vùng An toàn)', 'Khởi đầu tại nơi chốn quen thuộc, thoải mái nhưng trì trệ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000d2', '018f0003-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000066', 'dh_need', '2. Need (Nhu cầu / Khiếm khuyết)', 'Nhưng họ muốn điều gì đó; sự mất cân bằng thôi thúc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000dc', '018f0003-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000067', 'dh_go', '3. Go (Bước vào Hoàn cảnh Lạ)', 'Họ bước vào hoàn cảnh xa lạ, qua ranh giới trật tự và hỗn mang.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000e6', '018f0003-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000068', 'dh_search', '4. Search (Thích nghi & Tìm kiếm)', 'Thích nghi với luật chơi mới, liên tục vấp ngã và thử nghiệm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000f0', '018f0003-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000069', 'dh_find', '5. Find (Đạt được Điều mong muốn)', 'Chạm đến cốt lõi mục tiêu tìm kiếm, nhưng là chiến thắng sớm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-0000000000fa', '018f0003-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000006a', 'dh_take', '6. Take (Trả một Cái giá Đắt)', 'Cái giá phải trả cho việc nắm giữ điều đó là vô cùng nặng nề.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-000000000105', '018f0003-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'cost_of_change_balance'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000006b', 'dh_return', '7. Return (Quay về Nơi Xuất phát)', 'Trở lại nơi chốn quen thuộc mang theo kinh nghiệm sống sót.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-00000000010e', '018f0003-0000-7000-8000-00000000006b', (SELECT id FROM state_rules WHERE code = 'circular_descent_ascent_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000006c', 'dh_change', '8. Change (Chuyển hóa Bản thân)', 'Có khả năng thay đổi thế giới xung quanh vì bản thân đã vĩnh viễn đổi khác.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0003-0000-7000-8000-000000000119', '018f0003-0000-7000-8000-00000000006c', (SELECT id FROM state_rules WHERE code = 'cost_of_change_balance'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000012d', 'cmp_comfort_bubble', 'Bong bóng An toàn', 'Mô tả sinh hoạt hàng ngày trước biến cố.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000012e', 'cmp_internal_itch', 'Cơn ngứa Ngáy Khao khát', 'Nỗi khao khát không thể kìm nén.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-00000000012f', 'cmp_portal_step', 'Bước qua Cổng lạ', 'Lao vào vùng đất hoặc hoàn cảnh đối nghịch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000130', 'cmp_trial_adaptation', 'Thích ứng Khắc nghiệt', 'Học cách thở dưới nước hay sinh tồn trong rừng rậm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000131', 'cmp_goal_acquisition', 'Thu giữ Mục tiêu', 'Tìm thấy chiếc chìa khóa hoặc vị trí đối tượng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000132', 'cmp_heavy_toll_payment', 'Thanh toán Giá đắt', 'Mất đi vũ khí, bạn đồng hành hoặc một phần danh dự.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000133', 'cmp_threshold_reentry', 'Tái nhập Cảnh giới', 'Quay về căn phòng quen thuộc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0003-0000-7000-8000-000000000134', 'cmp_active_mastery', 'Làm chủ Hoàn cảnh', 'Giải quyết mâu thuẫn cũ bằng góc nhìn mới tinh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000191', 'emo_complacent_comfort', 'Thoải mái Thỏa mãn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000192', 'emo_restless_craving', 'Bứt rứt Thèm khát')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000193', 'emo_disoriented_wonder', 'Ngỡ ngàng Choáng ngợp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000194', 'emo_tenacious_struggle', 'Nỗ lực Kiên trì')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000195', 'emo_euphoric_discovery', 'Hưng phấn Khám phá')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000196', 'emo_crushing_toll', 'Đau xót Mất mát')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000197', 'emo_nostalgic_exhaustion', 'Kiệt sức Hoài niệm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0003-0000-7000-8000-000000000198', 'emo_empowered_peace', 'Tự tin Thanh thản')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f5', 'eb_dh_you', '018f0003-0000-7000-8000-00000000012d', '018f0003-0000-7000-8000-000000000065', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_complacent_comfort'), '018f0003-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f6', 'eb_dh_need', '018f0003-0000-7000-8000-00000000012e', '018f0003-0000-7000-8000-000000000066', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_restless_craving'), '018f0003-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f7', 'eb_dh_go', '018f0003-0000-7000-8000-00000000012f', '018f0003-0000-7000-8000-000000000067', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_disoriented_wonder'), '018f0003-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f8', 'eb_dh_search', '018f0003-0000-7000-8000-000000000130', '018f0003-0000-7000-8000-000000000068', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_tenacious_struggle'), '018f0003-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001f9', 'eb_dh_find', '018f0003-0000-7000-8000-000000000131', '018f0003-0000-7000-8000-000000000069', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_euphoric_discovery'), '018f0003-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001fa', 'eb_dh_take', '018f0003-0000-7000-8000-000000000132', '018f0003-0000-7000-8000-00000000006a', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_crushing_toll'), '018f0003-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001fb', 'eb_dh_return', '018f0003-0000-7000-8000-000000000133', '018f0003-0000-7000-8000-00000000006b', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-0000000002a4', (SELECT id FROM emotional_ranges WHERE code = 'emo_nostalgic_exhaustion'), '018f0003-0000-7000-8000-0000000001fb')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0003-0000-7000-8000-0000000001fc', 'eb_dh_change', '018f0003-0000-7000-8000-000000000134', '018f0003-0000-7000-8000-00000000006c', '018f0003-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0003-0000-7000-8000-0000000002af', (SELECT id FROM emotional_ranges WHERE code = 'emo_empowered_peace'), '018f0003-0000-7000-8000-0000000001fc')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002bd', 'Khát khao thúc bách', '018f0003-0000-7000-8000-0000000001f5', '018f0003-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002be', 'Dấn thân vùng lạ', '018f0003-0000-7000-8000-0000000001f6', '018f0003-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002bf', 'Thích ứng sóng gió', '018f0003-0000-7000-8000-0000000001f7', '018f0003-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c0', 'Chạm tay vào điều ước', '018f0003-0000-7000-8000-0000000001f8', '018f0003-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c1', 'Hiến tế đẫm máu', '018f0003-0000-7000-8000-0000000001f9', '018f0003-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c2', 'Hồi hương thấm mệt', '018f0003-0000-7000-8000-0000000001fa', '018f0003-0000-7000-8000-0000000001fb', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0003-0000-7000-8000-0000000002c3', 'Bộc lộ bản lĩnh mới', '018f0003-0000-7000-8000-0000000001fb', '018f0003-0000-7000-8000-0000000001fc', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0003-0000-7000-8000-000000000321', 'dh_loss_equilibrium', 'Tại giai đoạn Take (Bước 6), cái giá phải trả phải khiến nhân vật mất đi sự hồn nhiên hoặc một vật phẩm quý giá.', 'Không có sự mất mát thì nhân vật không có sự tái sinh ở Bước 8.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0003-0000-7000-8000-000000000385', 'dan_harmon_story_circle_dh_loss_equilibrium', (SELECT id FROM constraints WHERE code = 'dh_loss_equilibrium'), '018f0003-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''dan_harmon_story_circle'', ''dh_loss_equilibrium'')', 'Tại giai đoạn Take (Bước 6), cái giá phải trả phải khiến nhân vật mất đi sự hồn nhiên hoặc một vật phẩm quý giá.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0003-0000-7000-8000-000000000322', 'dh_top_bottom_division', 'Nửa trên vòng tròn (1,2,7,8) là thế giới ý thức quen thuộc; nửa dưới (3,4,5,6) bắt buộc là thế giới vô thức/xa lạ.', 'Giữ đúng nguyên lý cấu trúc tự sự lưỡng phân của Dan Harmon.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0003-0000-7000-8000-000000000386', 'dan_harmon_story_circle_dh_top_bottom_division', (SELECT id FROM constraints WHERE code = 'dh_top_bottom_division'), '018f0003-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''dan_harmon_story_circle'', ''dh_top_bottom_division'')', 'Nửa trên vòng tròn (1,2,7,8) là thế giới ý thức quen thuộc; nửa dưới (3,4,5,6) bắt buộc là thế giới vô thức/xa lạ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Helping Writers Become Authors - Kishōtenketsu Story Structure: What It Is, How It Works, and Examples](https://www.helpingwritersbecomeauthors.com/kishotenketsu-story-structure/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0004-0000-7000-8000-000000000001', 'kishotenketsu_4_acts', 'Cấu trúc 4 Hồi Kishōtenketsu (Khởi - Thừa - Chuyển - Kết)', 'plot', '1.0.0', true, 'Cấu trúc tự sự Đông Á truyền thống không dựa trên xung đột đối kháng kiểu phương Tây mà dựa trên sự tiếp cận, đối chiếu và bất ngờ hòa quyện ý nghĩa: Ki (Khởi) -> Shō (Thừa) -> Ten (Chuyển) -> Ketsu (Kết).', 4, 50);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0004-0000-7000-8000-000000000015', '018f0004-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'slice_of_life'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0004-0000-7000-8000-000000000016', '018f0004-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mystery_detective'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0004-0000-7000-8000-000000000017', '018f0004-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'literary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000029', 'non_conflict_juxtaposition', 'Quy tắc ghép nối phi đối kháng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-00000000002a', 'thematic_synthesis_revelation', 'Quy tắc tổng hòa ý nghĩa giác ngộ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000065', 'kstk_ki', 'Ki (Khởi - Giới thiệu Hiện trạng)', 'Giới thiệu nhân vật, thời gian, không gian và nếp sống ban đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000d2', '018f0004-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'non_conflict_juxtaposition'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000066', 'kstk_sho', 'Shō (Thừa - Mở rộng Phát triển)', 'Tiếp nối mạch câu chuyện, mở rộng chi tiết về hoàn cảnh mà không tạo xung đột lớn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000dc', '018f0004-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'non_conflict_juxtaposition'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000067', 'kstk_ten', 'Ten (Chuyển - Yếu tố Bất ngờ / Đổi hướng)', 'Đột ngột xuất hiện một chủ đề, đối tượng hoặc sự kiện tưởng chừng không liên quan.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000e6', '018f0004-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'non_conflict_juxtaposition'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000068', 'kstk_ketsu', 'Ketsu (Kết - Hòa quyện Ý nghĩa)', 'Liên kết yếu tố Ten với Ki và Shō, tạo ra cái nhìn sâu sắc mới mẻ về cuộc đời.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0004-0000-7000-8000-0000000000f1', '018f0004-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'thematic_synthesis_revelation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-00000000012d', 'cmp_scene_introduction', 'Thiết lập Bức họa Mở màn', 'Góc nhìn tĩnh tại về cảnh vật hoặc hoạt động thường nhật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-00000000012e', 'cmp_deepening_cadence', 'Nhịp điệu Sâu sắc', 'Thêm nhân vật phụ hoặc thói quen được củng cố.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-00000000012f', 'cmp_disruptive_angle', 'Góc nhìn Khác biệt Hoàn toàn', 'Một phân cảnh hoàn toàn tách biệt được lồng ghép.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0004-0000-7000-8000-000000000130', 'cmp_harmonic_unification', 'Hợp âm Hòa quyện', 'Mối dây vô hình buộc hai nửa lại với nhau.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000191', 'emo_serene_curiosity', 'Hiếu kỳ Thanh thản')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000192', 'emo_gentle_engagement', 'Cuốn hút Dịu êm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000193', 'emo_baffled_surprise', 'Ngạc nhiên Ngỡ ngàng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0004-0000-7000-8000-000000000194', 'emo_illuminated_harmony', 'Hòa hợp Bừng sáng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f5', 'eb_kstk_ki', '018f0004-0000-7000-8000-00000000012d', '018f0004-0000-7000-8000-000000000065', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_serene_curiosity'), '018f0004-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f6', 'eb_kstk_sho', '018f0004-0000-7000-8000-00000000012e', '018f0004-0000-7000-8000-000000000066', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_gentle_engagement'), '018f0004-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f7', 'eb_kstk_ten', '018f0004-0000-7000-8000-00000000012f', '018f0004-0000-7000-8000-000000000067', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_baffled_surprise'), '018f0004-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0004-0000-7000-8000-0000000001f8', 'eb_kstk_ketsu', '018f0004-0000-7000-8000-000000000130', '018f0004-0000-7000-8000-000000000068', '018f0004-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0004-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_illuminated_harmony'), '018f0004-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0004-0000-7000-8000-0000000002bd', 'Mở rộng không gian', '018f0004-0000-7000-8000-0000000001f5', '018f0004-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0004-0000-7000-8000-0000000002be', 'Góc ngoặt bất ngờ', '018f0004-0000-7000-8000-0000000001f6', '018f0004-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0004-0000-7000-8000-0000000002bf', 'Hội tụ ý nghĩa', '018f0004-0000-7000-8000-0000000001f7', '018f0004-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0004-0000-7000-8000-000000000321', 'kstk_ten_independence', 'Hồi Ten phải mang tính bất ngờ, độc lập bề ngoài so với mạch kể của Ki và Sho trước khi được giải mã ở Ketsu.', 'Sức hấp dẫn của Kishōtenketsu đến từ sự kinh ngạc trước mối liên hệ ngầm giữa hai hình tượng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0004-0000-7000-8000-000000000385', 'kishotenketsu_4_acts_kstk_ten_independence', (SELECT id FROM constraints WHERE code = 'kstk_ten_independence'), '018f0004-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''kishotenketsu_4_acts'', ''kstk_ten_independence'')', 'Hồi Ten phải mang tính bất ngờ, độc lập bề ngoài so với mạch kể của Ki và Sho trước khi được giải mã ở Ketsu.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0004-0000-7000-8000-000000000322', 'kstk_no_villain_necessity', 'Cấu trúc không bắt buộc phải có nhân vật phản diện hay động lực tiêu diệt đối thủ.', 'Bảo tồn bản sắc tự sự phi xung đột triết lý Á Đông.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0004-0000-7000-8000-000000000386', 'kishotenketsu_4_acts_kstk_no_villain_necessity', (SELECT id FROM constraints WHERE code = 'kstk_no_villain_necessity'), '018f0004-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''kishotenketsu_4_acts'', ''kstk_no_villain_necessity'')', 'Cấu trúc không bắt buộc phải có nhân vật phản diện hay động lực tiêu diệt đối thủ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [StudioBinder - What is Freytag's Pyramid: 5 Steps of Story Structure](https://www.studiobinder.com/blog/what-is-freytags-pyramid-definition/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0005-0000-7000-8000-000000000001', 'freytags_pyramid_dramatic', 'Kim tự tháp Kịch tính Freytag (Freytag''s Pyramid 5 Acts)', 'plot', '1.0.0', true, 'Mô hình bi kịch kinh điển thế kỷ 19 của Gustav Freytag gồm 5 phần đối xứng: Exposition, Rising Action, Climax, Falling Action và Catastrophe/Denouement.', 15, 75);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0005-0000-7000-8000-000000000015', '018f0005-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'literary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0005-0000-7000-8000-000000000017', '018f0005-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'dark'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000029', 'hamartia_driven_escalation', 'Quy tắc leo thang do lỗi lầm định mệnh (Hamartia)')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-00000000002a', 'inevitable_downward_momentum', 'Quy tắc trượt dốc tất yếu hậu đỉnh điểm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000065', 'fp_exposition', '1. Exposition (Khởi đầu & Thiết lập Bối cảnh)', 'Giới thiệu nhân vật, tính cách, bối cảnh thời đại và hạt mầm xung đột ban sơ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000d2', '018f0005-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'hamartia_driven_escalation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000066', 'fp_rising_action', '2. Rising Action (Hành động Leo thang)', 'Chuỗi sự kiện gia tăng căng thẳng, tham vọng bùng cháy thúc đẩy các quyết định sai lầm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000dc', '018f0005-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'hamartia_driven_escalation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000067', 'fp_climax', '3. Climax (Đỉnh điểm Bi kịch)', 'Đỉnh chóp kim tự tháp: khoảnh khắc nhân vật phạm phải sai lầm không thể cứu vãn (Hubris/Hamartia).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000e6', '018f0005-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'hamartia_driven_escalation'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000068', 'fp_falling_action', '4. Falling Action (Hành động Rơi xuống / Suy tàn)', 'Hậu quả đổ ập xuống, các thế lực liên minh tan rã, sự bất lực trước số mệnh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000f1', '018f0005-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'inevitable_downward_momentum'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000069', 'fp_catastrophe', '5. Catastrophe / Dénouement (Thảm họa & Khép lại)', 'Sự sụp đổ hoàn toàn về thể xác hoặc quyền lực, tái lập trật tự đau thương nhưng công bằng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0005-0000-7000-8000-0000000000fb', '018f0005-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'inevitable_downward_momentum'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-00000000012d', 'cmp_status_and_hubris', 'Vị thế & Kiêu ngạo', 'Khắc họa uy quyền ban đầu đan xen hạt mầm mù quáng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-00000000012e', 'cmp_fatal_entanglement', 'Vướng mắc Định mệnh', 'Những toan tính mưu mô ngày càng lún sâu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-00000000012f', 'cmp_fatal_decision_apex', 'Quyết định Định mệnh Tối thượng', 'Hành vi vượt quá giới hạn đạo đức hoặc số phận.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000130', 'cmp_nemesis_unraveling', 'Báo ứng Giáng xuống', 'Sự phản trắc của thuộc hạ và mất dần quyền kiểm soát.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0005-0000-7000-8000-000000000131', 'cmp_tragic_catharsis', 'Thanh lọc Bi tráng', 'Cái chết của nhân vật trung tâm để lại bài học luân lý sâu sắc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000191', 'emo_sovereign_pride', 'Kiêu ngạo Quyền thế')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000192', 'emo_mounting_fever', 'Cơn sốt Tham vọng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000193', 'emo_apocalyptic_shock', 'Chấn động Kinh hoàng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000194', 'emo_inexorable_dread', 'Nỗi sợ Không thể Cưỡng lại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0005-0000-7000-8000-000000000195', 'emo_tragic_awe', 'Kính sợ Bi tráng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f5', 'eb_fp_expo', '018f0005-0000-7000-8000-00000000012d', '018f0005-0000-7000-8000-000000000065', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_sovereign_pride'), '018f0005-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f6', 'eb_fp_rising', '018f0005-0000-7000-8000-00000000012e', '018f0005-0000-7000-8000-000000000066', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_mounting_fever'), '018f0005-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f7', 'eb_fp_climax', '018f0005-0000-7000-8000-00000000012f', '018f0005-0000-7000-8000-000000000067', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_apocalyptic_shock'), '018f0005-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f8', 'eb_fp_falling', '018f0005-0000-7000-8000-000000000130', '018f0005-0000-7000-8000-000000000068', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_inexorable_dread'), '018f0005-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0005-0000-7000-8000-0000000001f9', 'eb_fp_catastrophe', '018f0005-0000-7000-8000-000000000131', '018f0005-0000-7000-8000-000000000069', '018f0005-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0005-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_tragic_awe'), '018f0005-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002bd', 'Ngòi nổ tham vọng', '018f0005-0000-7000-8000-0000000001f5', '018f0005-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002be', 'Đỉnh cao mù quáng', '018f0005-0000-7000-8000-0000000001f6', '018f0005-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002bf', 'Trượt dốc không phanh', '018f0005-0000-7000-8000-0000000001f7', '018f0005-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0005-0000-7000-8000-0000000002c0', 'Bản án số phận', '018f0005-0000-7000-8000-0000000001f8', '018f0005-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0005-0000-7000-8000-000000000321', 'fp_tragic_flaw_presence', 'Nhân vật trung tâm bắt buộc phải có một khiếm khuyết chết người (Hamartia) gây ra đỉnh điểm Climax.', 'Bản chất Kim tự tháp Freytag là kiến trúc phục vụ bi kịch nhân quả nội tại.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0005-0000-7000-8000-000000000385', 'freytags_pyramid_dramatic_fp_tragic_flaw_presence', (SELECT id FROM constraints WHERE code = 'fp_tragic_flaw_presence'), '018f0005-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''freytags_pyramid_dramatic'', ''fp_tragic_flaw_presence'')', 'Nhân vật trung tâm bắt buộc phải có một khiếm khuyết chết người (Hamartia) gây ra đỉnh điểm Climax.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0005-0000-7000-8000-000000000322', 'fp_catharsis_rebalancing', 'Phần Catastrophe phải giải tỏa được căng thẳng đạo đức qua sự thanh tẩy bi tráng (Catharsis).', 'Giữ đúng nguyên lý thi học Aristotle và kịch nghệ Freytag.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0005-0000-7000-8000-000000000386', 'freytags_pyramid_dramatic_fp_catharsis_rebalancing', (SELECT id FROM constraints WHERE code = 'fp_catharsis_rebalancing'), '018f0005-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''freytags_pyramid_dramatic'', ''fp_catharsis_rebalancing'')', 'Phần Catastrophe phải giải tỏa được căng thẳng đạo đức qua sự thanh tẩy bi tráng (Catharsis).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Plottr - Using the Fichtean Curve Plot Structure for Gripping Crises](https://plottr.com/fichtean-curve-plot-structure/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0006-0000-7000-8000-000000000001', 'fichtean_curve_crises', 'Đường cong Khủng hoảng Fichtean (The Fichtean Curve)', 'plot', '1.0.0', true, 'Cấu trúc nhịp độ cao liên hoàn bỏ qua phần mở đầu dài dòng, bắt đầu in media res và đẩy nhân vật qua một chuỗi các khủng hoảng leo thang không ngừng trước đại đỉnh điểm.', 18, 80);

-- 2. Table motif_applicable_genres

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0006-0000-7000-8000-000000000016', '018f0006-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'horror'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0006-0000-7000-8000-000000000017', '018f0006-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'hunter_dungeon'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0006-0000-7000-8000-000000001017', '018f0006-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'litrpg'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000029', 'continuous_escalating_pressure', 'Quy tắc áp lực gia tăng liên tục không khoảng nghỉ lớn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-00000000002a', 'cumulative_damage_penalty', 'Quy tắc vết thương tích lũy qua từng đợt khủng hoảng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000065', 'fc_in_media_res', 'Khởi đầu giữa Biến cố (In Media Res Start)', 'Nhảy thẳng vào giữa hành động; nhân vật đang trong tình huống ngặt nghèo ngay trang 1.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000d2', '018f0006-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000066', 'fc_crisis_first', 'Khủng hoảng Thứ nhất (Crisis 1 - Thoát hiểm Tạm thời)', 'Giải quyết được nguy cấp đầu tiên nhưng làm phát sinh mối nguy lớn hơn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000dc', '018f0006-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000067', 'fc_crisis_second', 'Khủng hoảng Thứ hai (Crisis 2 - Vây bọc Thắt chặt)', 'Nguồn lực cạn kiệt, đồng minh bị cô lập, bẫy tử thần thu hẹp.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000e7', '018f0006-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'cumulative_damage_penalty'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000068', 'fc_crisis_third', 'Khủng hoảng Thứ ba (Crisis 3 - Bờ vực Tuyệt vọng)', 'Đỉnh điểm phụ kịch tính; mất đi phòng tuyến kiên cố cuối cùng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000f1', '018f0006-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'cumulative_damage_penalty'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000069', 'fc_major_climax', 'Đại Đỉnh điểm (Main Climax)', 'Trận chiến sống còn tối hậu đối đầu trực tiếp nguồn gốc thảm họa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-0000000000fa', '018f0006-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000006a', 'fc_rapid_falling_action', 'Thoát hiểm & Lắng đọng Nhanh (Falling Action & Resolution)', 'Nhịp thở phào nhẹ nhõm ngắn sau chuỗi sinh tử liên miên.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0006-0000-7000-8000-000000000104', '018f0006-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'continuous_escalating_pressure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000012d', 'cmp_immediate_hazard', 'Nguy cơ Trực diện Khẩn cấp', 'Chạy trốn khỏi vụ nổ hoặc sự truy đuổi sát nút.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000012e', 'cmp_escalated_obstacle', 'Chướng ngại Tăng cấp', 'Cửa ngõ bị khóa chặt, thời gian cạn dần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-00000000012f', 'cmp_resource_depletion', 'Cạn kiệt Tài nguyên', 'Hết đạn dược, máu cạn, thuốc men biến mất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000130', 'cmp_cornered_stand', 'Dồn vào Góc tường', 'Không còn đường lui, buộc phải quay đầu tử chiến.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000131', 'cmp_definitive_strike', 'Đòn Đột kích Quyết tử', 'Khai thác điểm yếu chí mạng của kẻ thủ ác.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0006-0000-7000-8000-000000000132', 'cmp_exhausted_respite', 'Nghỉ ngơi Kiệt quệ', 'Ngồi giữa đống đổ nát ngắm ánh bình minh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000191', 'emo_frenetic_panic', 'Hoảng loạn Cuống cuồng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000192', 'emo_breathless_tension', 'Căng thẳng Hụt hơi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000193', 'emo_oppressive_exhaustion', 'Kiệt quệ Đè nặng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000194', 'emo_feral_desperation', 'Tuyệt vọng Hoang dại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000195', 'emo_explosive_catharsis', 'Bùng nổ Giải thoát')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0006-0000-7000-8000-000000000196', 'emo_sobering_relief', 'Nhẹ nhõm Tỉnh táo')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f5', 'eb_fc_media_res', '018f0006-0000-7000-8000-00000000012d', '018f0006-0000-7000-8000-000000000065', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_frenetic_panic'), '018f0006-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f6', 'eb_fc_crisis1', '018f0006-0000-7000-8000-00000000012e', '018f0006-0000-7000-8000-000000000066', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_breathless_tension'), '018f0006-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f7', 'eb_fc_crisis2', '018f0006-0000-7000-8000-00000000012f', '018f0006-0000-7000-8000-000000000067', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_oppressive_exhaustion'), '018f0006-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f8', 'eb_fc_crisis3', '018f0006-0000-7000-8000-000000000130', '018f0006-0000-7000-8000-000000000068', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_feral_desperation'), '018f0006-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001f9', 'eb_fc_climax', '018f0006-0000-7000-8000-000000000131', '018f0006-0000-7000-8000-000000000069', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_explosive_catharsis'), '018f0006-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0006-0000-7000-8000-0000000001fa', 'eb_fc_fall', '018f0006-0000-7000-8000-000000000132', '018f0006-0000-7000-8000-00000000006a', '018f0006-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0006-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_sobering_relief'), '018f0006-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002bd', 'Thoát hiểm tạm thời', '018f0006-0000-7000-8000-0000000001f5', '018f0006-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002be', 'Vòng vây thắt chặt', '018f0006-0000-7000-8000-0000000001f6', '018f0006-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002bf', 'Dồn vào chân tường', '018f0006-0000-7000-8000-0000000001f7', '018f0006-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002c0', 'Bùng nổ tổng lực', '018f0006-0000-7000-8000-0000000001f8', '018f0006-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0006-0000-7000-8000-0000000002c1', 'Khói bụi lắng xuống', '018f0006-0000-7000-8000-0000000001f9', '018f0006-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0006-0000-7000-8000-000000000321', 'fc_no_prolonged_exposition', 'Không được dành quá 5% dung lượng đầu truyện cho giới thiệu bối cảnh tĩnh; phải mở màn bằng hành động trực tiếp.', 'Triết lý Fichtean Curve là giữ chặt độc giả ngay từ câu chữ đầu tiên bằng khủng hoảng tức thì.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0006-0000-7000-8000-000000000385', 'fichtean_curve_crises_fc_no_prolonged_exposition', (SELECT id FROM constraints WHERE code = 'fc_no_prolonged_exposition'), '018f0006-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''fichtean_curve_crises'', ''fc_no_prolonged_exposition'')', 'Không được dành quá 5% dung lượng đầu truyện cho giới thiệu bối cảnh tĩnh; phải mở màn bằng hành động trực tiếp.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0006-0000-7000-8000-000000000322', 'fc_escalation_staircase', 'Mỗi đợt khủng hoảng tiếp theo bắt buộc phải có mức độ thiệt hại hoặc cọc cược cao hơn đợt trước.', 'Ngăn chặn hiện tượng tụt giảm kịch tính giữa các chương.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0006-0000-7000-8000-000000000386', 'fichtean_curve_crises_fc_escalation_staircase', (SELECT id FROM constraints WHERE code = 'fc_escalation_staircase'), '018f0006-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''fichtean_curve_crises'', ''fc_escalation_staircase'')', 'Mỗi đợt khủng hoảng tiếp theo bắt buộc phải có mức độ thiệt hại hoặc cọc cược cao hơn đợt trước.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [MasterClass - Story Archetypes: How to Recognize Christopher Booker's 7 Basic Plots](https://www.masterclass.com/articles/a-guide-to-story-archetypes):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0007-0000-7000-8000-000000000001', 'booker_overcoming_the_monster', 'Hạ gục Quái thú / Thế lực Tà ác (Overcoming the Monster)', 'plot', '1.0.0', true, 'Một trong 7 cốt truyện cơ bản của Christopher Booker: Vùng đất bị đe dọa bởi một Quái vật/Ác nhân dường như bất khả chiến bại. Người anh hùng phải đối mặt trong 5 giai đoạn: Anticipation, Dream, Frustration, Nightmare và Miraculous Escape/Triumph.', 20, 90);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000000015', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000000016', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'horror'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000000017', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mat_the'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0007-0000-7000-8000-000000001017', '018f0007-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'science_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000029', 'monster_existential_threat_rule', 'Quy tắc đe dọa sinh tồn toàn cục')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-00000000002a', 'nightmare_phase_powerlessness', 'Quy tắc giai đoạn ác mộng tước đoạt toàn bộ ưu thế phàm trần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000065', 'otm_anticipation', '1. Anticipation Stage & Call (Kỳ vọng & Tiếng báo nguy)', 'Ý thức về sự hiện diện của con quái vật từ xa; tiếng gọi dấn thân giải cứu cộng đồng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000d2', '018f0007-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000066', 'otm_dream', '2. Dream Stage (Giai đoạn Mơ mộng / Thành công ban đầu)', 'Tiến quân thuận lợi, vượt qua ranh giới an toàn, tự tin đối đầu quái vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000dc', '018f0007-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000067', 'otm_frustration', '3. Frustration Stage (Giai đoạn Ức chế / Rơi vào Bẫy)', 'Lần đầu trực tiếp đối đầu; nhận ra sức mạnh quái thú vượt xa dự liệu, bị mắc kẹt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000e6', '018f0007-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000068', 'otm_nightmare', '4. Nightmare Stage (Giai đoạn Ác mộng Kinh hoàng)', 'Quái thú vươn nanh vuốt áp đảo, cái chết và sự diệt vong kề cận trong gang tấc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000f1', '018f0007-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'nightmare_phase_powerlessness'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000069', 'otm_miraculous_triumph', '5. The Miraculous Triumph (Kỳ tích Thắng lợi & Giải thoát)', 'Khai thác điểm mù độc nhất của quái vật hoặc bộc phát ý chí siêu phàm tiêu diệt nó.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0007-0000-7000-8000-0000000000fa', '018f0007-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'monster_existential_threat_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-00000000012d', 'cmp_monstrous_shadow', 'Bóng ma Quái thú', 'Tin đồn và dấu vết tàn sát rợn người khắp nơi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-00000000012e', 'cmp_confident_march', 'Cuộc hành quân Tự tin', 'Trang bị giáp trụ vũ khí tinh nhuệ sẵn sàng xuất trận.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-00000000012f', 'cmp_crushing_ambush', 'Phục kích Nghiền nát', 'Vũ khí thông thường gãy vụn trước lớp giáp quái thú.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000130', 'cmp_desperate_survival_crawl', 'Bò trườn Tuyệt vọng', 'Đồng đội thương vong, bản thân bị dồn vào ngõ cụt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0007-0000-7000-8000-000000000131', 'cmp_fatal_flaw_strike', 'Cú đâm Gót chân Achilles', 'Nhát chém quyết định kết liễu thực thể tà ác.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000191', 'emo_righteous_zeal', 'Nhiệt huyết Chính nghĩa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000192', 'emo_exhilarated_optimism', 'Lạc quan Phấn chấn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000193', 'emo_chilling_dismay', 'Hoang mang Lạnh gáy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000194', 'emo_paralyzing_horror', 'Kinh hoàng Tê liệt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0007-0000-7000-8000-000000000195', 'emo_euphoric_liberation', 'Giải phóng Hân hoan')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f5', 'eb_otm_anticipate', '018f0007-0000-7000-8000-00000000012d', '018f0007-0000-7000-8000-000000000065', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_righteous_zeal'), '018f0007-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f6', 'eb_otm_dream', '018f0007-0000-7000-8000-00000000012e', '018f0007-0000-7000-8000-000000000066', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_exhilarated_optimism'), '018f0007-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f7', 'eb_otm_frustrate', '018f0007-0000-7000-8000-00000000012f', '018f0007-0000-7000-8000-000000000067', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_chilling_dismay'), '018f0007-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f8', 'eb_otm_nightmare', '018f0007-0000-7000-8000-000000000130', '018f0007-0000-7000-8000-000000000068', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_paralyzing_horror'), '018f0007-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0007-0000-7000-8000-0000000001f9', 'eb_otm_triumph', '018f0007-0000-7000-8000-000000000131', '018f0007-0000-7000-8000-000000000069', '018f0007-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0007-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_euphoric_liberation'), '018f0007-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002bd', 'Tiến quân xuất trận', '018f0007-0000-7000-8000-0000000001f5', '018f0007-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002be', 'Đụng độ bức tường thép', '018f0007-0000-7000-8000-0000000001f6', '018f0007-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002bf', 'Rơi vào miệng quái vật', '018f0007-0000-7000-8000-0000000001f7', '018f0007-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0007-0000-7000-8000-0000000002c0', 'Đòn kỳ tích lật ngược thế cờ', '018f0007-0000-7000-8000-0000000001f8', '018f0007-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0007-0000-7000-8000-000000000321', 'otm_monster_asymmetry', 'Quái vật/Ác nhân phải sở hữu sức mạnh áp đảo vượt trội gấp nhiều lần nhân vật chính ở giai đoạn đầu và giữa.', 'Tạo ra sức nặng đạo đức và cảm giác kỳ tích chân thật khi đánh bại quái vật.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0007-0000-7000-8000-000000000385', 'booker_overcoming_the_monster_otm_monster_asymmetry', (SELECT id FROM constraints WHERE code = 'otm_monster_asymmetry'), '018f0007-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''booker_overcoming_the_monster'', ''otm_monster_asymmetry'')', 'Quái vật/Ác nhân phải sở hữu sức mạnh áp đảo vượt trội gấp nhiều lần nhân vật chính ở giai đoạn đầu và giữa.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0007-0000-7000-8000-000000000322', 'otm_communal_stakes', 'Sự thất bại của nhân vật chính phải đồng nghĩa với sự hủy diệt của toàn bộ cộng đồng hoặc vương quốc.', 'Giữ đúng tầm vóc sử thi của cổ mẫu Christopher Booker.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0007-0000-7000-8000-000000000386', 'booker_overcoming_the_monster_otm_communal_stakes', (SELECT id FROM constraints WHERE code = 'otm_communal_stakes'), '018f0007-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''booker_overcoming_the_monster'', ''otm_communal_stakes'')', 'Sự thất bại của nhân vật chính phải đồng nghĩa với sự hủy diệt của toàn bộ cộng đồng hoặc vương quốc.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Story Grid - The Five Commandments of Storytelling](https://storygrid.com/articles/five-commandments-of-storytelling):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0008-0000-7000-8000-000000000001', 'story_grid_five_commandments', 'Năm Điều răn Tự sự của Story Grid (Five Commandments of Storytelling)', 'plot', '1.0.0', true, 'Khung phương pháp luận biên tập chuyên sâu của Shawn Coyne: Mỗi cảnh (Scene), trường đoạn (Sequence) hay toàn bộ tác phẩm đều phải tuân thủ 5 điều răn: Inciting Incident, Turning Point Progressive Complication, Crisis, Climax và Resolution.', 10, 70);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0008-0000-7000-8000-000000000015', '018f0008-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mystery_detective'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000029', 'value_shift_requirement', 'Quy tắc bắt buộc chuyển dịch giá trị từ đầu đến cuối cảnh')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-00000000002a', 'irreversible_best_bad_choice', 'Quy tắc khủng hoảng buộc chọn điều ít tồi tệ hơn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000065', 'sg_inciting_incident', '1. Sự cố Khởi xướng (Inciting Incident)', 'Phá vỡ trạng thái cân bằng của nhân vật, hoặc do sự việc ngẫu nhiên (Causal) hoặc do hành vi chủ động.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000d2', '018f0008-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000066', 'sg_turning_point', '2. Điểm xoay Biến chứng Tiến triển (Turning Point Complication)', 'Một hành động của nhân vật khác hoặc một tiết lộ bất ngờ (Action/Revelation) làm tắc nghẽn kế hoạch cũ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000dc', '018f0008-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000067', 'sg_crisis_question', '3. Câu hỏi Khủng hoảng (The Crisis Question)', 'Một tình thế tiến thoái lưỡng nan: Chọn điều tốt hơn giữa hai điều tốt, hoặc chọn điều ít tệ hơn (Best Bad Choice).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000e7', '018f0008-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'irreversible_best_bad_choice'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000068', 'sg_climax_action', '4. Hành động Đỉnh điểm (The Climax Choice)', 'Nhân vật dứt khoát đưa ra quyết định hành động dưới áp lực lớn nhất.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000f0', '018f0008-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000069', 'sg_resolution_new_order', '5. Giải quyết & Thiết lập Trật tự Mới (Resolution)', 'Hậu quả của quyết định được bộc lộ, tạo ra một giá trị mới cho cảnh hoặc toàn truyện.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0008-0000-7000-8000-0000000000fa', '018f0008-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'value_shift_requirement'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-00000000012d', 'cmp_equilibrium_disruptor', 'Tác nhân Phá vỡ Cân bằng', 'Vụ án mạng xảy ra hoặc nhận được cuộc gọi tống tiền.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-00000000012e', 'cmp_unexpected_revelation', 'Tiết lộ Trái khoáy Bất ngờ', 'Manh mối chính biến thành bằng chứng ngoại phạm của nghi phạm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-00000000012f', 'cmp_dilemma_crucible', 'Lò nung Tiến thoái Lưỡng nan', 'Phải chọn cứu con tin hay giữ bảo mật quốc gia.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000130', 'cmp_committed_verdict', 'Hành vi Đột phá Dứt khoát', 'Lập tức nổ súng hoặc công bố tài liệu mật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0008-0000-7000-8000-000000000131', 'cmp_stabilized_aftermath', 'Hiện trạng Cân bằng Mới', 'Sự thật được định hình, kẻ phạm tội bị còng tay.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000191', 'emo_alert_intrigued', 'Cảnh giác Tò mò')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000192', 'emo_destabilized_shock', 'Chấn động Mất phương hướng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000193', 'emo_agonizing_dilemma', 'Giằng xé Khắc khoải')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000194', 'emo_resolute_execution', 'Kiên quyết Thực thi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0008-0000-7000-8000-000000000195', 'emo_calibrated_settlement', 'Định hình Lắng đọng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f5', 'eb_sg_incite', '018f0008-0000-7000-8000-00000000012d', '018f0008-0000-7000-8000-000000000065', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_alert_intrigued'), '018f0008-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f6', 'eb_sg_turn', '018f0008-0000-7000-8000-00000000012e', '018f0008-0000-7000-8000-000000000066', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_destabilized_shock'), '018f0008-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f7', 'eb_sg_crisis', '018f0008-0000-7000-8000-00000000012f', '018f0008-0000-7000-8000-000000000067', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_agonizing_dilemma'), '018f0008-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f8', 'eb_sg_climax', '018f0008-0000-7000-8000-000000000130', '018f0008-0000-7000-8000-000000000068', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_resolute_execution'), '018f0008-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0008-0000-7000-8000-0000000001f9', 'eb_sg_resolve', '018f0008-0000-7000-8000-000000000131', '018f0008-0000-7000-8000-000000000069', '018f0008-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0008-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_calibrated_settlement'), '018f0008-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002bd', 'Xung đột bùng nổ phức tạp', '018f0008-0000-7000-8000-0000000001f5', '018f0008-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002be', 'Đẩy vào bước đường cùng', '018f0008-0000-7000-8000-0000000001f6', '018f0008-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002bf', 'Quyết định sinh tử', '018f0008-0000-7000-8000-0000000001f7', '018f0008-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0008-0000-7000-8000-0000000002c0', 'Trật tự mới hình thành', '018f0008-0000-7000-8000-0000000001f8', '018f0008-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0008-0000-7000-8000-000000000321', 'sg_crisis_binary_nature', 'Khủng hoảng Crisis bắt buộc phải là một câu hỏi nhị phân rõ ràng đòi hỏi hành động lựa chọn cụ thể.', 'Ngăn ngừa việc câu chuyện trôi dạt vô định thiếu động lực nhân vật.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0008-0000-7000-8000-000000000385', 'story_grid_five_commandments_sg_crisis_binary_nature', (SELECT id FROM constraints WHERE code = 'sg_crisis_binary_nature'), '018f0008-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''story_grid_five_commandments'', ''sg_crisis_binary_nature'')', 'Khủng hoảng Crisis bắt buộc phải là một câu hỏi nhị phân rõ ràng đòi hỏi hành động lựa chọn cụ thể.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0008-0000-7000-8000-000000000322', 'sg_value_shift_nonzero', 'Trạng thái Resolution bắt buộc phải có giá trị sống/chết, danh dự/nhục nhã bị dịch chuyển so với Inciting Incident.', 'Đảm bảo mỗi đơn vị kịch bản đều tạo ra tiến trình thực chất cho toàn mạch tự sự.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0008-0000-7000-8000-000000000386', 'story_grid_five_commandments_sg_value_shift_nonzero', (SELECT id FROM constraints WHERE code = 'sg_value_shift_nonzero'), '018f0008-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''story_grid_five_commandments'', ''sg_value_shift_nonzero'')', 'Trạng thái Resolution bắt buộc phải có giá trị sống/chết, danh dự/nhục nhã bị dịch chuyển so với Inciting Incident.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Spirit Cultivation Genre: The Trash Underdog Trope](https://tvtropes.org/pmwiki/pmwiki.php/Main/SpiritCultivationGenre):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0009-0000-7000-8000-000000000001', 'phe_vat_nghich_tap_xianxia', 'Phế vật Nghịch tập (Underdog to Sovereign Trope)', 'plot', '1.0.0', true, 'Mô-típ cốt lõi của dòng Tiên hiệp/Huyền huyễn mạng Trung Quốc: Nhân vật chính ban đầu đan điền tổn hại, linh căn rác rưởi bị tông tộc khinh khi; nhờ kỳ ngộ bảo vật nghịch thiên, từng bước âm thầm tu luyện, vả mặt thiên kiêu và xưng bá vạn giới.', 50, 500);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0009-0000-7000-8000-000000000015', '018f0009-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xianxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0009-0000-7000-8000-000000000016', '018f0009-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xuanhuan'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0009-0000-7000-8000-000000000017', '018f0009-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'progression'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000029', 'face_slapping_retaliation_rule', 'Quy tắc vả mặt tương xứng với nhục nhã ban đầu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-00000000002a', 'realm_breakthrough_threshold', 'Quy tắc bình cảnh và vượt cấp khiêu chiến')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000065', 'pv_humiliated_cripple', '1. Phế căn & Ô nhục (Humiliated Trash Phase)', 'Đan điền bị phế, hôn thê hủy hôn, tộc nhân sỉ nhục, phụ thân chịu áp lực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000d2', '018f0009-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'face_slapping_retaliation_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000066', 'pv_fortuitous_relic_awakening', '2. Cơ duyên Nghịch thiên (Relic & Master Awakening)', 'Nhặt được nhẫn cổ chứa tàn hồn cường giả hoặc Thần thạch thượng cổ tái tạo kinh mạch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000dc', '018f0009-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'face_slapping_retaliation_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000067', 'pv_clandestine_grinding', '3. Ẩn nhẫn Khổ tu (Clandestine Grinding & Alchemy)', 'Ẩn giấu thực lực, học thuật luyện đan, rèn đúc thần thể vượt xa cực hạn thế tục.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000e7', '018f0009-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'realm_breakthrough_threshold'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000068', 'pv_clan_tournament_face_slap', '4. Đại bỉ Gia tộc Vả mặt (Clan Tournament Face-Slap)', 'Tung hoành trên lôi đài, một chiêu trấn áp kẻ từng khinh khi mình, đoạt vị trí quán quân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000f0', '018f0009-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'face_slapping_retaliation_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000069', 'pv_sect_genius_crushing', '5. Trấn áp Thiên kiêu Tông môn (Sect Sovereign Ascension)', 'Bước ra thế giới lớn, lấy yếu thắng mạnh vượt cấp khiêu chiến, mở ra con đường vô địch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0009-0000-7000-8000-0000000000fb', '018f0009-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'realm_breakthrough_threshold'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-00000000012d', 'cmp_engagement_annulment', 'Thư Hủy hôn & Lời thề 3 Năm', 'Thiếu nữ ngạo mạn tới cửa hủy ước gây nên vết nhơ danh dự gia tộc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-00000000012e', 'cmp_ancient_ring_resonance', 'Cổ giới Linh quang Thức tỉnh', 'Giọt máu nhỏ vào di vật đánh thức lão gia gia thông tuệ vạn cổ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-00000000012f', 'cmp_bone_marrow_cleansing', 'Tẩy tủy Phạt mao Tái tạo', 'Chịu đựng nỗi đau xé thịt nát xương để rèn nên Hỗn Độn Thể.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000130', 'cmp_shockwave_arena_strike', 'Kình phong Chấn động Lôi đài', 'Tung chiêu thức nghịch thiên khiến toàn trường há hốc kinh ngạc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0009-0000-7000-8000-000000000131', 'cmp_supreme_domain_unleash', 'Khai mở Lĩnh vực Chí tôn', 'Nghiền nát thần tử các đại giáo phái, đạp thiên kiêu dưới chân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000191', 'emo_bitter_humiliation', 'Nhục nhã Uất hận')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000192', 'emo_fierce_vindication_seed', 'Hạt mầm Phấn chấn Nghịch thiên')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000193', 'emo_iron_will_solitude', 'Cô độc Sắt đá')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000194', 'emo_ecstatic_face_slap_catharsis', 'Khoái cảm Vả mặt Sảng khoái')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0009-0000-7000-8000-000000000195', 'emo_arrogant_domination', 'Ngạo nghễ Độc tôn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f5', 'eb_pv_trash', '018f0009-0000-7000-8000-00000000012d', '018f0009-0000-7000-8000-000000000065', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_bitter_humiliation'), '018f0009-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f6', 'eb_pv_relic', '018f0009-0000-7000-8000-00000000012e', '018f0009-0000-7000-8000-000000000066', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_fierce_vindication_seed'), '018f0009-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f7', 'eb_pv_grind', '018f0009-0000-7000-8000-00000000012f', '018f0009-0000-7000-8000-000000000067', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_iron_will_solitude'), '018f0009-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f8', 'eb_pv_tourney', '018f0009-0000-7000-8000-000000000130', '018f0009-0000-7000-8000-000000000068', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_ecstatic_face_slap_catharsis'), '018f0009-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0009-0000-7000-8000-0000000001f9', 'eb_pv_ascend', '018f0009-0000-7000-8000-000000000131', '018f0009-0000-7000-8000-000000000069', '018f0009-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0009-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_arrogant_domination'), '018f0009-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002bd', 'Kỳ ngộ thức tỉnh giữa tuyệt lộ', '018f0009-0000-7000-8000-0000000001f5', '018f0009-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002be', 'Nuốt đắng bế quan tu luyện', '018f0009-0000-7000-8000-0000000001f6', '018f0009-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002bf', 'Xuất quan trấn áp lôi đài', '018f0009-0000-7000-8000-0000000001f7', '018f0009-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0009-0000-7000-8000-0000000002c0', 'Khiêu chiến thiên kiêu đại vực', '018f0009-0000-7000-8000-0000000001f8', '018f0009-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0009-0000-7000-8000-000000000321', 'pv_must_suffer_unjust_humiliation', 'Giai đoạn đầu bắt buộc phải có ít nhất một hành vi coi thường danh dự bất công (hủy hôn, đuổi khỏi gia tộc).', 'Tạo ra điện thế cảm xúc phẫn nộ làm đòn bẩy cho khoái cảm sảng văn ở giai đoạn sau.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0009-0000-7000-8000-000000000385', 'phe_vat_nghich_tap_xianxia_pv_must_suffer_unjust_humiliation', (SELECT id FROM constraints WHERE code = 'pv_must_suffer_unjust_humiliation'), '018f0009-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phe_vat_nghich_tap_xianxia'', ''pv_must_suffer_unjust_humiliation'')', 'Giai đoạn đầu bắt buộc phải có ít nhất một hành vi coi thường danh dự bất công (hủy hôn, đuổi khỏi gia tộc).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0009-0000-7000-8000-000000000322', 'pv_relic_exclusivity', 'Cơ duyên/Bảo vật ban đầu phải mang tính duy nhất và chỉ nhân vật chính mới kích hoạt được.', 'Khẳng định thiên mệnh và sự khác biệt tuyệt đối của nhân vật chính.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0009-0000-7000-8000-000000000386', 'phe_vat_nghich_tap_xianxia_pv_relic_exclusivity', (SELECT id FROM constraints WHERE code = 'pv_relic_exclusivity'), '018f0009-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phe_vat_nghich_tap_xianxia'', ''pv_relic_exclusivity'')', 'Cơ duyên/Bảo vật ban đầu phải mang tính duy nhất và chỉ nhân vật chính mới kích hoạt được.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - LitRPG: The Game System Progression Trope](https://tvtropes.org/pmwiki/pmwiki.php/Main/LitRPG):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000a-0000-7000-8000-000000000001', 'he_thong_luu_system_progression', 'Hệ thống lưu (System LitRPG Progression)', 'plot', '1.0.0', true, 'Mô-típ số hóa sức mạnh đặc trưng: Nhân vật chính thức tỉnh Giao diện Hệ thống độc quyền (Status Screen, Quest Log, Gacha, Skill Points); hoàn thành nhiệm vụ để nhận phần thưởng buff vượt cấp và thao túng quy tắc thế giới.', 30, 400);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000000015', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'litrpg'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000000016', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'di_nang'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000000017', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xuyen_khong'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000a-0000-7000-8000-000000001017', '018f000a-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'di_gioi'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000029', 'numerical_stat_consistency', 'Quy tắc tính nhất quán của bảng chỉ số số học')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-00000000002a', 'quest_penalty_urgency_rule', 'Quy tắc áp lực từ hình phạt nhiệm vụ thất bại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000065', 'sys_binding_awakening', '1. Thức tỉnh Ràng buộc Hệ thống (System Binding)', 'Tiếng ''Ding!'' báo hiệu kích hoạt bảng chỉ số giao diện nổi trước mắt giữa cơn nguy kịch.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000d2', '018f000a-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000066', 'sys_novice_quest_grinding', '2. Cày cuốc Nhiệm vụ Tân thủ (Novice Grinding)', 'Nhận nhiệm vụ hàng ngày: chống đẩy, tiêu diệt quái nhỏ, tích lũy điểm EXP và vàng ban đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000dc', '018f000a-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000067', 'sys_penalty_quest_crisis', '3. Khủng hoảng Nhiệm vụ Trừng phạt (Penalty Quest Threat)', 'Hệ thống đưa ra nhiệm vụ cưỡng chế với hình phạt trí mạng nếu không hoàn thành đúng hạn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000e7', '018f000a-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'quest_penalty_urgency_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000068', 'sys_gacha_breakthrough', '4. Quay thưởng Kỳ tích / Đột phá Kỹ năng (S-Rank Gacha Awakening)', 'Rút trúng kỹ năng cấp Thần độc nhất vô nhị (SSS-Rank Skill) đảo lộn toàn bộ logic trò chơi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000f0', '018f000a-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000069', 'sys_rule_transcendence', '5. Vượt thoát Quyền hạn Hệ thống (System Authority Transcendence)', 'Phát hiện bí mật đằng sau nguồn gốc của hệ thống và trở thành kẻ kiểm soát quy tắc tối cao.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000a-0000-7000-8000-0000000000fa', '018f000a-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'numerical_stat_consistency'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-00000000012d', 'cmp_status_screen_pop', 'Bảng Thuộc tính Hiện thị', 'Hiển thị HP, MP, Sức mạnh, Nhanh nhẹn và danh sách kỹ năng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-00000000012e', 'cmp_daily_routine_completion', 'Hoàn thành Lộ trình Thường nhật', 'Nhận hộp quà tân thủ và tăng nhẹ điểm số nền tảng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-00000000012f', 'cmp_ticking_countdown_hazard', 'Đồng hồ Đếm ngược Trừng phạt', 'Cảnh báo dịch chuyển vào sa mạc tử thần nếu dám lười biếng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000130', 'cmp_golden_roulette_spark', 'Vòng quay Hoàng kim Phát sáng', 'Nhận được skill bị động phá vỡ giới hạn cấp độ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000a-0000-7000-8000-000000000131', 'cmp_admin_privilege_seizure', 'Đoạt Quyền Quản trị viên', 'Bẻ khóa mã nguồn thế giới, tự do điều chỉnh tham số vạn vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000191', 'emo_disbelieving_shock', 'Kinh ngạc Hoài nghi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000192', 'emo_dopamine_level_up_rush', 'Hưng phấn Dopamine Tăng cấp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000193', 'emo_cold_sweat_urgency', 'Đổ mồ hôi hột Nguy cấp')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000194', 'emo_lottery_jackpot_ecstasy', 'Vỡ òa Trúng số Độc đắc')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000a-0000-7000-8000-000000000195', 'emo_godlike_omnipotence', 'Tự tin Thần thánh')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f5', 'eb_sys_awaken', '018f000a-0000-7000-8000-00000000012d', '018f000a-0000-7000-8000-000000000065', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_disbelieving_shock'), '018f000a-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f6', 'eb_sys_grind', '018f000a-0000-7000-8000-00000000012e', '018f000a-0000-7000-8000-000000000066', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_dopamine_level_up_rush'), '018f000a-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f7', 'eb_sys_penalty', '018f000a-0000-7000-8000-00000000012f', '018f000a-0000-7000-8000-000000000067', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_cold_sweat_urgency'), '018f000a-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f8', 'eb_sys_gacha', '018f000a-0000-7000-8000-000000000130', '018f000a-0000-7000-8000-000000000068', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_lottery_jackpot_ecstasy'), '018f000a-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000a-0000-7000-8000-0000000001f9', 'eb_sys_transcend', '018f000a-0000-7000-8000-000000000131', '018f000a-0000-7000-8000-000000000069', '018f000a-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000a-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_godlike_omnipotence'), '018f000a-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002bd', 'Khai mở giao diện cày cấp', '018f000a-0000-7000-8000-0000000001f5', '018f000a-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002be', 'Hình phạt treo đầu súng', '018f000a-0000-7000-8000-0000000001f6', '018f000a-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002bf', 'Phần thưởng thần thoại đảo chiều', '018f000a-0000-7000-8000-0000000001f7', '018f000a-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000a-0000-7000-8000-0000000002c0', 'Xâm nhập căn nguyên hệ thống', '018f000a-0000-7000-8000-0000000001f8', '018f000a-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000a-0000-7000-8000-000000000321', 'sys_stat_progression_logic', 'Chỉ số sức mạnh khi tăng phải có tác động định lượng cụ thể lên chiến đấu (tốc độ, sát thương).', 'Duy trì tính thuyết phục của dòng LitRPG đối với độc giả yêu thích dữ liệu.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000a-0000-7000-8000-000000000385', 'he_thong_luu_system_progression_sys_stat_progression_logic', (SELECT id FROM constraints WHERE code = 'sys_stat_progression_logic'), '018f000a-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''he_thong_luu_system_progression'', ''sys_stat_progression_logic'')', 'Chỉ số sức mạnh khi tăng phải có tác động định lượng cụ thể lên chiến đấu (tốc độ, sát thương).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000a-0000-7000-8000-000000000322', 'sys_limit_on_exploit', 'Lỗ hổng hệ thống (exploit) phải đi kèm nguy cơ bị thực thể tạo ra hệ thống truy lùng thanh trừ.', 'Tạo kịch tính và mâu thuẫn lớn hơn ở giai đoạn cuối truyện.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000a-0000-7000-8000-000000000386', 'he_thong_luu_system_progression_sys_limit_on_exploit', (SELECT id FROM constraints WHERE code = 'sys_limit_on_exploit'), '018f000a-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''he_thong_luu_system_progression'', ''sys_limit_on_exploit'')', 'Lỗ hổng hệ thống (exploit) phải đi kèm nguy cơ bị thực thể tạo ra hệ thống truy lùng thanh trừ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Peggy Sue: Second Chance Time Regression & Revenge Trope](https://tvtropes.org/pmwiki/pmwiki.php/Main/PeggySue):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000b-0000-7000-8000-000000000001', 'trong_sinh_bao_thu_regression', 'Trọng sinh Báo thù (Time Regression Revenge Trope)', 'plot', '1.0.0', true, 'Nhân vật kiếp trước tin lầm trà xanh, cặn bã khiến gia tộc tan nát, chết trong uất hận; được trời cho tái sinh về thời điểm then chốt trước khi bi kịch xảy ra, nắm giữ ký ức tương lai để chặn đứng âm mưu và trả thù từng kẻ một.', 35, 250);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000b-0000-7000-8000-000000000015', '018f000b-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'trong_sinh'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000b-0000-7000-8000-000000001015', '018f000b-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'romance'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000b-0000-7000-8000-000000000017', '018f000b-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mat_the'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000029', 'future_memory_foreknowledge_rule', 'Quy tắc sử dụng ký ức tương lai làm vũ khí chiến lược')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-00000000002a', 'butterfly_effect_divergence', 'Quy tắc hiệu ứng cánh bướm làm biến đổi sự kiện gốc')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000065', 'ts_betrayal_and_agony', '1. Kiếp trước Bi thảm & Uất hận (Tragic Past Life Death)', 'Chứng kiến người thân bị sát hại, bản thân bị độc chết hoặc phế truất trong lãnh cung.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000d2', '018f000b-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000066', 'ts_awakening_in_youth', '2. Tỉnh giấc Thời niên thiếu (Rebirth at Crossroads)', 'Mở mắt thấy mình năm 16 tuổi, ngay trước ngày lễ đính ước hoặc ngày xảy ra biến cố khởi đầu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000dc', '018f000b-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000067', 'ts_preemptive_entrapment', '3. Bố trí Cạm bẫy Phản đòn (Preemptive Scheme Unraveling)', 'Giả vờ ngây thơ, gậy ông đập lưng ông, khiến kẻ hãm hại tự rơi vào chính cái bẫy của chúng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000e6', '018f000b-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000068', 'ts_stripping_enemy_power', '4. Cắt đứt Vây cánh & Chỗ dựa (Dismantling Enemy Faction)', 'Tước đoạt tài sản, địa vị, bóc trần bộ mặt đạo đức giả của kẻ thù trước toàn thiên hạ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000f1', '018f000b-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'butterfly_effect_divergence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000069', 'ts_rewrite_destiny_peace', '5. Viết lại Vận mệnh & Bình yên (Destiny Rewritten & Catharsis)', 'Bảo vệ trọn vẹn gia đình, kẻ thù nhận bản án thích đáng, bản thân tìm được hạnh phúc đích thực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000b-0000-7000-8000-0000000000fa', '018f000b-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'future_memory_foreknowledge_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-00000000012d', 'cmp_blood_soaked_reminiscence', 'Ký ức Máu Lãnh cung', 'Lời trăn trối nguyền rủa kẻ phản trắc trước khi nhắm mắt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-00000000012e', 'cmp_mirror_youth_reflection', 'Soi gương Nhận ra Thanh xuân', 'Chạm vào làn da chưa vết sẹo, xác định mốc thời gian trùng sinh.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-00000000012f', 'cmp_poison_wine_switch', 'Hoán đổi Chén rượu Độc', 'Bắt quả tang tại trận kẻ hạ thủ trong đại tiệc gia đình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000130', 'cmp_ledger_fraud_exposure', 'Tung Sổ sách Tham ô Công khai', 'Triệt hạ điểm tựa quyền lực của nhị phòng hay gian thần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000b-0000-7000-8000-000000000131', 'cmp_triumphant_wedding_reign', 'Đại lễ Đăng quang / Hôn lễ Viên mãn', 'Nắm tay đồng minh đích thực nhìn non sông tươi đẹp.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000191', 'emo_bloodcurdling_hatred', 'Oán hận Ngút trời')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000192', 'emo_shivering_disbelief', 'Bàng hoàng Run rẩy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000193', 'emo_icy_calculated_malice', 'Lạnh lùng Toan tính')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000194', 'emo_savory_retribution', 'Thỏa mãn Báo thù')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000b-0000-7000-8000-000000000195', 'emo_profound_tranquility', 'Thanh thản Bình an')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f5', 'eb_ts_past', '018f000b-0000-7000-8000-00000000012d', '018f000b-0000-7000-8000-000000000065', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_bloodcurdling_hatred'), '018f000b-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f6', 'eb_ts_awaken', '018f000b-0000-7000-8000-00000000012e', '018f000b-0000-7000-8000-000000000066', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_shivering_disbelief'), '018f000b-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f7', 'eb_ts_trap', '018f000b-0000-7000-8000-00000000012f', '018f000b-0000-7000-8000-000000000067', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_icy_calculated_malice'), '018f000b-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f8', 'eb_ts_strip', '018f000b-0000-7000-8000-000000000130', '018f000b-0000-7000-8000-000000000068', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_savory_retribution'), '018f000b-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000b-0000-7000-8000-0000000001f9', 'eb_ts_peace', '018f000b-0000-7000-8000-000000000131', '018f000b-0000-7000-8000-000000000069', '018f000b-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000b-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_profound_tranquility'), '018f000b-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002bd', 'Mở mắt hồi quy thời niên thiếu', '018f000b-0000-7000-8000-0000000001f5', '018f000b-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002be', 'Lập mưu phản kích kẻ ác', '018f000b-0000-7000-8000-0000000001f6', '018f000b-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002bf', 'Đập tan vây cánh tập đoàn đối địch', '018f000b-0000-7000-8000-0000000001f7', '018f000b-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000b-0000-7000-8000-0000000002c0', 'Khép lại oán thù xây đắp tương lai', '018f000b-0000-7000-8000-0000000001f8', '018f000b-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000b-0000-7000-8000-000000000321', 'ts_butterfly_divergence_complication', 'Hành động can thiệp của nhân vật chính bắt buộc phải tạo ra biến số mới ngoài ký ức kiếp trước.', 'Tránh cốt truyện bị một màu dễ dãi, duy trì sự bất ngờ kịch tính.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000b-0000-7000-8000-000000000385', 'trong_sinh_bao_thu_regression_ts_butterfly_divergence_complication', (SELECT id FROM constraints WHERE code = 'ts_butterfly_divergence_complication'), '018f000b-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''trong_sinh_bao_thu_regression'', ''ts_butterfly_divergence_complication'')', 'Hành động can thiệp của nhân vật chính bắt buộc phải tạo ra biến số mới ngoài ký ức kiếp trước.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000b-0000-7000-8000-000000000322', 'ts_no_premature_forgiveness', 'Nhân vật chính tuyệt đối không được tha thứ cho kẻ thù kiếp trước chỉ vì kiếp này chúng chưa kịp ra tay.', 'Giữ vững cảm xúc chính thống của thể loại sảng khoái phục thù.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000b-0000-7000-8000-000000000386', 'trong_sinh_bao_thu_regression_ts_no_premature_forgiveness', (SELECT id FROM constraints WHERE code = 'ts_no_premature_forgiveness'), '018f000b-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''trong_sinh_bao_thu_regression'', ''ts_no_premature_forgiveness'')', 'Nhân vật chính tuyệt đối không được tha thứ cho kẻ thù kiếp trước chỉ vì kiếp này chúng chưa kịp ra tay.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Helping Writers Become Authors - How to Write Character Arcs: Positive Change and Flat Arcs](https://www.helpingwritersbecomeauthors.com/write-character-arcs/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000c-0000-7000-8000-000000000001', 'positive_change_character_arc', 'Cung Phát triển Tính cách Tích cực (Positive Change Arc)', 'character_arc', '1.0.0', true, 'Lý thuyết cốt lõi của K.M. Weiland về cung nhân vật tích cực: Nhân vật khởi đầu bằng niềm tin mù quáng vào Ngụy ngôn (The Lie). Qua thử thách, họ nhận ra Sự thật (The Truth), buông bỏ cái Tôi vị kỷ (Want) để ôm trọn Nhu cầu đích thực (Need).', 20, 100);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000c-0000-7000-8000-000000000015', '018f000c-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'literary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000c-0000-7000-8000-000000000016', '018f000c-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'contemporary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000c-0000-7000-8000-000000000017', '018f000c-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000029', 'lie_vs_truth_internal_dialectic', 'Quy tắc biện chứng nội tâm giữa Ngụy ngôn và Chân lý')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-00000000002a', 'want_subjugation_to_need', 'Quy tắc hy sinh điều nhân vật Muốn để đạt được điều nhân vật Cần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000065', 'pca_living_the_lie', '1. Sống trong Ngụy ngôn (Living the Lie)', 'Nhân vật tin vào một quan niệm sai lầm về bản thân hoặc thế giới để tự bảo vệ vết thương cũ (The Ghost).')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000d2', '018f000c-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'lie_vs_truth_internal_dialectic'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000066', 'pca_first_glimpse_of_truth', '2. Thoáng thấy Chân lý (First Glimpse of the Truth)', 'Gặp gỡ một tấm gương hoặc trải nghiệm thách thức niềm tin cũ, hé mở con đường mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000dc', '018f000c-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'lie_vs_truth_internal_dialectic'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000067', 'pca_clinging_to_comfort_lie', '3. Bám chấp Vỏ ốc Ngụy ngôn (Resisting Truth & Clinging to Lie)', 'Khi áp lực tăng cao, nhân vật cố chấp dùng lại thói quen cũ nhưng thất bại thảm hại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000e7', '018f000c-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'want_subjugation_to_need'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000068', 'pca_surrendering_the_want', '4. Buông bỏ Tham vọng Vị kỷ (Surrendering the Want for the Need)', 'Khoảnh khắc nhận thức đau đớn: chấp nhận từ bỏ mục tiêu ích kỷ để bảo vệ giá trị cao đẹp.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000f1', '018f000c-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'want_subjugation_to_need'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000069', 'pca_embodying_the_truth', '5. Hiện thân của Chân lý (Embodying the Truth in Climax)', 'Vượt qua thử thách tối thượng nhờ sức mạnh của sự thật, hoàn thiện nhân cách trọn vẹn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000c-0000-7000-8000-0000000000fa', '018f000c-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'lie_vs_truth_internal_dialectic'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-00000000012d', 'cmp_defensive_flaw_display', 'Bộc lộ Khiếm khuyết Phòng thủ', 'Nhân vật đẩy người khác ra xa vì sợ bị tổn thương lần nữa.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-00000000012e', 'cmp_unsettling_altruism_witness', 'Chứng kiến Hành vi Cao thượng', 'Một người bạn hy sinh vô điều kiện khiến nhân vật lung lay.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-00000000012f', 'cmp_catastrophic_lie_failure', 'Thất bại Thảm hại của Ngụy ngôn', 'Kế hoạch dựa trên sự dối trá sụp đổ làm tổn thương đồng đội.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000130', 'cmp_tearful_ego_sacrifice', 'Hiến tế Cái tôi trong Nước mắt', 'Tự tay ném bỏ vương miện hoặc danh vọng hão huyền.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000c-0000-7000-8000-000000000131', 'cmp_integrated_courage_triumph', 'Chiến thắng Từ Tâm thế Hợp nhất', 'Hành động vì người khác mang lại sức mạnh bất khả chiến bại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000191', 'emo_guarded_cynicism', 'Hoài nghi Đề phòng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000192', 'emo_flicker_of_hope', 'Tia sáng Hy vọng Mong manh')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000193', 'emo_defensive_stubbornness', 'Cố chấp Ngoan cố')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000194', 'emo_heartbreaking_catharsis', 'Nghẹn ngào Giác ngộ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000c-0000-7000-8000-000000000195', 'emo_sovereign_wholeness', 'Toàn vẹn Tự do')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f5', 'eb_pca_lie', '018f000c-0000-7000-8000-00000000012d', '018f000c-0000-7000-8000-000000000065', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_guarded_cynicism'), '018f000c-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f6', 'eb_pca_glimpse', '018f000c-0000-7000-8000-00000000012e', '018f000c-0000-7000-8000-000000000066', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_flicker_of_hope'), '018f000c-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f7', 'eb_pca_resist', '018f000c-0000-7000-8000-00000000012f', '018f000c-0000-7000-8000-000000000067', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_defensive_stubbornness'), '018f000c-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f8', 'eb_pca_surrender', '018f000c-0000-7000-8000-000000000130', '018f000c-0000-7000-8000-000000000068', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_heartbreaking_catharsis'), '018f000c-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000c-0000-7000-8000-0000000001f9', 'eb_pca_truth', '018f000c-0000-7000-8000-000000000131', '018f000c-0000-7000-8000-000000000069', '018f000c-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000c-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_sovereign_wholeness'), '018f000c-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002bd', 'Hé mở chân trời mới', '018f000c-0000-7000-8000-0000000001f5', '018f000c-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002be', 'Khủng hoảng níu kéo ảo ảnh', '018f000c-0000-7000-8000-0000000001f6', '018f000c-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002bf', 'Buông tay đầu hàng sự thật', '018f000c-0000-7000-8000-0000000001f7', '018f000c-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000c-0000-7000-8000-0000000002c0', 'Trỗi dậy với tâm thức hoàn thiện', '018f000c-0000-7000-8000-0000000001f8', '018f000c-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000c-0000-7000-8000-000000000321', 'pca_ghost_wound_necessity', 'Ngụy ngôn (The Lie) phải bắt nguồn từ một vết thương tâm lý quá khứ (The Ghost/Wound) cụ thể.', 'Giữ tính logic tâm lý chiều sâu, giải thích lý do nhân vật bảo thủ.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000c-0000-7000-8000-000000000385', 'positive_change_character_arc_pca_ghost_wound_necessity', (SELECT id FROM constraints WHERE code = 'pca_ghost_wound_necessity'), '018f000c-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''positive_change_character_arc'', ''pca_ghost_wound_necessity'')', 'Ngụy ngôn (The Lie) phải bắt nguồn từ một vết thương tâm lý quá khứ (The Ghost/Wound) cụ thể.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000c-0000-7000-8000-000000000322', 'pca_need_trumps_want', 'Tại cao trào, chiến thắng chỉ đạt được khi nhân vật ưu tiên Nhu cầu tinh thần (Need) hơn Tham muốn vật chất (Want).', 'Bản chất định nghĩa cung phát triển nhân vật tích cực của K.M. Weiland.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000c-0000-7000-8000-000000000386', 'positive_change_character_arc_pca_need_trumps_want', (SELECT id FROM constraints WHERE code = 'pca_need_trumps_want'), '018f000c-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''positive_change_character_arc'', ''pca_need_trumps_want'')', 'Tại cao trào, chiến thắng chỉ đạt được khi nhân vật ưu tiên Nhu cầu tinh thần (Need) hơn Tham muốn vật chất (Want).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Helping Writers Become Authors - How to Write a Negative Character Arc: Disillusionment, Fall, and Corruption](https://www.helpingwritersbecomeauthors.com/negative-character-arc-1/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000d-0000-7000-8000-000000000001', 'corruption_negative_character_arc', 'Cung Nhân vật Tha hóa / Hắc hóa (The Corruption Arc)', 'character_arc', '1.0.0', true, 'Bi kịch tha hóa đạo đức kinh điển: Nhân vật ban đầu nhận biết Sự thật (The Truth) nhưng trước áp lực tàn khốc và cám dỗ quyền lực, họ dần thỏa hiệp, ôm lấy Ngụy ngôn (The Lie) và biến chất thành kẻ phản diện đáng sợ nhất.', 25, 110);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000d-0000-7000-8000-000000000015', '018f000d-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'dark'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000d-0000-7000-8000-000000001016', '018f000d-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'psychological'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000029', 'moral_rationalization_progression', 'Quy tắc hợp lý hóa tội lỗi từng bậc thang')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-00000000002a', 'abyss_irreversibility_point', 'Quy tắc điểm không thể quay đầu của sự tha hóa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000065', 'cna_noble_idealism', '1. Lý tưởng Cao thượng Sơ khởi (Noble Idealism & Known Truth)', 'Nhân vật kiên định bảo vệ chính nghĩa, tin vào lòng tốt và các giá trị đạo đức mẫu mực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000d2', '018f000d-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'moral_rationalization_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000066', 'cna_impotence_against_evil', '2. Bất lực trước Cường quyền / Bi kịch (Impotence in Crisis)', 'Quy tắc chính trực bị kẻ thủ ác chà đạp; chứng kiến người thương yêu chịu đau đớn mà luật lệ không cứu được.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000dc', '018f000d-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'moral_rationalization_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000067', 'cna_first_dark_compromise', '3. Thỏa hiệp Đen tối Đầu tiên (First Moral Compromise)', 'Sử dụng biện pháp tàn bạo vì ''mục đích tốt đẹp''; bắt đầu tin rằng ''mục đích biện minh cho phương tiện''.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000e6', '018f000d-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'moral_rationalization_progression'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000068', 'cna_point_of_no_return_atrocity', '4. Điểm Vực thẳm / Tội ác Không thể Tha thứ (Crossing Point of No Return)', 'Hiến tế đồng đội cũ hoặc phản bội lý tưởng cốt lõi để đổi lấy quyền năng tuyệt đối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000f1', '018f000d-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'abyss_irreversibility_point'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000069', 'cna_complete_tyrannical_decay', '5. Tha hóa Toàn diện / Trở thành Bạo chúa (Complete Decay into Villainy)', 'Hoàn toàn bị nuốt chửng bởi ảo tưởng kiểm soát; tàn sát bất kỳ ai cản đường nhân danh trật tự mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000d-0000-7000-8000-0000000000fb', '018f000d-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'abyss_irreversibility_point'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-00000000012d', 'cmp_shining_banner_stand', 'Lời thề Dưới Quân kỳ', 'Khẳng định danh dự hiệp sĩ và công lý tuyệt đối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-00000000012e', 'cmp_unpunished_atrocity_witness', 'Chứng kiến Ác nhân Vô tội vạ', 'Hệ thống pháp luật thối nát tha bổng kẻ thủ ác tàn nhẫn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-00000000012f', 'cmp_forbidden_art_usage', 'Khai triển Cấm thuật Cứu viện', 'Hút linh hồn kẻ thù để bảo vệ thành trì.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000130', 'cmp_slaying_the_moral_anchor', 'Hạ sát Điểm tựa Đạo đức', 'Tự tay sát hại người thầy hoặc người bạn từng khuyên can mình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000d-0000-7000-8000-000000000131', 'cmp_obsidian_throne_coronation', 'Đăng cơ trên Ngai vàng Hắc ám', 'Khoác lên mình áo choàng bạo chúa với đôi mắt lạnh lùng vô cảm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000191', 'emo_pure_righteousness', 'Chính nghĩa Trong sáng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000192', 'emo_burning_helplessness', 'Bất lực Bùng cháy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000193', 'emo_sinister_rationalization', 'Biện minh U ám')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000194', 'emo_shattered_remorse', 'Hối hận Vỡ vụn Câm lặng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000d-0000-7000-8000-000000000195', 'emo_chilling_megalomania', 'Cuồng vọng Lạnh giá')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f5', 'eb_cna_noble', '018f000d-0000-7000-8000-00000000012d', '018f000d-0000-7000-8000-000000000065', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_pure_righteousness'), '018f000d-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f6', 'eb_cna_impotent', '018f000d-0000-7000-8000-00000000012e', '018f000d-0000-7000-8000-000000000066', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_burning_helplessness'), '018f000d-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f7', 'eb_cna_compromise', '018f000d-0000-7000-8000-00000000012f', '018f000d-0000-7000-8000-000000000067', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_sinister_rationalization'), '018f000d-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f8', 'eb_cna_abyss', '018f000d-0000-7000-8000-000000000130', '018f000d-0000-7000-8000-000000000068', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_shattered_remorse'), '018f000d-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000d-0000-7000-8000-0000000001f9', 'eb_cna_decay', '018f000d-0000-7000-8000-000000000131', '018f000d-0000-7000-8000-000000000069', '018f000d-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000d-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_chilling_megalomania'), '018f000d-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002bd', 'Vỡ mộng trước hiện thực tàn khốc', '018f000d-0000-7000-8000-0000000001f5', '018f000d-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002be', 'Bước qua lằn ranh đạo đức đầu tiên', '018f000d-0000-7000-8000-0000000001f6', '018f000d-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002bf', 'Sa lầy vào tội ác không thể tha thứ', '018f000d-0000-7000-8000-0000000001f7', '018f000d-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000d-0000-7000-8000-0000000002c0', 'Biến chất thành ma đầu tàn bạo', '018f000d-0000-7000-8000-0000000001f8', '018f000d-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000d-0000-7000-8000-000000000321', 'cna_sympathetic_initial_intent', 'Động cơ ban đầu của nhân vật bắt buộc phải mang tính cao đẹp và vì người khác (cứu gia đình, chấm dứt chiến tranh).', 'Bi kịch tha hóa chỉ có sức lay động khi người đọc cảm thương cho lý do vấp ngã.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000d-0000-7000-8000-000000000385', 'corruption_negative_character_arc_cna_sympathetic_initial_intent', (SELECT id FROM constraints WHERE code = 'cna_sympathetic_initial_intent'), '018f000d-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''corruption_negative_character_arc'', ''cna_sympathetic_initial_intent'')', 'Động cơ ban đầu của nhân vật bắt buộc phải mang tính cao đẹp và vì người khác (cứu gia đình, chấm dứt chiến tranh).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000d-0000-7000-8000-000000000322', 'cna_severing_moral_anchor', 'Trước khi hắc hóa hoàn toàn, nhân vật phải cắt đứt hoặc sát hại người đóng vai trò là mỏ neo đạo đức (Moral Anchor).', 'Đánh dấu sự sụp đổ vĩnh viễn của tiếng nói lương tâm.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000d-0000-7000-8000-000000000386', 'corruption_negative_character_arc_cna_severing_moral_anchor', (SELECT id FROM constraints WHERE code = 'cna_severing_moral_anchor'), '018f000d-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''corruption_negative_character_arc'', ''cna_severing_moral_anchor'')', 'Trước khi hắc hóa hoàn toàn, nhân vật phải cắt đứt hoặc sát hại người đóng vai trò là mỏ neo đạo đức (Moral Anchor).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [AI Write Book - Enemies to Lovers: Beat Sheet, Tension Pacing and Emotional Trajectory](https://aiwritebook.com/en/blog/enemies-to-lovers-guide/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000e-0000-7000-8000-000000000001', 'enemies_to_lovers_progression', 'Từ Kẻ thù hóa Người yêu (Enemies to Lovers Dynamic)', 'relationship', '1.0.0', true, 'Mô-típ lãng mạn kinh điển: Hai nhân vật xuất phát điểm ở hai đầu chiến tuyến, khinh ghét lẫn nhau; bị hoàn cảnh cưỡng chế ép buộc hợp tác (Forced Proximity), dần bóc trần điểm yếu nội tâm, biến thù hận thành khao khát nồng cháy và tình yêu bất diệt.', 20, 90);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000e-0000-7000-8000-000000000015', '018f000e-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'romance'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000029', 'forced_proximity_catalyst_rule', 'Quy tắc không gian cưỡng chế buộc phải tiếp xúc thân mật')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-00000000002a', 'vulnerability_revelation_sequence', 'Quy tắc hé lộ tổn thương trước khi nảy sinh rung cảm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000065', 'e2l_initial_hostility_clash', '1. Thù địch Đối đầu Tuyệt đối (Initial Mutual Hostility)', 'Coi nhau như cái gai trong mắt, đấu khẩu gay gắt, sẵn sàng rút kiếm hoặc triệt hạ nhau trong công việc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000d2', '018f000e-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'forced_proximity_catalyst_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000066', 'e2l_forced_proximity_containment', '2. Cưỡng chế Giam cầm / Chung chiến hào (Forced Proximity)', 'Bị nhốt chung hầm ngục, lạc trong bão tuyết, hoặc bị ép kết hôn chính trị; không thể tránh né nhau.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000dc', '018f000e-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'forced_proximity_catalyst_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000067', 'e2l_unmasking_vulnerability', '3. Hé lộ Điểm yếu & Tôn trọng Miễn cưỡng (Unmasking the Wound)', 'Bắt gặp đối phương băng bó vết thương trong đêm, thấu hiểu bi kịch đằng sau lớp vỏ sắc nhọn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000e7', '018f000e-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000068', 'e2l_smoldering_romantic_tension', '4. Điện thế Lãng mạn Bùng cháy (Smoldering Tension & First Kiss)', 'Sự va chạm thể xác vô tình khơi dậy đam mê ngột ngạt; nụ hôn đầu tiên vừa cuồng nhiệt vừa phòng thủ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000f1', '018f000e-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000069', 'e2l_apparent_betrayal_crisis', '5. Khủng hoảng Hiểu lầm Phản trắc (The Apparent Betrayal / Dark Night)', 'Một bí mật cũ bị khui ra khiến một bên ngỡ rằng mình bị lợi dụng; thù hận cũ bùng phát trở lại.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-0000000000fb', '018f000e-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000006a', 'e2l_all_in_vow_union', '6. Thú nhận Chân tình & Liên minh Vĩnh hằng (Grand Gesture & Union)', 'Sẵn sàng hy sinh sinh mạng để chứng minh tình cảm, phá vỡ định kiến hai phe, gắn kết trọn đời.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000e-0000-7000-8000-000000000105', '018f000e-0000-7000-8000-00000000006a', (SELECT id FROM state_rules WHERE code = 'vulnerability_revelation_sequence'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000012d', 'cmp_blade_to_throat_dialogue', 'Kề Kiếm vào Cổ Đấu khẩu', 'Cuộc chạm trán nảy lửa với ánh mắt tóe lửa căm hờn.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000012e', 'cmp_single_bed_storm_shelter', 'Trú bão Chỉ có Một Giường', 'Hoàn cảnh éo le buộc hai người phải chia sẻ thân nhiệt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-00000000012f', 'cmp_tending_battle_wounds', 'Băng bó Vết thương Đêm khuya', 'Những ngón tay run rẩy lau sạch vết máu trên lưng trần.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000130', 'cmp_furious_breathless_kiss', 'Nụ hôn Giận dữ Cuồng nhiệt', 'Không thể kìm nén khao khát sau khi cùng thoát hiểm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000131', 'cmp_stolen_military_map_blame', 'Nghi án Bản đồ Bị đánh cắp', 'Vật chứng giả mạo tố cáo đối phương là nội gián.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000e-0000-7000-8000-000000000132', 'cmp_defying_both_armies_declaration', 'Tuyên ngôn Tình yêu Trước Hai Trận tuyến', 'Đứng chắn trước mũi tên của đồng minh để bảo vệ người thương.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000191', 'emo_vitriolic_scorn', 'Khinh bỉ Cay độc')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000192', 'emo_claustrophobic_friction', 'Bức bối Ngột ngạt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000193', 'emo_reluctant_compassion', 'Thương cảm Miễn cưỡng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000194', 'emo_electric_yearning', 'Rạo rực Bỏng cháy')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000195', 'emo_devastating_grief', 'Tan vỡ Đau đớn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000e-0000-7000-8000-000000000196', 'emo_unshakable_devotion', 'Thủy chung Bất diệt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f5', 'eb_e2l_hostile', '018f000e-0000-7000-8000-00000000012d', '018f000e-0000-7000-8000-000000000065', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_vitriolic_scorn'), '018f000e-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f6', 'eb_e2l_proximity', '018f000e-0000-7000-8000-00000000012e', '018f000e-0000-7000-8000-000000000066', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_claustrophobic_friction'), '018f000e-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f7', 'eb_e2l_wound', '018f000e-0000-7000-8000-00000000012f', '018f000e-0000-7000-8000-000000000067', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_reluctant_compassion'), '018f000e-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f8', 'eb_e2l_kiss', '018f000e-0000-7000-8000-000000000130', '018f000e-0000-7000-8000-000000000068', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_electric_yearning'), '018f000e-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001f9', 'eb_e2l_betray', '018f000e-0000-7000-8000-000000000131', '018f000e-0000-7000-8000-000000000069', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_devastating_grief'), '018f000e-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000e-0000-7000-8000-0000000001fa', 'eb_e2l_union', '018f000e-0000-7000-8000-000000000132', '018f000e-0000-7000-8000-00000000006a', '018f000e-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000e-0000-7000-8000-000000000299', (SELECT id FROM emotional_ranges WHERE code = 'emo_unshakable_devotion'), '018f000e-0000-7000-8000-0000000001fa')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002bd', 'Dồn vào tình thế kẹt chung', '018f000e-0000-7000-8000-0000000001f5', '018f000e-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002be', 'Lột bỏ lớp giáp phòng ngự', '018f000e-0000-7000-8000-0000000001f6', '018f000e-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002bf', 'Bùng nổ ham muốn ngột ngạt', '018f000e-0000-7000-8000-0000000001f7', '018f000e-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002c0', 'Sụp đổ vì nghi án phản bội', '018f000e-0000-7000-8000-0000000001f8', '018f000e-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000e-0000-7000-8000-0000000002c1', 'Minh oan và thề nguyện trọn kiếp', '018f000e-0000-7000-8000-0000000001f9', '018f000e-0000-7000-8000-0000000001fa', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000e-0000-7000-8000-000000000321', 'e2l_real_initial_conflict', 'Xung đột ban đầu giữa hai nhân vật phải là xung đột thực tế có căn nguyên chính đáng (lập trường, gia tộc), không được là hiểu lầm hời hợt ngớ ngẩn.', 'Sức hấp dẫn của mô-típ phụ thuộc vào độ cao của bức tường ngăn cách ban đầu.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000e-0000-7000-8000-000000000385', 'enemies_to_lovers_progression_e2l_real_initial_conflict', (SELECT id FROM constraints WHERE code = 'e2l_real_initial_conflict'), '018f000e-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''enemies_to_lovers_progression'', ''e2l_real_initial_conflict'')', 'Xung đột ban đầu giữa hai nhân vật phải là xung đột thực tế có căn nguyên chính đáng (lập trường, gia tộc), không được là hiểu lầm hời hợt ngớ ngẩn.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000e-0000-7000-8000-000000000322', 'e2l_no_unearned_kiss', 'Nụ hôn đầu tiên không được diễn ra trước khi hai bên đã chứng kiến ít nhất một khía cạnh yếu mềm hoặc danh dự của đối phương.', 'Ngăn chặn tình yêu bị chê là nông nổi, hời hợt.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000e-0000-7000-8000-000000000386', 'enemies_to_lovers_progression_e2l_no_unearned_kiss', (SELECT id FROM constraints WHERE code = 'e2l_no_unearned_kiss'), '018f000e-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''enemies_to_lovers_progression'', ''e2l_no_unearned_kiss'')', 'Nụ hôn đầu tiên không được diễn ra trước khi hai bên đã chứng kiến ít nhất một khía cạnh yếu mềm hoặc danh dự của đối phương.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Mentor Archetype & Master-Apprentice Bond](https://tvtropes.org/pmwiki/pmwiki.php/Main/MentorArchetype):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f000f-0000-7000-8000-000000000001', 'mentor_protege_dynamic', 'Mối quan hệ Tôn sư & Đồ đệ (Mentor-Protégé Dynamic)', 'relationship', '1.0.0', true, 'Mô-típ gắn kết giữa bậc tiền bối thông tuệ và đệ tử kế nghiệp: Từ giai đoạn nhận đệ tử ngỗ ngược, khổ luyện truyền thừa, xung đột tư tưởng về giới hạn quyền năng, cho đến bi kịch hy sinh của người thầy và sự vượt ngưỡng của người học trò.', 25, 150);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000000015', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'wuxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000000016', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000000017', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'shounen'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f000f-0000-7000-8000-000000001017', '018f000f-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'sports'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000029', 'torch_passing_irreversibility', 'Quy tắc chuyển giao ngọn đuốc truyền thừa một đi không trở lại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-00000000002a', 'surpass_the_teacher_mandate', 'Quy tắc đệ tử bắt buộc phải vượt qua cảnh giới của sư phụ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000065', 'mp_unrefined_scouting', '1. Thu nhận Ngọc thô Ngỗ ngược (Rough Diamond Scouting)', 'Sư phụ nhìn thấu tiềm năng ẩn giấu sau vẻ ngoài bất kham, nhận đồ đệ bất chấp tông môn phản đối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000d2', '018f000f-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'torch_passing_irreversibility'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000066', 'mp_grueling_orthodoxy_trials', '2. Khổ luyện Truyền thừa Tâm pháp (Grueling Esoteric Training)', 'Gánh nước, bổ củi, tôi rèn căn cơ; sư phụ vừa nghiêm khắc vừa âm thầm bảo bọc đồ đệ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000dc', '018f000f-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'torch_passing_irreversibility'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000067', 'mp_philosophical_divergence', '3. Xung đột Quan điểm Võ đạo (Philosophical Rift)', 'Học trò khao khát tốc độ và sức mạnh thực dụng, chất vấn phương pháp bảo thủ của sư phụ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000e7', '018f000f-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'surpass_the_teacher_mandate'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000068', 'mp_mentor_sacrificial_shield', '4. Sư phụ Chắn đòn Hy sinh (Mentor Occupational Hazard)', 'Kẻ thù cũ tập kích; sư phụ dốc cạn tu vi thiêu đốt sinh mệnh để chặn hậu mở đường máu cho đệ tử trốn thoát.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000f0', '018f000f-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'torch_passing_irreversibility'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000069', 'mp_surpassing_the_master', '5. Đồ đệ Vượt ngưỡng Báo ân (Surpassing the Master & Honoring Legacy)', 'Đồ đệ giác ngộ chân lý của thầy, hoàn thiện chiêu thức cuối cùng, tiêu diệt đại địch và lập nên môn phái mới.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f000f-0000-7000-8000-0000000000fb', '018f000f-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'surpass_the_teacher_mandate'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-00000000012d', 'cmp_bamboo_stick_reprimand', 'Gậy Trúc Răn đe Rèn chí', 'Cú gõ đầu thức tỉnh tâm trí lơ đãng của thiếu niên kiêu ngạo.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-00000000012e', 'cmp_secret_manual_handover', 'Trao gửi Tàn quyển Trấn phái', 'Chiếc hộp gỗ chứa bí tịch thấm đẫm mồ hôi bao đời tôn sư.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-00000000012f', 'cmp_stormy_rooftop_argument', 'Tranh cãi Dưới Mưa trên Mái đình', 'Đệ tử quay lưng bỏ đi vì không đồng tình với sự nhẫn nhục.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000130', 'cmp_shattering_talisman_last_stand', 'Bóp nát Linh phù Đoạn hậu', 'Ánh mắt thanh thản của sư phụ khi đóng sập cửa hầm ngầm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f000f-0000-7000-8000-000000000131', 'cmp_carving_tablet_succession', 'Khắc bia Tưởng niệm Kế nghiệp', 'Dâng chén rượu lên mộ sư phụ rồi rút kiếm xuất sư.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000191', 'emo_rebellious_bravado', 'Ngỗ ngược Bất phục')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000192', 'emo_exhausted_gratitude', 'Kiệt sức Biết ơn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000193', 'emo_frustrated_indignation', 'Ức chế Bất bình')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000194', 'emo_gut_wrenching_anguish', 'Đứt ruột Đau đớn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f000f-0000-7000-8000-000000000195', 'emo_solemn_transcendence', 'Trang nghiêm Siêu việt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f5', 'eb_mp_scout', '018f000f-0000-7000-8000-00000000012d', '018f000f-0000-7000-8000-000000000065', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_rebellious_bravado'), '018f000f-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f6', 'eb_mp_train', '018f000f-0000-7000-8000-00000000012e', '018f000f-0000-7000-8000-000000000066', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_exhausted_gratitude'), '018f000f-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f7', 'eb_mp_rift', '018f000f-0000-7000-8000-00000000012f', '018f000f-0000-7000-8000-000000000067', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_frustrated_indignation'), '018f000f-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f8', 'eb_mp_sacrifice', '018f000f-0000-7000-8000-000000000130', '018f000f-0000-7000-8000-000000000068', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_gut_wrenching_anguish'), '018f000f-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f000f-0000-7000-8000-0000000001f9', 'eb_mp_surpass', '018f000f-0000-7000-8000-000000000131', '018f000f-0000-7000-8000-000000000069', '018f000f-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f000f-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_solemn_transcendence'), '018f000f-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002bd', 'Bước vào sơn môn khổ tu', '018f000f-0000-7000-8000-0000000001f5', '018f000f-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002be', 'Rạn nứt vì bất đồng quan điểm', '018f000f-0000-7000-8000-0000000001f6', '018f000f-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002bf', 'Bi kịch giáng xuống đền đáp công ơn', '018f000f-0000-7000-8000-0000000001f7', '018f000f-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f000f-0000-7000-8000-0000000002c0', 'Hóa bi thương thành sức mạnh cái thế', '018f000f-0000-7000-8000-0000000001f8', '018f000f-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000f-0000-7000-8000-000000000321', 'mp_mentor_must_pass_torch', 'Sư phụ phải trao lại biểu tượng uy quyền hoặc tri thức cốt lõi trước khi rút khỏi vị thế bảo hộ.', 'Tạo tính hợp pháp kế vị của người đệ tử.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000f-0000-7000-8000-000000000385', 'mentor_protege_dynamic_mp_mentor_must_pass_torch', (SELECT id FROM constraints WHERE code = 'mp_mentor_must_pass_torch'), '018f000f-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''mentor_protege_dynamic'', ''mp_mentor_must_pass_torch'')', 'Sư phụ phải trao lại biểu tượng uy quyền hoặc tri thức cốt lõi trước khi rút khỏi vị thế bảo hộ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f000f-0000-7000-8000-000000000322', 'mp_disciple_must_not_stay_shadow', 'Đồ đệ ở giai đoạn cuối bắt buộc phải giải quyết được vấn đề mà thế hệ sư phụ từng bất lực bó tay.', 'Thể hiện sự tiến bộ của lịch sử và sự trưởng thành chân thực.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f000f-0000-7000-8000-000000000386', 'mentor_protege_dynamic_mp_disciple_must_not_stay_shadow', (SELECT id FROM constraints WHERE code = 'mp_disciple_must_not_stay_shadow'), '018f000f-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''mentor_protege_dynamic'', ''mp_disciple_must_not_stay_shadow'')', 'Đồ đệ ở giai đoạn cuối bắt buộc phải giải quyết được vấn đề mà thế hệ sư phụ từng bất lực bó tay.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Defeat Means Friendship & Rivals Team Up Dynamic](https://tvtropes.org/pmwiki/pmwiki.php/Main/DefeatMeansFriendship):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0010-0000-7000-8000-000000000001', 'rivals_to_staunch_allies', 'Kình địch hóa Đồng minh Sinh tử (Rivals to Allies Dynamic)', 'relationship', '1.0.0', true, 'Mô-típ kình phùng địch thủ: Hai thiên tài có tính cách trái ngược (Lửa và Băng, Ánh sáng và Bóng tối), cạnh tranh từng bước chân. Sau trận huyết chiến định đoạt hoặc trước sự xuất hiện của mối đe dọa hủy diệt chung, họ bắt tay kề vai sát cánh thành cặp bài trùng bất khả chiến bại.', 20, 120);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000000015', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'shounen'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000000016', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xianxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000001016', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'urban_dushi'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0010-0000-7000-8000-000000000017', '018f0010-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mecha'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000029', 'mirror_foil_personality_rule', 'Quy tắc tính cách tương phản bổ trợ (Foil Dynamic)')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-00000000002a', 'mutual_respect_through_combat', 'Quy tắc tôn trọng được xây dựng qua đòn thế sinh tử')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000065', 'ra_fiery_competitiveness', '1. Cạnh tranh Gay gắt Bất phân Thắng bại (Fierce Ideological Rivalry)', 'Luôn so kè từng điểm số, từng đòn thế; coi đối phương là rào cản lớn nhất phải vượt qua.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000d2', '018f0010-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'mirror_foil_personality_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000066', 'ra_climactic_duel_of_honor', '2. Huyết chiến Quyết định Danh dự (Climactic Duel of Honor)', 'Trận đấu riêng tư không khoan nhượng; cả hai bộc lộ hết lá bài tẩy, nhận ra tri kỷ duy nhất thấu hiểu mình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000dd', '018f0010-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'mutual_respect_through_combat'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000067', 'ra_cataclysmic_common_foe', '3. Hiểm họa Tối thượng Xuất hiện (Greater Existential Threat)', 'Thế lực hắc ám hủy diệt ập đến đe dọa sinh tồn của cả hai; buộc phải tạm gác tranh chấp cá nhân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000e7', '018f0010-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'mutual_respect_through_combat'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000068', 'ra_flawless_combo_symphony', '4. Phối hợp Tác chiến Lưng tựa Lưng (Back-to-Back Synergy)', 'Không cần lời nói, một ánh mắt là hiểu ý; chiêu thức hai bên bù trừ hoàn hảo hạ gục quái vật.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000f0', '018f0010-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'mirror_foil_personality_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000069', 'ra_eternal_unspoken_bond', '5. Huynh đệ Tri kỷ Trọn đời (Unspoken Lifelong Brotherhood)', 'Tiếp tục cà khịa ngoài miệng nhưng sẵn sàng đỡ kiếm cho nhau trong chớp mắt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0010-0000-7000-8000-0000000000fa', '018f0010-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'mirror_foil_personality_rule'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-00000000012d', 'cmp_smug_scoreboard_taunt', 'Khiêu khích Bảng điểm Chiến tích', '''Hôm nay ta hạ 50 tên, ngươi mới có 49 thôi đấy!''.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-00000000012e', 'cmp_cross_counter_simultaneous_fist', 'Đấm Trúng Mặt Nhau Cùng Lúc', 'Cả hai ngã nhào ra đất rồi cùng bật cười sảng khoái.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-00000000012f', 'cmp_world_devouring_sky_crack', 'Bầu trời Nứt vỡ Báo hiệu Tận thế', 'Quân đoàn hắc ám kéo đến san bằng võ đài thi đấu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000130', 'cmp_interlocking_spear_and_shield', 'Thương Khiên Giao thoa Bọc lót', 'Một người tạo rào chắn phòng thủ tuyệt đối, người kia tung đòn kết liễu chí mạng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0010-0000-7000-8000-000000000131', 'cmp_fist_bump_silent_nod', 'Chạm Nắm đấm & Cái gật đầu Im lặng', 'Hành động ngắn gọn khẳng định sự tin tưởng giao phó tính mạng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000191', 'emo_fiery_jealousy', 'Ganh đua Bừng bừng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000192', 'emo_exhilarating_resonance', 'Đồng điệu Thăng hoa')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000193', 'emo_grim_sobering_crisis', 'Nghiêm trọng Nguy ngập')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000194', 'emo_telepathic_euphoria', 'Ăn ý Tuyệt đối')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0010-0000-7000-8000-000000000195', 'emo_fierce_camaraderie', 'Tình bạn Thép Kiên cố')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f5', 'eb_ra_rival', '018f0010-0000-7000-8000-00000000012d', '018f0010-0000-7000-8000-000000000065', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_fiery_jealousy'), '018f0010-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f6', 'eb_ra_duel', '018f0010-0000-7000-8000-00000000012e', '018f0010-0000-7000-8000-000000000066', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_exhilarating_resonance'), '018f0010-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f7', 'eb_ra_foe', '018f0010-0000-7000-8000-00000000012f', '018f0010-0000-7000-8000-000000000067', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_grim_sobering_crisis'), '018f0010-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f8', 'eb_ra_combo', '018f0010-0000-7000-8000-000000000130', '018f0010-0000-7000-8000-000000000068', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_telepathic_euphoria'), '018f0010-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0010-0000-7000-8000-0000000001f9', 'eb_ra_bond', '018f0010-0000-7000-8000-000000000131', '018f0010-0000-7000-8000-000000000069', '018f0010-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0010-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_fierce_camaraderie'), '018f0010-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002bd', 'Trận phân định đỉnh cao', '018f0010-0000-7000-8000-0000000001f5', '018f0010-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002be', 'Bão táp chung ập xuống', '018f0010-0000-7000-8000-0000000001f6', '018f0010-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002bf', 'Lưng tựa lưng nghênh địch', '018f0010-0000-7000-8000-0000000001f7', '018f0010-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0010-0000-7000-8000-0000000002c0', 'Định hình tình tri kỷ bất tử', '018f0010-0000-7000-8000-0000000001f8', '018f0010-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0010-0000-7000-8000-000000000321', 'ra_foil_complementarity', 'Hai nhân vật phải có ít nhất 2 đặc điểm tương phản trực tiếp (tính cách bốc đồng vs trầm tĩnh, cận chiến vs xạ thủ).', 'Tạo ra sự bổ khuyết hoàn hảo khi kết hợp lực lượng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0010-0000-7000-8000-000000000385', 'rivals_to_staunch_allies_ra_foil_complementarity', (SELECT id FROM constraints WHERE code = 'ra_foil_complementarity'), '018f0010-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''rivals_to_staunch_allies'', ''ra_foil_complementarity'')', 'Hai nhân vật phải có ít nhất 2 đặc điểm tương phản trực tiếp (tính cách bốc đồng vs trầm tĩnh, cận chiến vs xạ thủ).')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0010-0000-7000-8000-000000000322', 'ra_banter_retention', 'Dù đã trở thành đồng minh sinh tử, cả hai vẫn phải giữ thói quen mỉa mai trêu chọc lẫn nhau khi tình thế lắng dịu.', 'Giữ đúng nét đặc sắc đáng yêu của trope Rivals to Allies.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0010-0000-7000-8000-000000000386', 'rivals_to_staunch_allies_ra_banter_retention', (SELECT id FROM constraints WHERE code = 'ra_banter_retention'), '018f0010-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''rivals_to_staunch_allies'', ''ra_banter_retention'')', 'Dù đã trở thành đồng minh sinh tử, cả hai vẫn phải giữ thói quen mỉa mai trêu chọc lẫn nhau khi tình thế lắng dịu.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Literary Devices - Hollywood Symbolism: The Mirror and The Mask](https://literarydevices.net/hollywood-symbolism/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0011-0000-7000-8000-000000000001', 'the_mirror_and_mask_symbol', 'Biểu tượng Tấm gương & Mặt nạ (The Mirror & The Mask)', 'symbolic', '1.0.0', true, 'Mô-típ biểu tượng chiều sâu văn học: Mặt nạ đại diện cho bản ngã xã hội (Persona), sự che giấu tội lỗi hoặc lớp vỏ tự vệ; Tấm gương đại diện cho sự tự soi chiếu, đối diện nhân cách bóng tối (Shadow Self) và sự vỡ vụn của ảo tưởng.', 15, 80);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000000015', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'mystery_detective'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000001015', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'psychological'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000000016', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'horror'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0011-0000-7000-8000-000000000017', '018f0011-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'dark'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000029', 'symbolic_fracture_reflection', 'Quy tắc vết rạn trên gương phản chiếu sự đổ vỡ tâm thần')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-00000000002a', 'mask_peeling_exposure', 'Quy tắc lột mặt nạ phơi bày bản thể trần trụi')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000065', 'mm_impenetrable_persona', '1. Mặt nạ Hoàn hảo & Gương Phẳng lặng (Pristine Reflection)', 'Nhân vật đeo mặt nạ đạo mạo, tự mãn với hình tượng xã hội hoàn mỹ trong gương.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000d2', '018f0011-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'symbolic_fracture_reflection'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000066', 'mm_micro_cracks_whisper', '2. Vết rạn Li ti & Tiếng thì thầm (Micro-Fissures in the Glass)', 'Một biến cố nhỏ làm xuất hiện vết nứt trên kính; hình ảnh phản chiếu bắt đầu cười khi nhân vật khóc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000dc', '018f0011-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'symbolic_fracture_reflection'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000067', 'mm_nightmarish_distortion', '3. Khúc xạ Biến dạng Nhân cách (Distorted Shadow Confrontation)', 'Đối diện với chiếc gương méo mó; con quái vật bên trong cào cấu đòi thoát ra ngoài.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000e6', '018f0011-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'symbolic_fracture_reflection'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000068', 'mm_shattering_the_glass', '4. Gương Vỡ tan & Mặt nạ Rơi rụng (The Shattered Mask & Glass)', 'Khoảnh khắc đập tan tấm gương hoặc xé toạc mặt nạ; ảo tưởng che đậy chính thức tan biến.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000f1', '018f0011-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'mask_peeling_exposure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000069', 'mm_naked_authenticity_rebirth', '5. Bản ngã Chân thực Tái sinh (Naked Self Integration)', 'Nhìn vào hàng ngàn mảnh vỡ nhỏ, chấp nhận sự không hoàn hảo và hòa hợp với phần bóng tối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0011-0000-7000-8000-0000000000fb', '018f0011-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'mask_peeling_exposure'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-00000000012d', 'cmp_silver_gilded_frame', 'Khung Gương Bạc Mạ vàng', 'Chăm chút trang phục và nụ cười công nghiệp trước buổi dạ vũ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-00000000012e', 'cmp_spiderweb_crack_appearance', 'Màng Nhện Nứt nẻ Đột ngột', 'Một viên sỏi va vào kính làm lộ đường nứt chạy dọc con ngươi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-00000000012f', 'cmp_monstrous_reflection_reach', 'Cánh tay Bóng ma Vươn khỏi Gương', 'Bản sao tăm tối tóm lấy cổ tay nhân vật cảnh báo sự suy đồi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000130', 'cmp_bloody_knuckle_glass_strike', 'Cú đấm Nát Gương Nhuốm máu', 'Vung nắm đấm phá nát hình ảnh giả dối trong cơn phẫn nộ cùng cực.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0011-0000-7000-8000-000000000131', 'cmp_holding_tiny_shard_smile', 'Cầm Mảnh Vỡ Nhỏ Mỉm cười', 'Nhìn thấy đôi mắt mộc mạc không còn lớp trang điểm che giấu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000191', 'emo_varnished_complacency', 'Thỏa mãn Giả tạo')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000192', 'emo_creeping_dread_itch', 'Bất an Bứt rứt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000193', 'emo_hallucinatory_vertigo', 'Chóng mặt Hoang tưởng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000194', 'emo_violent_shattering_relief', 'Đổ vỡ Giải thoát Khốc liệt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0011-0000-7000-8000-000000000195', 'emo_quiet_wholeness', 'Toàn vẹn Lặng lẽ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f5', 'eb_mm_pristine', '018f0011-0000-7000-8000-00000000012d', '018f0011-0000-7000-8000-000000000065', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_varnished_complacency'), '018f0011-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f6', 'eb_mm_cracks', '018f0011-0000-7000-8000-00000000012e', '018f0011-0000-7000-8000-000000000066', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_creeping_dread_itch'), '018f0011-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f7', 'eb_mm_shadow', '018f0011-0000-7000-8000-00000000012f', '018f0011-0000-7000-8000-000000000067', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_hallucinatory_vertigo'), '018f0011-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f8', 'eb_mm_shatter', '018f0011-0000-7000-8000-000000000130', '018f0011-0000-7000-8000-000000000068', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_violent_shattering_relief'), '018f0011-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0011-0000-7000-8000-0000000001f9', 'eb_mm_naked', '018f0011-0000-7000-8000-000000000131', '018f0011-0000-7000-8000-000000000069', '018f0011-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0011-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_quiet_wholeness'), '018f0011-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002bd', 'Vết rạn đầu tiên xuất hiện', '018f0011-0000-7000-8000-0000000001f5', '018f0011-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002be', 'Bóng tối đòi quyền kiểm soát', '018f0011-0000-7000-8000-0000000001f6', '018f0011-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002bf', 'Đập nát ảo ảnh ngụy tạo', '018f0011-0000-7000-8000-0000000001f7', '018f0011-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0011-0000-7000-8000-0000000002c0', 'Tái sinh từ mảnh vụn chân thật', '018f0011-0000-7000-8000-0000000001f8', '018f0011-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0011-0000-7000-8000-000000000321', 'mm_mirror_repetition_rule', 'Hình tượng tấm gương hoặc mặt nạ phải xuất hiện ít nhất 3 lần tại các bước ngoặt cảm xúc trọng đại.', 'Đảm bảo tính nhất quán của motif biểu tượng xuyên suốt tác phẩm.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0011-0000-7000-8000-000000000385', 'the_mirror_and_mask_symbol_mm_mirror_repetition_rule', (SELECT id FROM constraints WHERE code = 'mm_mirror_repetition_rule'), '018f0011-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''the_mirror_and_mask_symbol'', ''mm_mirror_repetition_rule'')', 'Hình tượng tấm gương hoặc mặt nạ phải xuất hiện ít nhất 3 lần tại các bước ngoặt cảm xúc trọng đại.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0011-0000-7000-8000-000000000322', 'mm_physical_mirror_shattering', 'Hành vi vỡ gương phải đi kèm với chấn thương thể xác hoặc sự trần trụi hóa thân phận xã hội.', 'Liên kết trực quan giữa biểu tượng vật lý và sự chuyển hóa tâm thức.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0011-0000-7000-8000-000000000386', 'the_mirror_and_mask_symbol_mm_physical_mirror_shattering', (SELECT id FROM constraints WHERE code = 'mm_physical_mirror_shattering'), '018f0011-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''the_mirror_and_mask_symbol'', ''mm_physical_mirror_shattering'')', 'Hành vi vỡ gương phải đi kèm với chấn thương thể xác hoặc sự trần trụi hóa thân phận xã hội.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [Literary Devices - Bird Symbolism: The Phoenix, Ashes and Immortal Rebirth](https://literarydevices.net/bird-symbolism/):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0012-0000-7000-8000-000000000001', 'phoenix_ashes_rebirth_symbol', 'Biểu tượng Phượng hoàng Tro tàn (The Phoenix & Ashes Rebirth)', 'symbolic', '1.0.0', true, 'Biểu tượng thần thoại phổ quát về chu kỳ Hủy diệt để Tái sinh: Nhân vật phải trải qua quá trình suy kiệt, tự thiêu rụi cái cũ trong ngọn lửa thống khổ tột cùng, chìm vào tro tàn câm lặng trước khi vút bay lên với hình hài bất tử kiêu hãnh.', 15, 85);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0012-0000-7000-8000-000000000015', '018f0012-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0012-0000-7000-8000-000000000016', '018f0012-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'contemporary_fiction'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0012-0000-7000-8000-000000000017', '018f0012-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'xianxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000029', 'total_combustion_purity', 'Quy tắc thiêu rụi hoàn toàn không còn tạp chất cũ')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-00000000002a', 'ashes_incubation_period', 'Quy tắc ủ mầm trong tro tàn tịch mịch')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000065', 'phx_waning_plumage', '1. Lông vũ Phai màu & Sức tàn Lực kiệt (Waning Plumage)', 'Sinh lực cạn kiệt, vinh quang lụi tàn; nhân vật nhận thức rõ cái cũ đã đi đến hồi kết.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000d2', '018f0012-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000066', 'phx_pyre_of_self_immolation', '2. Dàn hỏa Tự thiêu & Nỗi đau Niết bàn (The Sacred Pyre Combustion)', 'Chủ động bước vào ngọn lửa đau đớn; thiêu rụi toàn bộ sai lầm, ảo tưởng và thân xác cũ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000dc', '018f0012-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000067', 'phx_dormant_cold_ashes', '3. Tro tàn Câm lặng Ủ mầm (Dormant Smoldering Ashes)', 'Khoảng lặng tuyệt đối; thế giới bên ngoài tưởng rằng mọi thứ đã kết thúc trong vô vọng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000e7', '018f0012-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'ashes_incubation_period'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000068', 'phx_golden_spark_resurgence', '4. Đốm lửa Vàng Thức tỉnh (The Golden Spark Awakening)', 'Từ giữa đống tro nguội ngắt, một tia lửa thần thánh le lói bốc cháy, quả trứng vàng nứt vỏ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000f0', '018f0012-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000069', 'phx_radiant_skyward_ascendance', '5. Tung cánh Phục sinh Rực rỡ (Radiant Skyward Ascendance)', 'Đôi cánh rực lửa xé toạc màn đêm, bay vút lên bầu trời với sức mạnh vĩnh cửu vô song.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0012-0000-7000-8000-0000000000fa', '018f0012-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'total_combustion_purity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-00000000012d', 'cmp_falling_grey_feather', 'Cọng Lông vũ Xám Rụng rơi', 'Dấu hiệu suy yếu không thể cứu vãn của cơ thể già cỗi.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-00000000012e', 'cmp_embracing_holy_inferno', 'Ôm trọn Lửa thiêng Thần thánh', 'Bước lên ngọn lửa tế đàn không hề run sợ.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-00000000012f', 'cmp_grey_powder_silence', 'Cát bụi Xám xịt Lặng câm', 'Gió thổi qua đống tro tàn nơi từng là vị thần kiêu hùng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000130', 'cmp_heartbeat_beneath_embers', 'Nhịp tim Đập Dưới Tàn than', 'Âm thanh thình thịch hồi sinh đập tan sự tĩnh lặng chết chóc.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0012-0000-7000-8000-000000000131', 'cmp_blazing_wing_cloud_spread', 'Đôi Cánh Lửa Che rợp Trời', 'Tiếng hót vang vọng chín tầng trời xua tan mọi tà khí.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000191', 'emo_melancholy_fading', 'Lụi tàn U sầu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000192', 'emo_searing_agony', 'Đau đớn Xé ruột Thiêu đốt')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000193', 'emo_abyssal_stillness', 'Tĩnh mịch Đáy vực')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000194', 'emo_miraculous_flutter', 'Rạo rực Kỳ diệu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0012-0000-7000-8000-000000000195', 'emo_majestic_ecstasy', 'Hân hoan Uy nghiêm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f5', 'eb_phx_waning', '018f0012-0000-7000-8000-00000000012d', '018f0012-0000-7000-8000-000000000065', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_melancholy_fading'), '018f0012-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f6', 'eb_phx_pyre', '018f0012-0000-7000-8000-00000000012e', '018f0012-0000-7000-8000-000000000066', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_searing_agony'), '018f0012-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f7', 'eb_phx_ashes', '018f0012-0000-7000-8000-00000000012f', '018f0012-0000-7000-8000-000000000067', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_abyssal_stillness'), '018f0012-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f8', 'eb_phx_spark', '018f0012-0000-7000-8000-000000000130', '018f0012-0000-7000-8000-000000000068', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_miraculous_flutter'), '018f0012-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0012-0000-7000-8000-0000000001f9', 'eb_phx_ascend', '018f0012-0000-7000-8000-000000000131', '018f0012-0000-7000-8000-000000000069', '018f0012-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0012-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_majestic_ecstasy'), '018f0012-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002bd', 'Bước vào ngọn lửa tự thiêu', '018f0012-0000-7000-8000-0000000001f5', '018f0012-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002be', 'Lắng đọng thành tro tàn lạnh giá', '018f0012-0000-7000-8000-0000000001f6', '018f0012-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002bf', 'Nhen nhóm tia sáng phục sinh', '018f0012-0000-7000-8000-0000000001f7', '018f0012-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0012-0000-7000-8000-0000000002c0', 'Tung cánh bay vút trời xanh', '018f0012-0000-7000-8000-0000000001f8', '018f0012-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0012-0000-7000-8000-000000000321', 'phx_must_touch_ashes', 'Quá trình tái sinh bắt buộc phải trải qua giai đoạn tro tàn (Dormant Ashes), không được bỏ qua bước chạm đáy tuyệt đối.', 'Tạo sự tương phản mãnh liệt và ý nghĩa triết học về sự tôi rèn qua mất mát.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0012-0000-7000-8000-000000000385', 'phoenix_ashes_rebirth_symbol_phx_must_touch_ashes', (SELECT id FROM constraints WHERE code = 'phx_must_touch_ashes'), '018f0012-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phoenix_ashes_rebirth_symbol'', ''phx_must_touch_ashes'')', 'Quá trình tái sinh bắt buộc phải trải qua giai đoạn tro tàn (Dormant Ashes), không được bỏ qua bước chạm đáy tuyệt đối.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0012-0000-7000-8000-000000000322', 'phx_purity_elevation', 'Sau khi tái sinh, nhân vật phải đạt được cảnh giới tinh thần hoặc sức mạnh vượt trội hơn hẳn trạng thái trước khi tự thiêu.', 'Bảo đảm ý nghĩa thăng hoa của biểu tượng Phượng hoàng.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0012-0000-7000-8000-000000000386', 'phoenix_ashes_rebirth_symbol_phx_purity_elevation', (SELECT id FROM constraints WHERE code = 'phx_purity_elevation'), '018f0012-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''phoenix_ashes_rebirth_symbol'', ''phx_purity_elevation'')', 'Sau khi tái sinh, nhân vật phải đạt được cảnh giới tinh thần hoặc sức mạnh vượt trội hơn hẳn trạng thái trước khi tự thiêu.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- Tham chiếu nghiên cứu thị trường: + [TV Tropes - Reforged Blade: Shattered Weapon Renewal](https://tvtropes.org/pmwiki/pmwiki.php/Main/ReforgedBlade):
-- 1. Table motifs
INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
  ('018f0013-0000-7000-8000-000000000001', 'shattered_and_reforged_blade', 'Biểu tượng Kiếm gãy Đúc lại (The Shattered & Reforged Blade)', 'symbolic', '1.0.0', true, 'Biểu tượng hiệp sĩ và di sản dòng tộc: Thanh bảo kiếm gia truyền tượng trưng cho danh dự và lý tưởng bị gãy vụn trước ác nhân. Người thừa kế phải nhặt từng mảnh vỡ, đem tôi luyện trong lò rèn định mệnh bằng máu và ý chí mới để đúc nên thanh thần kiếm phục hưng vương quốc.', 20, 100);

-- 2. Table motif_applicable_genres
INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0013-0000-7000-8000-000000000015', '018f0013-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'fantasy'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

INSERT INTO motif_applicable_genres (id, motif_id, genre_id) VALUES
  ('018f0013-0000-7000-8000-000000000016', '018f0013-0000-7000-8000-000000000001', (SELECT id FROM genres WHERE code = 'wuxia'))
  ON CONFLICT (motif_id, genre_id) DO NOTHING;

-- 3. Table state_rules
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000029', 'heirloom_resonance_continuity', 'Quy tắc huyết mạch cộng hưởng với linh hồn thanh kiếm')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO state_rules (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-00000000002a', 'crucible_sacrifice_reforging', 'Quy tắc bắt buộc phải có chất xúc tác hy sinh khi tôi luyện lại')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 4. Table states & state_down_by_rules
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000065', 'srb_flawed_ancestral_glory', '1. Thanh kiếm Gia truyền Khuyết tật (Flawed Ancestral Heritage)', 'Mang theo thanh kiếm của tổ tiên; danh tiếng vang dội nhưng ẩn chứa vết nứt định mệnh của thế hệ trước.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000d2', '018f0013-0000-7000-8000-000000000065', (SELECT id FROM state_rules WHERE code = 'heirloom_resonance_continuity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000066', 'srb_catastrophic_fracture', '2. Gãy vụn Dưới Búa rìu Kẻ thù (Catastrophic Fracture in Battle)', 'Đối đầu kẻ thù không đội trời chung; lưỡi kiếm gãy làm đôi, niềm tin sụp đổ, bóng tối bao trùm.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000dc', '018f0013-0000-7000-8000-000000000066', (SELECT id FROM state_rules WHERE code = 'heirloom_resonance_continuity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000067', 'srb_gathering_sacred_shards', '3. Nhặt từng Mảnh vỡ Đau thương (Gathering the Broken Shards)', 'Qùy gối nhặt từng mảnh thép sắc nhọn cứa vào tay; chấp nhận bài học thất bại xương máu.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000e6', '018f0013-0000-7000-8000-000000000067', (SELECT id FROM state_rules WHERE code = 'heirloom_resonance_continuity'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000068', 'srb_crucible_of_reforging', '4. Lò lửa Tôi luyện & Hiến tế Máu (The Crucible of Sacred Reforging)', 'Nấu chảy mảnh kiếm trong ngọn lửa rồng hoặc dung nham thiêng; hòa tan máu và lời thề mới của chính mình.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000f1', '018f0013-0000-7000-8000-000000000068', (SELECT id FROM state_rules WHERE code = 'crucible_sacrifice_reforging'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;
INSERT INTO states (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000069', 'srb_awakened_sun_blade', '5. Thần kiếm Tái sinh Sáng như Thái dương (The Awakened Unbreakable Blade)', 'Rút thanh kiếm mới ra khỏi lò đúc; đường vân phát sáng, chém đứt xiềng xích nô dịch bảo vệ nhân dân.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed) VALUES
  ('018f0013-0000-7000-8000-0000000000fb', '018f0013-0000-7000-8000-000000000069', (SELECT id FROM state_rules WHERE code = 'crucible_sacrifice_reforging'), true)
  ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

-- 5. Table components
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-00000000012d', 'cmp_silver_hilt_crested_lion', 'Chuôi Kiếm Bạc Khắc Sư tử', 'Di vật truyền qua 7 đời vua nhưng mang lời nguyền thất trận.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-00000000012e', 'cmp_metallic_snap_screech', 'Tiếng Kim loại Gãy Vang trời', 'Lưỡi thép văng tung tóe cắm vào đá lạnh khi bị chùy đen đập trúng.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-00000000012f', 'cmp_bleeding_palms_shard_pouch', 'Bàn tay Nhuốm máu Bọc mảnh Thép', 'Cẩn thận bọc từng mảnh vỡ vào tấm lụa giữ sát bên tim.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000130', 'cmp_anvil_strike_dragon_fire', 'Nhát Búa Đập Dưới Lửa Rồng', 'Từng tiếng búa chan chát dội vào không gian hòa cùng mồ hôi và nước mắt.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;
INSERT INTO components (id, code, display_name, description) VALUES
  ('018f0013-0000-7000-8000-000000000131', 'cmp_unsheathing_solar_flare', 'Rút Kiếm Phát Quang Rực rỡ', 'Lưỡi gươm bén ngót không vết xước xé toạc màn sương mù u tối.')
  ON CONFLICT (code) DO UPDATE SET description = EXCLUDED.description;

-- 6. Table emotional_ranges
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000191', 'emo_heavy_lineage_burden', 'Gánh nặng Huyết thống Nặng nề')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000192', 'emo_crushing_disillusionment', 'Sụp đổ Vỡ mộng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000193', 'emo_penitent_reverence', 'Thành kính Ăn năn')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000194', 'emo_furnace_determination', 'Quyết tâm Lò lửa Nung nấu')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;
INSERT INTO emotional_ranges (id, code, display_name) VALUES
  ('018f0013-0000-7000-8000-000000000195', 'emo_radiant_invincibility', 'Bất khả Chiến bại Rực sáng')
  ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name;

-- 7. Table event_blocks & event_block_emotionals
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f5', 'eb_srb_heritage', '018f0013-0000-7000-8000-00000000012d', '018f0013-0000-7000-8000-000000000065', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-000000000262', (SELECT id FROM emotional_ranges WHERE code = 'emo_heavy_lineage_burden'), '018f0013-0000-7000-8000-0000000001f5')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f6', 'eb_srb_fracture', '018f0013-0000-7000-8000-00000000012e', '018f0013-0000-7000-8000-000000000066', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-00000000026d', (SELECT id FROM emotional_ranges WHERE code = 'emo_crushing_disillusionment'), '018f0013-0000-7000-8000-0000000001f6')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f7', 'eb_srb_shards', '018f0013-0000-7000-8000-00000000012f', '018f0013-0000-7000-8000-000000000067', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-000000000278', (SELECT id FROM emotional_ranges WHERE code = 'emo_penitent_reverence'), '018f0013-0000-7000-8000-0000000001f7')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f8', 'eb_srb_reforge', '018f0013-0000-7000-8000-000000000130', '018f0013-0000-7000-8000-000000000068', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-000000000283', (SELECT id FROM emotional_ranges WHERE code = 'emo_furnace_determination'), '018f0013-0000-7000-8000-0000000001f8')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;
INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES
  ('018f0013-0000-7000-8000-0000000001f9', 'eb_srb_awakened', '018f0013-0000-7000-8000-000000000131', '018f0013-0000-7000-8000-000000000069', '018f0013-0000-7000-8000-000000000001')
  ON CONFLICT (code) DO NOTHING;
INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id) VALUES
  ('018f0013-0000-7000-8000-00000000028e', (SELECT id FROM emotional_ranges WHERE code = 'emo_radiant_invincibility'), '018f0013-0000-7000-8000-0000000001f9')
  ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

-- 8. Table event_transitions
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002bd', 'Thảm bại gãy kiếm trong tuyệt vọng', '018f0013-0000-7000-8000-0000000001f5', '018f0013-0000-7000-8000-0000000001f6', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002be', 'Gom nhặt mảnh vỡ đúc kết bài học', '018f0013-0000-7000-8000-0000000001f6', '018f0013-0000-7000-8000-0000000001f7', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002bf', 'Thổi bùng ngọn lửa tôi rèn ý chí mới', '018f0013-0000-7000-8000-0000000001f7', '018f0013-0000-7000-8000-0000000001f8', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;
INSERT INTO event_transitions (id, display_name, from_event_block_id, to_event_block_id, priority) VALUES
  ('018f0013-0000-7000-8000-0000000002c0', 'Rút kiếm xuất thế định đoạt thiên hạ', '018f0013-0000-7000-8000-0000000001f8', '018f0013-0000-7000-8000-0000000001f9', 1)
  ON CONFLICT (from_event_block_id, to_event_block_id) DO UPDATE SET priority = EXCLUDED.priority;

-- 9. Table constraints & motif_constraints
INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0013-0000-7000-8000-000000000321', 'srb_ancestral_flaw_redemption', 'Thanh kiếm ban đầu bắt buộc phải mang một khiếm khuyết trong tư tưởng của tiền nhân được khắc phục khi đúc lại.', 'Tượng trưng cho sự tiến hóa của thế hệ sau vượt qua giới hạn của thế hệ trước.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0013-0000-7000-8000-000000000385', 'shattered_and_reforged_blade_srb_ancestral_flaw_redemption', (SELECT id FROM constraints WHERE code = 'srb_ancestral_flaw_redemption'), '018f0013-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''shattered_and_reforged_blade'', ''srb_ancestral_flaw_redemption'')', 'Thanh kiếm ban đầu bắt buộc phải mang một khiếm khuyết trong tư tưởng của tiền nhân được khắc phục khi đúc lại.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

INSERT INTO constraints (id, code, require, reason) VALUES
  ('018f0013-0000-7000-8000-000000000322', 'srb_reforging_catalyst', 'Khi đúc lại, bắt buộc phải có một kim loại mới hoặc một chất xúc tác tinh thần kết hợp với mảnh vỡ cũ.', 'Khẳng định thanh kiếm mới không chỉ đơn thuần là hàn gắn mà là sự chuyển hóa toàn diện.')
  ON CONFLICT (code) DO UPDATE SET require = EXCLUDED.require, reason = EXCLUDED.reason;
INSERT INTO motif_constraints (id, code, constraint_id, motif_id, syntax, constrain) VALUES
  ('018f0013-0000-7000-8000-000000000386', 'shattered_and_reforged_blade_srb_reforging_catalyst', (SELECT id FROM constraints WHERE code = 'srb_reforging_catalyst'), '018f0013-0000-7000-8000-000000000001', 'VALIDATE_MOTIF_RULE(''shattered_and_reforged_blade'', ''srb_reforging_catalyst'')', 'Khi đúc lại, bắt buộc phải có một kim loại mới hoặc một chất xúc tác tinh thần kết hợp với mảnh vỡ cũ.')
  ON CONFLICT (code) DO UPDATE SET syntax = EXCLUDED.syntax, constrain = EXCLUDED.constrain;

-- -----------------------------------------------------------------------------
-- 3. Narrator Configurations
-- -----------------------------------------------------------------------------
INSERT INTO narrator_configs
  (id, code, display_name, description, person, person_subtype, pov_scope, is_reliable, breaks_fourth_wall)
VALUES
  ('01a1bc44-fc00-7f57-8ce2-8ff270060e71', 'first_person_protagonist_single_pov_reliable_no_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-7ffa-97bf-2a8a03115cfd', 'first_person_protagonist_single_pov_reliable_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7c7a-bce7-04b88608a1fb', 'first_person_protagonist_single_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-757f-bd3b-a4789d7375b8', 'first_person_protagonist_single_pov_unreliable_fourth_wall', 'narrator_config.first_person_protagonist_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_single_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7b70-a28d-a648bca1cd06', 'first_person_protagonist_multiple_pov_reliable_no_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-714b-b1c7-803b23fdb707', 'first_person_protagonist_multiple_pov_reliable_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7ae1-8777-ad6402042ade', 'first_person_protagonist_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7541-baec-2421b4a64e43', 'first_person_protagonist_multiple_pov_unreliable_fourth_wall', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_protagonist_multiple_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'PROTAGONIST', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-70b0-9916-46e93118a1ce', 'first_person_witness_peripheral_single_pov_reliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-7365-b90a-7d839aaaaf0f', 'first_person_witness_peripheral_single_pov_reliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-706d-b510-589cae684ccd', 'first_person_witness_peripheral_single_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-71ba-a6cc-48c61319a4c9', 'first_person_witness_peripheral_single_pov_unreliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_single_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7a43-b61e-8ebd3f9cf8c2', 'first_person_witness_peripheral_multiple_pov_reliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7871-bc58-b0618a8c2887', 'first_person_witness_peripheral_multiple_pov_reliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_reliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7619-b046-8a5d8b6de639', 'first_person_witness_peripheral_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7059-b3b6-48f3037e6e1f', 'first_person_witness_peripheral_multiple_pov_unreliable_fourth_wall', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_witness_peripheral_multiple_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'WITNESS_PERIPHERAL', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-79a6-a0f4-b15d9d977982', 'first_person_unreliable_first_single_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7ba0-bb7f-6806ee9cd785', 'first_person_unreliable_first_single_pov_unreliable_fourth_wall', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_single_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7541-bbba-08729ac477b9', 'first_person_unreliable_first_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_no_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-713b-ba44-b3d23ecf6c17', 'first_person_unreliable_first_multiple_pov_unreliable_fourth_wall', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.first_person_unreliable_first_multiple_pov_unreliable_fourth_wall.description', 'FIRST_PERSON', 'UNRELIABLE_FIRST', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-78ef-85bf-7a21ca202385', 'second_person_direct_address_single_pov_reliable_no_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-74b2-bcc8-c5cd75558364', 'second_person_direct_address_single_pov_reliable_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7614-b325-7b51201e0fc6', 'second_person_direct_address_single_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7708-8666-da0a213bb47a', 'second_person_direct_address_single_pov_unreliable_fourth_wall', 'narrator_config.second_person_direct_address_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_single_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7685-ba62-42ce9fabd51f', 'second_person_direct_address_multiple_pov_reliable_no_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7f7a-a3ef-657ef9f7e493', 'second_person_direct_address_multiple_pov_reliable_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7d96-a80d-c63bbaa4e96d', 'second_person_direct_address_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7604-8029-76e8827cb379', 'second_person_direct_address_multiple_pov_unreliable_fourth_wall', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_direct_address_multiple_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'DIRECT_ADDRESS', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-7b2b-9f25-f6ba78b8a597', 'second_person_self_reflection_single_pov_reliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-748b-9f0e-dad40d856b89', 'second_person_self_reflection_single_pov_reliable_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7dd2-9210-b84fd3333f28', 'second_person_self_reflection_single_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7b6b-9bcd-e5c184a0f75c', 'second_person_self_reflection_single_pov_unreliable_fourth_wall', 'narrator_config.second_person_self_reflection_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_single_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-7725-82cf-bc8619bd275c', 'second_person_self_reflection_multiple_pov_reliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7669-a2dd-68a42ca93f38', 'second_person_self_reflection_multiple_pov_reliable_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7aa2-84b2-1ad0cb5e8166', 'second_person_self_reflection_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7625-abf2-52f5000fd82e', 'second_person_self_reflection_multiple_pov_unreliable_fourth_wall', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_self_reflection_multiple_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'SELF_REFLECTION', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-70f9-89fb-e526c112a435', 'second_person_interactive_single_pov_reliable_no_fourth_wall', 'narrator_config.second_person_interactive_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-7445-8447-93080aa7a2df', 'second_person_interactive_single_pov_reliable_fourth_wall', 'narrator_config.second_person_interactive_single_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-744f-b8ff-8aad2eadeba3', 'second_person_interactive_single_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_interactive_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7649-8428-c014ade93163', 'second_person_interactive_single_pov_unreliable_fourth_wall', 'narrator_config.second_person_interactive_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_single_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-75c7-91a1-c11b1a86c29a', 'second_person_interactive_multiple_pov_reliable_no_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_reliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7f9e-8e97-4cf8da4b299c', 'second_person_interactive_multiple_pov_reliable_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_reliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7f2b-93f5-55f2cc3cb8ff', 'second_person_interactive_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_unreliable_no_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-7260-81f3-80f31442970c', 'second_person_interactive_multiple_pov_unreliable_fourth_wall', 'narrator_config.second_person_interactive_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.second_person_interactive_multiple_pov_unreliable_fourth_wall.description', 'SECOND_PERSON', 'INTERACTIVE', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-7889-8be0-59c4ab1dd452', 'third_person_omniscient_shifting_omniscient_reliable_no_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 1, 0),
  ('01a1bc44-fc00-72db-9faa-69647b028b0a', 'third_person_omniscient_shifting_omniscient_reliable_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_reliable_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 1, 1),
  ('01a1bc44-fc00-7b8d-af2d-1c8622d6b25c', 'third_person_omniscient_shifting_omniscient_unreliable_no_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 0, 0),
  ('01a1bc44-fc00-7722-abf8-a442b13e983c', 'third_person_omniscient_shifting_omniscient_unreliable_fourth_wall', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_fourth_wall.display_name', 'narrator_config.third_person_omniscient_shifting_omniscient_unreliable_fourth_wall.description', 'THIRD_PERSON', 'OMNISCIENT', 'SHIFTING_OMNISCIENT', 0, 1),
  ('01a1bc44-fc00-7643-9cea-ffa43c0d94d9', 'third_person_limited_single_pov_reliable_no_fourth_wall', 'narrator_config.third_person_limited_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-788e-8313-4433f3ef575d', 'third_person_limited_single_pov_reliable_fourth_wall', 'narrator_config.third_person_limited_single_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-7e52-9b7d-56cf47f9ab69', 'third_person_limited_single_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_limited_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-7911-a9fd-56a3c953a668', 'third_person_limited_single_pov_unreliable_fourth_wall', 'narrator_config.third_person_limited_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_limited_single_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-75c9-a15d-bbe38907102c', 'third_person_limited_multiple_pov_reliable_no_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-74ad-aeff-191bb96a7495', 'third_person_limited_multiple_pov_reliable_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7d3b-8185-0fea8d199745', 'third_person_limited_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-79fe-a522-f8b955112e3e', 'third_person_limited_multiple_pov_unreliable_fourth_wall', 'narrator_config.third_person_limited_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_limited_multiple_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'LIMITED', 'MULTIPLE_POV', 0, 1),
  ('01a1bc44-fc00-7baa-b5c4-8dc968286223', 'third_person_objective_single_pov_reliable_no_fourth_wall', 'narrator_config.third_person_objective_single_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 1, 0),
  ('01a1bc44-fc00-794c-8b3a-09a4c48bd1a4', 'third_person_objective_single_pov_reliable_fourth_wall', 'narrator_config.third_person_objective_single_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 1, 1),
  ('01a1bc44-fc00-79b5-9874-3d6c28440d24', 'third_person_objective_single_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_objective_single_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 0, 0),
  ('01a1bc44-fc00-73c2-9ac2-c95baf83dc00', 'third_person_objective_single_pov_unreliable_fourth_wall', 'narrator_config.third_person_objective_single_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_objective_single_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'SINGLE_POV', 0, 1),
  ('01a1bc44-fc00-729d-9dbc-b7492a68e3be', 'third_person_objective_multiple_pov_reliable_no_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_reliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_reliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 1, 0),
  ('01a1bc44-fc00-7b82-9634-77a2ff558bdc', 'third_person_objective_multiple_pov_reliable_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_reliable_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_reliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 1, 1),
  ('01a1bc44-fc00-7d89-ba35-dbfda5532d62', 'third_person_objective_multiple_pov_unreliable_no_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_unreliable_no_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_unreliable_no_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 0, 0),
  ('01a1bc44-fc00-70bc-94f2-99d35394ebd4', 'third_person_objective_multiple_pov_unreliable_fourth_wall', 'narrator_config.third_person_objective_multiple_pov_unreliable_fourth_wall.display_name', 'narrator_config.third_person_objective_multiple_pov_unreliable_fourth_wall.description', 'THIRD_PERSON', 'OBJECTIVE', 'MULTIPLE_POV', 0, 1)
ON CONFLICT (code) DO UPDATE SET display_name = EXCLUDED.display_name, description = EXCLUDED.description, person = EXCLUDED.person, person_subtype = EXCLUDED.person_subtype, pov_scope = EXCLUDED.pov_scope, is_reliable = EXCLUDED.is_reliable, breaks_fourth_wall = EXCLUDED.breaks_fourth_wall;;

-- -----------------------------------------------------------------------------
-- 4. Narrative Directives
-- -----------------------------------------------------------------------------
-- ==============================================================================
-- 1. BẢNG NARRATIVE_DIRECTIVES (CHỈ THỊ TỰ SỰ & NGUYÊN TẮC HÀNH VĂN HỆ THỐNG)
-- Được tổng hợp và tinh lọc từ 18 nguồn học thuật & kinh nghiệm thực chiến tiểu thuyết:
-- Reedsy, Jane Friedman, MasterClass, Jericho Writers, Strunk & White (Elements of Style),
-- Stephen King (On Writing), George Orwell, Elmore Leonard, Brandon Sanderson, K.M. Weiland,
-- Tim Weed (Writer's Digest), TV Tropes (Chekhov's Gun, Red Herring, Infodump), LitReactor.
-- Phân loại chuẩn 3 danh mục: PROSE_PRINCIPLE, RESTRAINT, INFORMATION_CONTROL.
-- ==============================================================================

-- 1.1. Category: PROSE_PRINCIPLE (Nguyên tắc hành văn)
INSERT INTO narrative_directives (id, code, description, category, is_system) VALUES
('0196ba10-0001-7000-8000-000000000001', 'show_dont_tell', 'Thể hiện cảm xúc, tính cách nhân vật và diễn biến thông qua hành động vật lý, phản ứng cơ thể, đối thoại và chi tiết giác quan thay vì dùng người kể chuyện tóm tắt, dán nhãn trừu tượng.', 'PROSE_PRINCIPLE', true),
('0196ba10-0001-7000-8000-000000000002', 'sensory_grounding', 'Neo giữ phân cảnh vào thực tại bằng cách kích hoạt chọn lọc 5 giác quan (thị giác, thính giác, khứu giác, xúc giác, vị giác) gắn liền mật thiết với trải nghiệm tức thời của nhân vật điểm nhìn.', 'PROSE_PRINCIPLE', true),
('0196ba10-0001-7000-8000-000000000003', 'action_driven_emotion', 'Thay thế các từ ngữ chỉ cảm xúc chung chung (sợ hãi, tức giận, đau buồn) bằng phản ứng sinh lý cụ thể (tim đập dồn dập, đồng tử co thắt, khô khốc cổ họng, siết chặt nắm tay).', 'PROSE_PRINCIPLE', true),
('0196ba10-0002-7000-8000-000000000001', 'deep_pov_immersion', 'Hòa nhập hoàn toàn giọng văn trần thuật vào nhận thức, tâm lý, thái độ và vốn từ vựng của nhân vật điểm nhìn (Deep POV), xóa nhòa ranh giới giữa tác giả và nhân vật.', 'PROSE_PRINCIPLE', true),
('0196ba10-0002-7000-8000-000000000002', 'internal_monologue_weave', 'Đan cài dòng suy nghĩ nội tâm trực tiếp của nhân vật vào dòng chảy hành động một cách tự nhiên mà không cần dùng thẻ dẫn rườm rà (như ''anh nghĩ'', ''cô tự nhủ'') hay in nghiêng gượng ép.', 'PROSE_PRINCIPLE', true),
('0196ba10-0003-7000-8000-000000000001', 'active_voice_preference', 'Ưu tiên sử dụng câu chủ động với động từ hành động mạnh mẽ, tạo lực đẩy trực diện và nhịp điệu dứt khoát cho từng phân cảnh.', 'PROSE_PRINCIPLE', true),
('0196ba10-0003-7000-8000-000000000002', 'sentence_rhythm_variation', 'Biến hóa linh hoạt độ dài và cấu trúc câu: dùng chuỗi câu ngắn, cô đọng khi đẩy cao trào kịch tính và dùng câu phức có nhịp điệu khi miêu tả chiều sâu tâm cảnh.', 'PROSE_PRINCIPLE', true),
('0196ba10-0003-7000-8000-000000000003', 'specific_concrete_nouns', 'Sử dụng danh từ và động từ cụ thể, chính xác, giàu hình ảnh thị giác thay vì lạm dụng danh từ trừu tượng phải đi kèm nhiều tính từ giải thích.', 'PROSE_PRINCIPLE', true),
('0196ba10-0004-7000-8000-000000000001', 'clarity_first', 'Đặt sự trong sáng, rõ nghĩa và chuẩn xác lên hàng đầu; đảm bảo độc giả hình dung được ngay lập tức không gian, phương vị và hành động mà không phải khựng lại đọc lại.', 'PROSE_PRINCIPLE', true),
('0196ba10-0004-7000-8000-000000000002', 'fresh_metaphor_precision', 'Sáng tạo các hình ảnh so sánh và ẩn dụ độc đáo, bén nhọn, gắn liền với thế giới quan và cảm xúc nhân vật thay vì trang trí câu từ rỗng tuếch.', 'PROSE_PRINCIPLE', true),
('0196ba10-0005-7000-8000-000000000004', 'lyrical_economy', 'Kết hợp hài hòa giữa chất thơ, nhạc tính của câu chữ với tính kinh tế của ngôn từ (lyrical economy), giữ cho câu văn gợi cảm nhưng không làm trì trệ mạch tự sự.', 'PROSE_PRINCIPLE', true),
('0196ba10-0006-7000-8000-000000000002', 'positive_form_statements', 'Diễn đạt tư tưởng và sự kiện ở dạng khẳng định trực diện; hạn chế cách diễn đạt vòng vèo, phủ định của phủ định làm yếu đi lực tác động của câu văn.', 'PROSE_PRINCIPLE', true),
('0196ba10-0006-7000-8000-000000000004', 'emphatic_sentence_endings', 'Bố trí từ ngữ mang thông tin trọng tâm hoặc tạo điểm rơi cảm xúc mạnh nhất ở cuối câu hoặc cuối đoạn văn để tối đa hóa ấn tượng tâm lý.', 'PROSE_PRINCIPLE', true),
('0196ba10-0007-7000-8000-000000000003', 'contextual_dialogue_clarity', 'Để bản thân nội dung câu thoại, ngữ cảnh và hành động kèm theo tự thể hiện ngữ khí, cảm xúc của người nói thay vì phụ thuộc vào thẻ thoại giải thích.', 'PROSE_PRINCIPLE', true),
('0196ba10-0007-7000-8000-000000000004', 'fearless_direct_prose', 'Viết trực diện, can trường và dứt khoát; không rào đón, thanh minh hay dài dòng vì sợ độc giả không cảm nhận được ngụ ý.', 'PROSE_PRINCIPLE', true),
('0196ba10-0008-7000-8000-000000000005', 'clarity_over_dogma', 'Sẵn sàng linh hoạt phá vỡ các quy tắc ngữ pháp hay tu từ cứng nhắc khi điều đó đem lại câu văn tự nhiên, giàu sức sống và chân thực hơn.', 'PROSE_PRINCIPLE', true),
('0196ba10-0009-7000-8000-000000000001', 'invisible_authorial_style', 'Giữ giọng văn trần thuật tự nhiên, trong suốt (invisible style); nếu câu văn có vẻ đang ''phô diễn kỹ thuật viết văn'', hãy chỉnh sửa cho đến khi nó hòa nhập trơn tru vào câu chuyện.', 'PROSE_PRINCIPLE', true),
('0196ba10-000a-7000-8000-000000000001', 'windowpane_prose', 'Áp dụng phong cách ''kính cửa sổ trong suốt'' (windowpane prose theo Brandon Sanderson): câu chữ rõ ràng, không màu mè để độc giả nhìn xuyên qua từ ngữ vào thẳng cốt truyện.', 'PROSE_PRINCIPLE', true),
('0196ba10-000c-7000-8000-000000000003', 'pacing_cadence_modulation', 'Điều tiết nhịp điệu tự sự luân phiên khoa học giữa các trường đoạn hành động dồn dập (câu ngắn, nhịp nhanh) và các khoảng nghỉ phản tư (sequel scenes) để người đọc lắng đọng cảm xúc.', 'PROSE_PRINCIPLE', true),
('0196ba10-000e-7000-8000-000000000003', 'voice_authenticity_bias', 'Thể hiện tính cách và thiên kiến nhận thức của nhân vật ngay trong cách lựa chọn vốn từ, ngữ điệu và thái độ miêu tả sự vật xung quanh.', 'PROSE_PRINCIPLE', true),
('0196ba10-0011-7000-8000-000000000004', 'conflict_embedded_exposition', 'Cài cắm thông tin bối cảnh trực tiếp vào các cuộc tranh luận nảy lửa, xung đột quyền lợi hoặc các tình thế nan giải mà nhân vật đang nỗ lực tháo gỡ.', 'PROSE_PRINCIPLE', true),
('0196ba10-0012-7000-8000-000000000001', 'scene_transition_economy', 'Chuyển cảnh sắc bén và kinh tế: bước vào cảnh muộn nhất có thể (khi xung đột đã chớm nở) và rời cảnh sớm nhất có thể (ngay khi đạt được bước ngoặt cảm xúc).', 'PROSE_PRINCIPLE', true);

-- 1.2. Category: RESTRAINT (Tiết chế)
INSERT INTO narrative_directives (id, code, description, category, is_system) VALUES
('0196ba10-0001-7000-8000-000000000004', 'telling_summary_restraint', 'Tiết chế tối đa văn kể lể tóm tắt (telling summary) tại các phân cảnh cao trào; chỉ sử dụng tóm tắt khi cần chuyển cảnh lướt qua khoảng thời gian chết.', 'RESTRAINT', true),
('0196ba10-0002-7000-8000-000000000003', 'filter_word_restraint', 'Loại bỏ triệt để các từ lọc tạo khoảng cách nhận thức (như: thấy, nghe, nhận thấy, cảm thấy, dường như, tự hỏi) để người đọc trực tiếp trải nghiệm thế giới qua giác quan nhân vật.', 'RESTRAINT', true),
('0196ba10-0002-7000-8000-000000000004', 'authorial_intrusion_restraint', 'Kiềm chế sự can thiệp của tác giả (authorial intrusion); không chèn triết lý hay bình luận mang tính thượng đẳng vượt ngoài nhận thức của nhân vật điểm nhìn.', 'RESTRAINT', true),
('0196ba10-0003-7000-8000-000000000004', 'passive_voice_restraint', 'Tiết chế cấu trúc câu bị động; chỉ sử dụng khi cần nhấn mạnh đối tượng bị tác động hoặc khi kẻ thực hiện hành vi cố tình bị giấu kín trong bóng tối.', 'RESTRAINT', true),
('0196ba10-0003-7000-8000-000000000005', 'cliche_avoidance', 'Tuyệt đối tránh sử dụng các ẩn dụ sáo mòn, thành ngữ cũ rích (clichés) hoặc các mô tả rập khuôn làm giảm giá trị nghệ thuật của trang viết.', 'RESTRAINT', true),
('0196ba10-0004-7000-8000-000000000003', 'overwriting_restraint', 'Tiết chế lối viết dài dòng thừa thãi (overwriting); cắt bỏ các mệnh đề giải thích lại những gì đã được bộc lộ rõ qua hành động và ngữ cảnh đối thoại.', 'RESTRAINT', true),
('0196ba10-0004-7000-8000-000000000004', 'melodrama_restraint', 'Kiềm chế cảm xúc cường điệu hóa (melodrama), tránh gượng ép giọt nước mắt hay lạm dụng ngôn từ bi thương quá đà; để sức nặng tự nhiên của sự kiện chạm tới người đọc.', 'RESTRAINT', true),
('0196ba10-0005-7000-8000-000000000001', 'purple_prose_restraint', 'Kiên quyết tiết chế văn phong hoa mỹ diêm dúa (purple prose): loại bỏ câu từ cầu kỳ rườm rà, từ vựng đa âm bóng bẩy làm đứt gãy sự tập trung vào mạch truyện.', 'RESTRAINT', true),
('0196ba10-0005-7000-8000-000000000002', 'adjective_pileup_restraint', 'Tránh xếp chồng nhiều tính từ và trạng từ liên tiếp trước một danh từ/động từ; nếu có hai tính từ đồng nghĩa, hãy bỏ cả hai và chọn một danh từ thật mạnh.', 'RESTRAINT', true),
('0196ba10-0005-7000-8000-000000000003', 'thesaurus_syndrome_restraint', 'Tránh hội chứng lạm dụng từ điển đồng nghĩa (thesaurus syndrome); không dùng từ cổ, từ hiếm một cách gượng ép khi từ phổ thông diễn đạt chính xác hơn.', 'RESTRAINT', true),
('0196ba10-0006-7000-8000-000000000001', 'omit_needless_words', 'Cắt bỏ mọi từ ngữ thừa thãi (Omit needless words theo Strunk & White): một câu không nên thừa từ vô ích và một đoạn không nên chứa câu vô duyên.', 'RESTRAINT', true),
('0196ba10-0006-7000-8000-000000000005', 'loose_sentence_restraint', 'Tiết chế chuỗi câu ghép dài lỏng lẻo nối liên tiếp bằng các liên từ ''và'', ''nhưng'', ''vì vậy'' gây cảm giác đơn điệu, ru ngủ.', 'RESTRAINT', true),
('0196ba10-0007-7000-8000-000000000001', 'adverb_restraint', 'Tiết chế tối đa trạng từ chỉ cách thức đi kèm động từ (theo Stephen King: trạng từ là biểu hiện của sự bất lực trong việc chọn động từ chính xác).', 'RESTRAINT', true),
('0196ba10-0007-7000-8000-000000000002', 'dialogue_tag_restraint', 'Tiết chế các thẻ thoại cầu kỳ, hoa mỹ (như ''anh ta giận dữ gầm lên'', ''cô ta thổn thức than thở''); ưu tiên dùng thẻ thoại vô hình hoặc hành động kèm theo.', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000001', 'dying_metaphor_restraint', 'Không bao giờ sử dụng những hình ảnh tu từ, phép so sánh quen mòn đã xuất hiện tràn lan trên sách báo thông thường (theo quy tắc George Orwell).', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000002', 'short_word_preference', 'Ưu tiên dùng từ ngữ ngắn gọn, gãy gọn; không bao giờ dùng từ dài dòng hoa mỹ khi từ ngắn có thể diễn đạt trọn vẹn và sáng nghĩa.', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000003', 'radical_word_cutting', 'Nếu có thể cắt bỏ bớt một từ trong câu mà không làm suy giảm ý nghĩa, hãy luôn luôn kiên quyết cắt bỏ từ đó.', 'RESTRAINT', true),
('0196ba10-0008-7000-8000-000000000004', 'jargon_loanword_restraint', 'Tiết chế từ ngữ kỹ thuật chuyên ngành (jargon), thuật ngữ học thuật hoặc từ ngoại lai xa lạ khi đã có từ đời thường tương đương.', 'RESTRAINT', true),
('0196ba10-0009-7000-8000-000000000002', 'skip_boring_parts', 'Lược bỏ những phần nhàm chán mà độc giả có xu hướng lướt qua (theo Elmore Leonard): không tả thời tiết mở đầu dông dài, tả tĩnh vật vô thưởng vô phạt hay đường đi nước bước tẻ nhạt.', 'RESTRAINT', true),
('0196ba10-0009-7000-8000-000000000003', 'exclamation_point_restraint', 'Kiểm soát nghiêm ngặt dấu chấm than (!); không dùng dấu câu để cưỡng ép tạo kịch tính giả tạo cho lời thoại và câu văn.', 'RESTRAINT', true),
('0196ba10-000a-7000-8000-000000000005', 'worldbuilding_infodump_restraint', 'Tuyệt đối tiết chế việc xả hàng khối thông tin bối cảnh lịch sử, địa lý hay cơ chế phép thuật (infodump) tách rời khỏi hành động của nhân vật.', 'RESTRAINT', true),
('0196ba10-000b-7000-8000-000000000003', 'telegraphed_twist_restraint', 'Tiết chế việc gợi mở quá lộ liễu (over-telegraphing) trước cú ngoặt cốt truyện; giữ cho các manh mối ẩn mình tự nhiên dưới mục đích bề nổi của cảnh.', 'RESTRAINT', true),
('0196ba10-000c-7000-8000-000000000004', 'premature_exposition_restraint', 'Kiềm chế việc giải thích nguyên nhân quá sớm; hãy để người đọc tự đặt câu hỏi và khao khát tìm câu trả lời trước khi cung cấp lời giải đáp.', 'RESTRAINT', true),
('0196ba10-000d-7000-8000-000000000003', 'omniscient_spoiler_restraint', 'Khi sử dụng góc nhìn toàn tri tạo kịch tính, tiết chế việc tiết lộ toàn bộ kết cục để vẫn duy trì được câu hỏi nhân vật sẽ ứng biến ra sao.', 'RESTRAINT', true),
('0196ba10-000e-7000-8000-000000000004', 'cheap_lie_restraint', 'Tiết chế việc để người kể chuyện nói dối trắng trợn vô căn cứ với độc giả; sự che giấu thông tin phải bắt nguồn từ động cơ tâm lý hoặc chấn thương của nhân vật.', 'RESTRAINT', true),
('0196ba10-000f-7000-8000-000000000003', 'unfired_detail_restraint', 'Tiết chế việc miêu tả tỉ mỉ các đồ vật, đạo cụ hoặc chi tiết nhân vật phụ nếu chúng không mang lại giá trị cốt truyện, chủ đề hay cảm xúc nào về sau.', 'RESTRAINT', true),
('0196ba10-0010-7000-8000-000000000003', 'dead_end_sub_plot_restraint', 'Tiết chế việc kéo dài manh mối giả (red herring) quá lâu mà không đóng góp cho sự phát triển nhân vật hay hé lộ thông tin phụ nào, tránh gây cảm giác bị lừa rẻ tiền.', 'RESTRAINT', true),
('0196ba10-0011-7000-8000-000000000001', 'infodump_restraint', 'Tuyệt đối ngăn chặn việc tạm dừng mạch truyện để thuyết minh bách khoa toàn thư về thiết lập thế giới hay quá khứ nhân vật.', 'RESTRAINT', true),
('0196ba10-0011-7000-8000-000000000002', 'as_you_know_bob_restraint', 'Tránh triệt để lối thoại gượng gạo ''Như anh đã biết đấy...'' (As You Know, Bob), nơi hai nhân vật giảng giải cho nhau nghe những điều mà cả hai vốn đều đã biết rõ.', 'RESTRAINT', true),
('0196ba10-0012-7000-8000-000000000003', 'tangent_subplot_restraint', 'Tiết chế việc sa đà vào các nhánh cốt truyện phụ hoặc hồi ức quá khứ (flashbacks) lan man làm loãng xung đột trọng tâm và làm chùng nhịp chương.', 'RESTRAINT', true);

-- 1.3. Category: INFORMATION_CONTROL (Kiểm soát thông tin)
INSERT INTO narrative_directives (id, code, description, category, is_system) VALUES
('0196ba10-0001-7000-8000-000000000005', 'subtextual_emotion_reveal', 'Kiểm soát luồng cảm xúc ngầm (subtext): để độc giả tự nhận biết mâu thuẫn tâm lý nhân vật thông qua độ chênh lệch giữa lời nói dối bề ngoài và hành vi vô thức bên trong.', 'INFORMATION_CONTROL', true),
('0196ba10-0002-7000-8000-000000000005', 'pov_epistemic_boundary', 'Giới hạn nghiêm ngặt tầm nhìn thông tin trong phạm vi những gì nhân vật điểm nhìn trực tiếp quan sát, cảm nhận hoặc suy đoán hợp lý tại thời điểm sự kiện diễn ra.', 'INFORMATION_CONTROL', true),
('0196ba10-0009-7000-8000-000000000004', 'sparse_physical_description', 'Tiết chế liệt kê trang phục và dung mạo từ đầu đến chân; chỉ hé lộ 1-2 đặc điểm nhận dạng nổi bật mang tính biểu tượng để độc giả tự hoàn thiện hình dung.', 'INFORMATION_CONTROL', true),
('0196ba10-0009-7000-8000-000000000005', 'prologue_backstory_restraint', 'Tránh mở đầu bằng chương dẫn nhập (prologue) giải thích lịch sử dài dòng; ném nhân vật vào biến cố trực diện và hé lộ quá khứ nhỏ giọt khi thực sự cần thiết.', 'INFORMATION_CONTROL', true),
('0196ba10-000a-7000-8000-000000000002', 'earned_exposition_grounding', 'Chỉ cung cấp quy tắc thế giới hoặc cơ chế sức mạnh khi thông tin đó trực tiếp giải quyết vấn đề trước mắt hoặc tạo thành chướng ngại thực tế cho nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000a-7000-8000-000000000003', 'layered_foreshadowing', 'Gieo mầm phục bút (foreshadowing) theo nhiều tầng tinh tế từ sớm, để khi cú ngoặt bùng nổ, độc giả cảm thấy ngỡ ngàng nhưng hoàn toàn hợp lý và thỏa đáng.', 'INFORMATION_CONTROL', true),
('0196ba10-000a-7000-8000-000000000004', 'promise_progress_payoff_pacing', 'Điều tiết nhịp độ thông tin theo chu trình Lời hứa (Promise) -> Tiến triển (Progress) -> Đền đáp (Payoff), bảo đảm mỗi phân cảnh đều tạo ra sự dịch chuyển thông tin.', 'INFORMATION_CONTROL', true),
('0196ba10-000b-7000-8000-000000000001', 'information_gap_tension', 'Duy trì sức hút trang sách bằng cách chủ động tạo ra các khoảng trống thông tin then chốt và kiểm soát tốc độ lấp đầy câu trả lời cho người đọc.', 'INFORMATION_CONTROL', true),
('0196ba10-000b-7000-8000-000000000002', 'escalation_via_revelation', 'Mỗi lần hé lộ một sự thật mới (revelation) phải đóng vai trò như một đòn bẩy nâng cao cọc cược hiểm nguy hoặc làm đảo lộn cục diện của nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000c-7000-8000-000000000001', 'revelation_over_infodump', 'Thay thế các đoạn thuyết minh tĩnh bằng chuỗi khám phá động thông qua quá trình điều tra, va chạm và lần mò dấu vết của nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000c-7000-8000-000000000002', 'delayed_reveal_suspense', 'Trì hoãn có chủ đích thời điểm tung ra câu trả lời (delayed reveal) bằng cách cài cắm trở ngại hợp lý để kéo căng cảm giác hồi hộp chờ đợi.', 'INFORMATION_CONTROL', true),
('0196ba10-000d-7000-8000-000000000001', 'dramatic_irony_asymmetry', 'Tạo ra sự bất cân xứng thông tin có chủ đích (Dramatic Irony): cho độc giả biết trước hiểm họa hoặc bí mật mà nhân vật trong cảnh hoàn toàn chưa hay biết.', 'INFORMATION_CONTROL', true),
('0196ba10-000d-7000-8000-000000000002', 'anticipation_over_surprise', 'Ưu tiên xây dựng cảm giác thắc thỏm âu lo (anticipation/dread) thông qua sự bất đối xứng thông tin thay vì chỉ dựa vào những cú giật mình bất ngờ chốc lát.', 'INFORMATION_CONTROL', true),
('0196ba10-000e-7000-8000-000000000001', 'unreliable_perspective_filtering', 'Kiểm soát thông tin qua lăng kính người kể chuyện không đáng tin: che giấu, bóp méo sự thật thông qua định kiến, chấn thương hoặc sự tự lừa dối của nhân vật.', 'INFORMATION_CONTROL', true),
('0196ba10-000e-7000-8000-000000000002', 'breadcrumbs_of_contradiction', 'Cài cắm các chi tiết mâu thuẫn khách quan nhỏ để người đọc tinh ý nhận ra sự lệch chuẩn giữa lời trần thuật của nhân vật chính và thực tế diễn ra.', 'INFORMATION_CONTROL', true),
('0196ba10-000f-7000-8000-000000000001', 'chekhovs_gun_setup', 'Áp dụng nguyên lý khẩu súng của Chekhov: mọi chi tiết, bảo vật hoặc kỹ năng được nhấn mạnh ở hồi đầu bắt buộc phải được kích hoạt và giải quyết vấn đề ở hồi sau.', 'INFORMATION_CONTROL', true),
('0196ba10-000f-7000-8000-000000000002', 'camouflage_setup_in_scene', 'Ngụy trang chi tiết phục bút (Chekhov''s Gun) ẩn bên dưới một mục đích thông thường khác của phân cảnh để độc giả không sớm nhận ra dụng ý sắp đặt.', 'INFORMATION_CONTROL', true),
('0196ba10-0010-7000-8000-000000000001', 'red_herring_misdirection', 'Sử dụng kỹ thuật tung hỏa mù (Red Herring): cài cắm manh mối giả và đối tượng khả nghi hợp lý để che chắn cho đầu mối chân chính đang hiện diện.', 'INFORMATION_CONTROL', true),
('0196ba10-0010-7000-8000-000000000002', 'mystery_clue_pacing', 'Phân bổ nhịp độ hé lộ manh mối thật và manh mối giả đan xen có trật tự, duy trì trạng thái suy luận và lật mở giả thuyết liên tục của người đọc.', 'INFORMATION_CONTROL', true),
('0196ba10-0011-7000-8000-000000000003', 'iceberg_worldbuilding_control', 'Áp dụng nguyên lý tảng băng trôi (Iceberg Theory): chỉ phơi bày phần nổi gắn trực tiếp với cốt truyện, để phần chìm đồ sộ của thế giới tự toát lên qua chi tiết nhỏ.', 'INFORMATION_CONTROL', true),
('0196ba10-0012-7000-8000-000000000002', 'atmospheric_foreshadowing', 'Sử dụng không khí, thanh âm và sự biến đổi của bối cảnh không gian để dự báo ngầm về thảm họa sắp giáng xuống trước khi sự việc thực sự bùng nổ.', 'INFORMATION_CONTROL', true);


-- ==============================================================================
-- 2. GỢI Ý LIÊN KẾT STYLE_PRESET_NARRATIVES (MẪU CẤU HÌNH PHONG CÁCH TỰ SỰ)
-- Gán trọng số NarrativeWeight (NONE, LOW, MEDIUM, HIGH) cho các preset phong cách chuẩn.
-- ==============================================================================

-- Ví dụ Preset: "Hành văn Tiểu thuyết mạng Nhịp độ cao (Fast-Paced Webnovel)"
-- style_preset_id: '0196ba00-0001-7000-8000-000000000001'
-- INSERT INTO style_preset_narratives (id, style_preset_id, narrative_directives_id, weight) VALUES
-- ('0196ba20-0001-7000-8000-000000000001', '0196ba00-0001-7000-8000-000000000001', '0196ba10-0001-7000-8000-000000000001', 'HIGH'),
-- ('0196ba20-0001-7000-8000-000000000002', '0196ba00-0001-7000-8000-000000000001', '0196ba10-0003-7000-8000-000000000001', 'HIGH'),
-- ('0196ba20-0001-7000-8000-000000000003', '0196ba00-0001-7000-8000-000000000001', '0196ba10-0005-7000-8000-000000000001', 'HIGH'),
-- ('0196ba20-0001-7000-8000-000000000004', '0196ba00-0001-7000-8000-000000000001', '0196ba10-0009-7000-8000-000000000002', 'HIGH');

-- Ví dụ Preset: "Văn học Nghệ thuật / Trinh thám Tâm lý Sâu sắc (Deep Literary / Mystery)"
-- style_preset_id: '0196ba00-0002-7000-8000-000000000002'
-- INSERT INTO style_preset_narratives (id, style_preset_id, narrative_directives_id, weight) VALUES
-- ('0196ba20-0002-7000-8000-000000000001', '0196ba00-0002-7000-8000-000000000002', '0196ba10-0002-7000-8000-000000000001', 'HIGH'),
-- ('0196ba20-0002-7000-8000-000000000002', '0196ba00-0002-7000-8000-000000000002', '0196ba10-000d-7000-8000-000000000001', 'HIGH'),
-- ('0196ba20-0002-7000-8000-000000000003', '0196ba00-0002-7000-8000-000000000002', '0196ba10-0010-7000-8000-000000000001', 'HIGH'),
-- ('0196ba20-0002-7000-8000-000000000004', '0196ba00-0002-7000-8000-000000000002', '0196ba10-0011-7000-8000-000000000003', 'HIGH');

PRAGMA user_version = 14;
