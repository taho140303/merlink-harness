-- ============================================================
-- STATE_RULES SEED
-- Spec: docs/schema/system.dbml (Table state_rules)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO state_rules (id, code, name)
SELECT gen_random_uuid(), v.code, v.name
FROM (VALUES
  ('external_revelation', 'Hé lộ sự thật từ ngoại cảnh (Chân tướng phơi bày)'),
  ('irreversible_sacrifice', 'Hy sinh không thể vãn hồi (Trả giá đắt giá)'),
  ('cathartic_confrontation', 'Đối đầu trực diện giải tỏa mâu thuẫn (Đỉnh điểm bộc phát)'),
  ('gradual_decay', 'Suy thoái dần theo thời gian (Thời gian bào mòn)'),
  ('reconciliation_gesture', 'Hành động thiện chí / Hòa giải từ đối phương'),
  ('vow_fulfillment', 'Hoàn tất lời thề hoặc nghĩa vụ ràng buộc'),
  ('tragic_loss', 'Mất mát bi kịch cắt đứt mọi hy vọng quay đầu'),
  ('external_sanction', 'Hình phạt hoặc can thiệp từ thế lực cấp trên'),
  ('internal_epiphany', 'Tự nhận thức / Giác ngộ nội tâm sau khủng hoảng')
) AS v(code, name)
ON CONFLICT (code) DO NOTHING;

COMMIT;
