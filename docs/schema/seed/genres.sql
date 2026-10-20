-- ============================================================
-- GENRES SEED
-- Spec: docs/schema/system.dbml (Table genres)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO genres (id, code, name, description)
SELECT gen_random_uuid(), v.code, v.name, v.description
FROM (VALUES
  ('fantasy', 'Kỳ ảo (Fantasy)', 'Thế giới có yếu tố phép thuật, sinh vật thần thoại hoặc quy luật siêu nhiên vượt ngoài thực tại.'),
  ('high_fantasy', 'Sử thi kỳ ảo (High / Epic Fantasy)', 'Xây dựng thế giới hoàn toàn hư cấu với hệ thống phép thuật đồ sộ, lịch sử sâu rộng và xung đột quy mô thế giới.'),
  ('urban_fantasy', 'Kỳ ảo đô thị (Urban Fantasy)', 'Phép thuật, sinh vật kỳ bí tồn tại ngầm bên trong lòng các đô thị hiện đại.'),
  ('dark_fantasy', 'Kỳ ảo u tối (Dark Fantasy)', 'Thế giới huyền bí mang màu sắc u ám, khắc nghiệt, kinh dị và ranh giới đạo đức mập mờ.'),
  ('sci_fi', 'Khoa học viễn tưởng (Sci-Fi)', 'Khai thác công nghệ tương lai, du hành vũ trụ, trí tuệ nhân tạo hoặc các lý thuyết khoa học giả định.'),
  ('cyberpunk', 'Cyberpunk', 'Tương lai công nghệ cao nhưng xã hội suy đồi (High tech, low life), nơi các tập đoàn thống trị và tội phạm công nghệ tràn lan.'),
  ('space_opera', 'Sử thi vũ trụ (Space Opera)', 'Phiêu lưu kỳ vĩ giữa các vì sao, chiến tranh giữa các đế chế thiên hà và xung đột chính trị không gian.'),
  ('post_apocalyptic', 'Hậu tận thế (Post-Apocalyptic)', 'Bối cảnh sau thảm họa diệt vong (chiến tranh hạt nhân, dịch bệnh, thiên tai), con người đấu tranh để sinh tồn giữa đống đổ nát.'),
  ('mystery', 'Huyền bí / Trinh thám (Mystery)', 'Tập trung vào giải mã bí ẩn, điều tra vụ án, phát hiện âm mưu và truy tìm chân tướng.'),
  ('thriller', 'Giật gân (Thriller)', 'Nhịp độ dồn dập, hồi hộp, hiểm nguy rình rập và các cuộc đấu trí căng thẳng đến nghẹt thở.'),
  ('psychological_thriller', 'Giật gân tâm lý (Psychological Thriller)', 'Khai thác sự bất ổn tâm thần, thao túng tâm lý, ảo giác và ranh giới mong manh giữa thực tại với hoang tưởng.'),
  ('horror', 'Kinh dị (Horror)', 'Khơi gợi nỗi sợ hãi, kinh hoàng thông qua các thực thể tà ác, siêu nhiên hoặc bản chất đen tối của con người.'),
  ('romance', 'Lãng mạn (Romance)', 'Đặt mối quan hệ tình cảm, tình yêu đôi lứa và sự gắn kết cảm xúc làm trọng tâm câu chuyện.'),
  ('historical_fiction', 'Lịch sử (Historical Fiction)', 'Lấy bối cảnh thời kỳ lịch sử có thật, kết hợp nhân vật hoặc sự kiện hư cấu với bức tranh thời đại.'),
  ('wuxia', 'Võ hiệp (Wuxia)', 'Thế giới giang hồ, tinh thần nghĩa hiệp, ân oán giang hồ và võ học cổ truyền.'),
  ('xianxia', 'Tiên hiệp / Tu chân (Xianxia)', 'Hành trình tu tiên, nghịch thiên cải mệnh, lĩnh hội đạo pháp và vượt thoát luân hồi.'),
  ('slice_of_life', 'Đời thường (Slice of Life)', 'Miêu tả chân thực những lát cắt giản dị, nhẹ nhàng trong cuộc sống thường nhật và các mối quan hệ con người.'),
  ('political_drama', 'Kịch tính chính trị (Political Drama)', 'Đấu tranh quyền lực, mưu lược cung đình, tranh giành vị thế và xung đột ý thức hệ.'),
  ('adventure', 'Phiêu lưu (Adventure)', 'Hành trình khám phá những vùng đất mới, đối mặt cạm bẫy thiên nhiên và tìm kiếm kho báu/chân lý.'),
  ('comedy', 'Hài hước (Comedy)', 'Sử dụng tình huống trớ trêu, châm biếm và đối thoại dí dỏm để mang lại tiếng cười và góc nhìn châm biến.')
) AS v(code, name, description)
ON CONFLICT (code) DO NOTHING;

COMMIT;
