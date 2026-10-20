-- ============================================================
-- STATE_DOWN_BY_RULES SEED
-- Spec: docs/schema/system.dbml (Table state_down_by_rules)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); foreign keys to states & state_rules.
-- ============================================================
BEGIN;

INSERT INTO state_down_by_rules (id, state_id, state_rule_id, is_allowed)
SELECT 
  gen_random_uuid(), 
  s.id, 
  r.id, 
  v.is_allowed
FROM (VALUES
  -- naive_innocence: Có thể hạ/thoát trạng thái qua phơi bày sự thật hoặc mất mát bi kịch
  ('naive_innocence', 'external_revelation', true),
  ('naive_innocence', 'tragic_loss', true),
  ('naive_innocence', 'reconciliation_gesture', false),

  -- seduced_by_power: Thoát được khi có sự trả giá đắt hoặc giác ngộ nội tâm
  ('seduced_by_power', 'irreversible_sacrifice', true),
  ('seduced_by_power', 'internal_epiphany', true),
  ('seduced_by_power', 'reconciliation_gesture', false),

  -- haunted_by_guilt: Có thể giải tỏa qua hy sinh chuộc lỗi, đối đầu trực diện hoặc giác ngộ
  ('haunted_by_guilt', 'irreversible_sacrifice', true),
  ('haunted_by_guilt', 'cathartic_confrontation', true),
  ('haunted_by_guilt', 'internal_epiphany', true),
  ('haunted_by_guilt', 'gradual_decay', false),

  -- moral_decay_hardened: Chỉ bị phá vỡ bởi hình phạt trừng phạt từ bên ngoài hoặc bi kịch hủy diệt
  ('moral_decay_hardened', 'external_sanction', true),
  ('moral_decay_hardened', 'tragic_loss', true),
  ('moral_decay_hardened', 'reconciliation_gesture', false),
  ('moral_decay_hardened', 'internal_epiphany', false),

  -- guarded_suspicion: Có thể hạ xuống nhờ cử chỉ hòa giải chân thành hoặc trải qua hoạn nạn
  ('guarded_suspicion', 'reconciliation_gesture', true),
  ('guarded_suspicion', 'external_revelation', true),

  -- hubristic_pride: Bị hạ gục khi hiện thực phơi bày hoặc mất mát bi kịch
  ('hubristic_pride', 'tragic_loss', true),
  ('hubristic_pride', 'external_sanction', true),
  ('hubristic_pride', 'internal_epiphany', false),

  -- shattered_faith: Cần cử chỉ hòa giải bền bỉ hoặc sự kiện giác ngộ lớn
  ('shattered_faith', 'reconciliation_gesture', true),
  ('shattered_faith', 'internal_epiphany', true)
) AS v(state_code, rule_code, is_allowed)
JOIN states s ON s.code = v.state_code
JOIN state_rules r ON r.code = v.rule_code
ON CONFLICT (state_id, state_rule_id) DO UPDATE SET is_allowed = EXCLUDED.is_allowed;

COMMIT;
