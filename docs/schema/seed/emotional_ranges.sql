-- ============================================================
-- EMOTIONAL_RANGES SEED
-- Spec: docs/schema/system.dbml (Table emotional_ranges)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO emotional_ranges (id, code, name)
SELECT gen_random_uuid(), v.code, v.name
FROM (VALUES
  ('serenity', 'Thanh thản / Bình yên (Serenity)'),
  ('hope_anticipation', 'Hy vọng / Chờ mong (Hope & Anticipation)'),
  ('tension_anxiety', 'Căng thẳng / Bất an (Tension & Anxiety)'),
  ('dread_foreboding', 'Dự cảm sợ hãi / Rợn ngợp (Dread & Foreboding)'),
  ('intense_grief', 'Đau đớn tột cùng / Tang tóc (Intense Grief)'),
  ('fury_indignation', 'Phẫn nộ / Thịnh nộ (Fury & Indignation)'),
  ('melancholy_wistfulness', 'U buồn / Hoài niệm trầm mặc (Melancholy)'),
  ('euphoria_triumph', 'Thăng hoa / Thắng lợi vẻ vang (Triumph & Euphoria)'),
  ('confusion_disorientation', 'Hoang mang / Mất phương hướng (Confusion)'),
  ('paranoia_suspicion', 'Hoài nghi / Đề phòng cao độ (Suspicion & Paranoia)'),
  ('bitterness_resentment', 'Cay đắng / Hận thù ngấm ngầm (Bitterness)'),
  ('catharsis_relief', 'Giải tỏa / Thanh lọc tâm can (Catharsis & Relief)'),
  ('awe_wonder', 'Kinh ngạc / Trầm trồ trước sự vĩ đại (Awe & Wonder)'),
  ('resignation_despair', 'Tuyệt vọng / Buông xuôi số phận (Despair & Resignation)')
) AS v(code, name)
ON CONFLICT (code) DO NOTHING;

COMMIT;
