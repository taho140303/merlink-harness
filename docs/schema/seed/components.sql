-- ============================================================
-- COMPONENTS SEED
-- Spec: docs/schema/system.dbml (Table components)
-- Note: 'Để trigger states phải transition qua một eventblock khác'
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO components (id, code, name, description)
SELECT gen_random_uuid(), v.code, v.name, v.description
FROM (VALUES
  ('inciting_temptation', 'Cám dỗ mồi nhử', 'Xuất hiện cơ hội hoặc đề nghị hấp dẫn đánh trúng khao khát cháy bỏng nhất của nhân vật.'),
  ('oath_taken', 'Thiết lập lời thề / Giao ước', 'Nhân vật cam kết một lời thề danh dự, khế ước bằng máu hoặc thỏa thuận mang tính ràng buộc.'),
  ('secret_discovered', 'Bí mật bị phát giác', 'Một sự thật then chốt bị che giấu bất ngờ bị đối phương hoặc nhân vật chính phát hiện.'),
  ('taboo_breached', 'Phá vỡ ranh giới cấm kỵ', 'Nhân vật cố tình hoặc vô ý phạm vào điều răn cấm, mở ra chuỗi hậu quả không thể đảo ngược.'),
  ('crisis_confrontation', 'Biến cố đối đầu trực diện', 'Hai lực lượng đối lập hoặc hai nhân vật chạm trán gay gắt, đẩy mâu thuẫn lên đỉnh điểm.'),
  ('fatal_misjudgment', 'Quyết định sai lầm chí mạng', 'Nhân vật đưa ra lựa chọn tai hại do kiêu ngạo, thiếu thông tin hoặc bị che mắt bởi định kiến.'),
  ('sacrificial_act', 'Hành động xả thân hy sinh', 'Một nhân vật tự nguyện từ bỏ điều quý giá nhất hoặc mạng sống để bảo vệ người khác hay lý tưởng.'),
  ('symbolic_manifestation', 'Hiện diện biểu tượng then chốt', 'Vật phẩm hoặc hình ảnh biểu tượng xuất hiện trong một ngữ cảnh kịch tính, gợi mở tầng ý nghĩa sâu xa.'),
  ('betrayal_unveiled', 'Lộ diện phản bội', 'Hành vi phản phúc từ đồng minh thân cận nhất bị phơi bày trước ánh sáng.'),
  ('shared_ordeal', 'Trải qua hoạn nạn chung', 'Các nhân vật cùng bị dồn vào tình thế sinh tử, buộc phải phối hợp và thấu hiểu lẫn nhau.'),
  ('anagnorisis_moment', 'Thời khắc thức tỉnh chân tướng', 'Nhân vật thấu hiểu danh tính thực sự của bản thân hoặc bản chất thực của tình thế xung quanh.'),
  ('reckoning_demanded', 'Đòi hỏi sự trả giá / Phán xét', 'Thế lực công lý, định mệnh hoặc nạn nhân yêu cầu sự đền bù và phán quyết sòng phẳng.')
) AS v(code, name, description)
ON CONFLICT (code) DO NOTHING;

COMMIT;
