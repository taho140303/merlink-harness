-- ============================================================
-- STATES SEED
-- Spec: docs/schema/system.dbml (Table states)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO states (id, code, name, description)
SELECT gen_random_uuid(), v.code, v.name, v.description
FROM (VALUES
  ('naive_innocence', 'Ngây thơ / Chưa trải sự đời', 'Nhân vật tin vào thế giới lý tưởng, nhìn nhận mọi thứ qua lăng kính đen trắng rõ ràng.'),
  ('seduced_by_power', 'Bị quyền năng / Tham vọng tha hóa', 'Nhân vật bắt đầu thỏa hiệp đạo đức từng bước, tự biện minh cho hành vi sai trái vì mục đích tối thượng.'),
  ('haunted_by_guilt', 'Cắn rứt lương tâm dằn vặt', 'Nhân vật bị ám ảnh bởi lỗi lầm trong quá khứ hoặc hậu quả tàn khốc mà hành vi của mình gây ra.'),
  ('moral_decay_hardened', 'Tha hóa chai sạn / Trơ lì đạo đức', 'Mất đi hoàn toàn sự hối hận; nhân vật chấp nhận tàn nhẫn và coi thủ đoạn là phương tiện duy nhất.'),
  ('guarded_suspicion', 'Đề phòng / Nghi kỵ cảnh giác', 'Nhân vật giữ khoảng cách, soi xét mọi động thái của đối phương, không dễ dàng trao gửi niềm tin.'),
  ('reluctant_solidarity', 'Đoàn kết bất đắc dĩ', 'Tạm thời gạt bỏ hiềm khích để cùng tồn tại hoặc đối đầu với kẻ thù chung nguy hiểm hơn.'),
  ('profound_trust', 'Tin cậy sâu sắc / Đồng cam cộng khổ', 'Hình thành mối liên kết tâm giao, sẵn sàng giao phó tính mạng và bí mật sâu kín nhất cho đối phương.'),
  ('shattered_faith', 'Niềm tin tan vỡ', 'Bị phản bội hoặc vỡ mộng, nhân vật rơi vào trạng thái cô độc, thù ghét hoặc đóng băng cảm xúc.'),
  ('hubristic_pride', 'Tự phụ mù quáng (Hubris)', 'Nhân vật tin rằng mình đứng trên luật lệ hoặc định mệnh, khinh suất trước các dấu hiệu nguy hiểm.'),
  ('inevitable_downfall', 'Sụp đổ không thể cứu vãn', 'Nhân vật mất hết quyền lực, danh dự hoặc người thân, đối diện với kết cục bi thảm của chính mình.'),
  ('spiritual_epiphany', 'Giác ngộ tinh thần / Tỉnh ngộ lương tri', 'Nhân vật nhìn thấu được bản chất cuộc sống, chấp nhận sự tha thứ hoặc thanh thản nhận lãnh trách nhiệm.')
) AS v(code, name, description)
ON CONFLICT (code) DO NOTHING;

COMMIT;
