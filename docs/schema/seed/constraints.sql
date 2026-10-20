-- ============================================================
-- CONSTRAINTS SEED
-- Spec: docs/schema/system.dbml (Table constraints)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO constraints (id, code, require, reason)
SELECT gen_random_uuid(), v.code, v.require, v.reason
FROM (VALUES
  ('prior_bond_established', 
   'Mối liên kết tin cậy hoặc tình cảm phải được xây dựng rõ ràng trước đó', 
   'Cú sốc phản bội hoặc rạn nứt chỉ tạo ra sức nặng cảm xúc khi sự gắn kết ban đầu đủ chân thực.'),

  ('irrevocable_cost', 
   'Cái giá phải trả cho lựa chọn hoặc giao kèo là vĩnh viễn, không thể hoàn tác dễ dàng', 
   'Ngăn chặn sự giải quyết hời hợt (deus ex machina) và giữ vững trọng lượng đạo đức của quyết định.'),

  ('established_taboo', 
   'Quy tắc cấm đoán hoặc ranh giới cấm kỵ phải được cảnh báo minh bạch từ trước', 
   'Hành vi vi phạm cấm kỵ chỉ tạo ra bi kịch khi nhân vật hành động dù đã được báo trước hậu quả.'),

  ('asymmetric_information', 
   'Một bên nắm giữ sự thật then chốt mà bên kia hoàn toàn bị che mắt hoặc hiểu lầm', 
   'Tạo lực đẩy tự sự cho tình huống ngộ nhận, điều tra hoặc âm mưu ẩn giấu.'),

  ('foreshadowed_flaw', 
   'Khiếm khuyết nội tâm hoặc điểm yếu chí mạng phải được gieo mầm từ sớm', 
   'Đảm bảo sự sụp đổ hoặc sa ngã của nhân vật xuất phát từ tính cách tất yếu chứ không phải ngẫu nhiên.'),

  ('mutual_dependence', 
   'Hai bên buộc phải nương tựa hoặc hợp tác vì mục tiêu sống còn dù bất đồng sâu sắc', 
   'Tạo ra căng thẳng tương tác và ngăn nhân vật tách rời nhau quá sớm.'),

  ('visible_physical_anchor', 
   'Biểu tượng hoặc tín vật phải xuất hiện vật lý cụ thể trong các bối cảnh then chốt', 
   'Giúp người đọc liên kết ý niệm trừu tượng với hình ảnh thị giác cụ thể qua từng biến cố.'),

  ('inescapable_deadline', 
   'Giới hạn thời gian cụ thể hoặc thời khắc định mệnh không thể trì hoãn', 
   'Gia tăng áp lực dồn dập, buộc nhân vật phải ra quyết định dưới điều kiện ngặt nghèo.'),

  ('credible_antagonist_leverage', 
   'Đối trọng hoặc thế lực cám dỗ phải sở hữu quyền năng hoặc đòn bẩy thực tế đối với khao khát của nhân vật', 
   'Làm cho sự thỏa hiệp hoặc cám dỗ trở nên có sức thuyết phục và hợp lý về mặt tâm lý.'),

  ('unreturned_origin_state', 
   'Sau hành trình biến cố, trạng thái ban đầu không bao giờ có thể khôi phục nguyên vẹn', 
   'Khẳng định tính chuyển hóa tất yếu của nhân vật và chiều sâu của trải nghiệm tự sự.')
) AS v(code, require, reason)
ON CONFLICT (code) DO NOTHING;

COMMIT;
