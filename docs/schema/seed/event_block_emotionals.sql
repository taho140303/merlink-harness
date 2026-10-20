-- ============================================================
-- EVENT_BLOCK_EMOTIONALS SEED
-- Spec: docs/schema/system.dbml (Table event_block_emotionals)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by (event_block_id, emotional_range_id).
-- ============================================================
BEGIN;

INSERT INTO event_block_emotionals (id, emotional_range_id, event_block_id)
SELECT gen_random_uuid(), er.id, eb.id
FROM (VALUES
  ('eb_faust_1', 'hope_anticipation'),
  ('eb_faust_1', 'tension_anxiety'),
  ('eb_faust_2', 'euphoria_triumph'),
  ('eb_faust_2', 'paranoia_suspicion'),
  ('eb_faust_3', 'dread_foreboding'),
  ('eb_faust_3', 'confusion_disorientation'),
  ('eb_faust_4', 'resignation_despair'),
  ('eb_faust_4', 'intense_grief'),
  ('eb_mcorrupt_1', 'tension_anxiety'),
  ('eb_mcorrupt_2', 'paranoia_suspicion'),
  ('eb_mcorrupt_2', 'euphoria_triumph'),
  ('eb_mcorrupt_3', 'bitterness_resentment'),
  ('eb_mcorrupt_4', 'resignation_despair'),
  ('eb_mcorrupt_4', 'fury_indignation'),
  ('eb_relall_1', 'tension_anxiety'),
  ('eb_relall_1', 'paranoia_suspicion'),
  ('eb_relall_2', 'confusion_disorientation'),
  ('eb_relall_2', 'hope_anticipation'),
  ('eb_relall_3', 'awe_wonder'),
  ('eb_relall_3', 'catharsis_relief'),
  ('eb_prohib_1', 'hope_anticipation'),
  ('eb_prohib_1', 'tension_anxiety'),
  ('eb_prohib_2', 'dread_foreboding'),
  ('eb_prohib_2', 'confusion_disorientation'),
  ('eb_prohib_3', 'intense_grief'),
  ('eb_prohib_3', 'resignation_despair'),
  ('eb_mirror_1', 'melancholy_wistfulness'),
  ('eb_mirror_2', 'dread_foreboding'),
  ('eb_mirror_2', 'paranoia_suspicion'),
  ('eb_mirror_3', 'catharsis_relief'),
  ('eb_mirror_3', 'serenity'),
  ('eb_xenia_1', 'tension_anxiety'),
  ('eb_xenia_1', 'serenity'),
  ('eb_xenia_2', 'hope_anticipation'),
  ('eb_xenia_3', 'awe_wonder'),
  ('eb_xenia_3', 'catharsis_relief')
) AS v(eb_code, er_code)
JOIN event_blocks eb ON eb.code = v.eb_code
JOIN emotional_ranges er ON er.code = v.er_code
ON CONFLICT (event_block_id, emotional_range_id) DO NOTHING;

COMMIT;
