-- ============================================================
-- EVENT_BLOCKS SEED
-- Spec: docs/schema/system.dbml (Table event_blocks)
-- Note: 'Khối sự kiện chứa component (trigger)-> state'
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO event_blocks (id, code, component_id, state_id, motif_id)
SELECT gen_random_uuid(), v.code, c.id, s.id, m.id
FROM (VALUES
  ('eb_faust_1', 'inciting_temptation', 'seduced_by_power', 'faustian_bargain'),
  ('eb_faust_2', 'oath_taken', 'hubristic_pride', 'faustian_bargain'),
  ('eb_faust_3', 'secret_discovered', 'haunted_by_guilt', 'faustian_bargain'),
  ('eb_faust_4', 'reckoning_demanded', 'inevitable_downfall', 'faustian_bargain'),
  ('eb_mcorrupt_1', 'inciting_temptation', 'naive_innocence', 'moral_corruption'),
  ('eb_mcorrupt_2', 'fatal_misjudgment', 'seduced_by_power', 'moral_corruption'),
  ('eb_mcorrupt_3', 'taboo_breached', 'moral_decay_hardened', 'moral_corruption'),
  ('eb_mcorrupt_4', 'crisis_confrontation', 'inevitable_downfall', 'moral_corruption'),
  ('eb_relall_1', 'crisis_confrontation', 'guarded_suspicion', 'reluctant_alliance_against_threat'),
  ('eb_relall_2', 'shared_ordeal', 'reluctant_solidarity', 'reluctant_alliance_against_threat'),
  ('eb_relall_3', 'sacrificial_act', 'profound_trust', 'reluctant_alliance_against_threat'),
  ('eb_prohib_1', 'oath_taken', 'hubristic_pride', 'prohibition_broken_on_return'),
  ('eb_prohib_2', 'taboo_breached', 'haunted_by_guilt', 'prohibition_broken_on_return'),
  ('eb_prohib_3', 'anagnorisis_moment', 'shattered_faith', 'prohibition_broken_on_return'),
  ('eb_mirror_1', 'symbolic_manifestation', 'naive_innocence', 'mirrors_reflections'),
  ('eb_mirror_2', 'anagnorisis_moment', 'haunted_by_guilt', 'mirrors_reflections'),
  ('eb_mirror_3', 'crisis_confrontation', 'spiritual_epiphany', 'mirrors_reflections'),
  ('eb_xenia_1', 'oath_taken', 'guarded_suspicion', 'guest_friendship_xenia'),
  ('eb_xenia_2', 'shared_ordeal', 'reluctant_solidarity', 'guest_friendship_xenia'),
  ('eb_xenia_3', 'sacrificial_act', 'profound_trust', 'guest_friendship_xenia')
) AS v(code, comp_code, state_code, motif_code)
JOIN components c ON c.code = v.comp_code
JOIN states s ON s.code = v.state_code
JOIN motifs m ON m.code = v.motif_code
ON CONFLICT (code) DO NOTHING;

COMMIT;
