-- ============================================================
-- RELATIONSHIP_CATEGORIES SEED
-- Generated: 2026-10-18T16:20:03Z
-- Source: docs/schema/reference/enum-relationship-type.md
-- Spec: docs/schema/system.dbml (Table relationship_categories)
-- ============================================================
-- Usage:
--   psql -d <db> -f docs/schema/seed/relationship_categories/_seed.sql
-- Requirements:
--   - PostgreSQL 13+ (gen_random_uuid)
--   - Idempotent: re-run skips existing sub_type
-- Notes:
--   - DBML `sub_type` column holds values the md calls "type"
--     (father, mother, lover, ...). The md's "sub_type"
--     (biological/adopted/step/half/foster/god/chosen) is
--     FAMILY-only metadata NOT representable in current DBML;
--     preserved in comment block at bottom.
--   - Default hierarchy (superior/equal/inferior) per sub_type
--     is metadata for relationship_roles.hierarchy; NOT
--     representable per-category in DBML yet. Preserved in
--     comment block.
--   - 43 rows total: 13 FAMILY + 7 ROMANTIC + 8 SOCIAL
--     + 8 PROFESSIONAL + 7 ANTAGONISTIC.
-- ============================================================
BEGIN;

-- FAMILY (13)
INSERT INTO relationship_categories (id, category, sub_type) VALUES
  (gen_random_uuid(), 'FAMILY', 'father'),
  (gen_random_uuid(), 'FAMILY', 'mother'),
  (gen_random_uuid(), 'FAMILY', 'older_brother'),
  (gen_random_uuid(), 'FAMILY', 'older_sister'),
  (gen_random_uuid(), 'FAMILY', 'younger_brother'),
  (gen_random_uuid(), 'FAMILY', 'younger_sister'),
  (gen_random_uuid(), 'FAMILY', 'son'),
  (gen_random_uuid(), 'FAMILY', 'daughter'),
  (gen_random_uuid(), 'FAMILY', 'grandfather'),
  (gen_random_uuid(), 'FAMILY', 'grandmother'),
  (gen_random_uuid(), 'FAMILY', 'uncle'),
  (gen_random_uuid(), 'FAMILY', 'aunt'),
  (gen_random_uuid(), 'FAMILY', 'cousin')
ON CONFLICT (sub_type) DO NOTHING;

-- ROMANTIC (7)
INSERT INTO relationship_categories (id, category, sub_type) VALUES
  (gen_random_uuid(), 'ROMANTIC', 'lover'),
  (gen_random_uuid(), 'ROMANTIC', 'spouse'),
  (gen_random_uuid(), 'ROMANTIC', 'fiance'),
  (gen_random_uuid(), 'ROMANTIC', 'ex_lover'),
  (gen_random_uuid(), 'ROMANTIC', 'crush'),
  (gen_random_uuid(), 'ROMANTIC', 'soulmate'),
  (gen_random_uuid(), 'ROMANTIC', 'unrequited_love')
ON CONFLICT (sub_type) DO NOTHING;

-- SOCIAL (8)
INSERT INTO relationship_categories (id, category, sub_type) VALUES
  (gen_random_uuid(), 'SOCIAL', 'friend'),
  (gen_random_uuid(), 'SOCIAL', 'best_friend'),
  (gen_random_uuid(), 'SOCIAL', 'acquaintance'),
  (gen_random_uuid(), 'SOCIAL', 'neighbor'),
  (gen_random_uuid(), 'SOCIAL', 'classmate'),
  (gen_random_uuid(), 'SOCIAL', 'roommate'),
  (gen_random_uuid(), 'SOCIAL', 'ally'),
  (gen_random_uuid(), 'SOCIAL', 'comrade')
ON CONFLICT (sub_type) DO NOTHING;

-- PROFESSIONAL (8)
INSERT INTO relationship_categories (id, category, sub_type) VALUES
  (gen_random_uuid(), 'PROFESSIONAL', 'boss'),
  (gen_random_uuid(), 'PROFESSIONAL', 'subordinate'),
  (gen_random_uuid(), 'PROFESSIONAL', 'colleague'),
  (gen_random_uuid(), 'PROFESSIONAL', 'mentor'),
  (gen_random_uuid(), 'PROFESSIONAL', 'student'),
  (gen_random_uuid(), 'PROFESSIONAL', 'apprentice'),
  (gen_random_uuid(), 'PROFESSIONAL', 'servant'),
  (gen_random_uuid(), 'PROFESSIONAL', 'master')
ON CONFLICT (sub_type) DO NOTHING;

-- ANTAGONISTIC (7)
INSERT INTO relationship_categories (id, category, sub_type) VALUES
  (gen_random_uuid(), 'ANTAGONISTIC', 'rival'),
  (gen_random_uuid(), 'ANTAGONISTIC', 'enemy'),
  (gen_random_uuid(), 'ANTAGONISTIC', 'nemesis'),
  (gen_random_uuid(), 'ANTAGONISTIC', 'traitor'),
  (gen_random_uuid(), 'ANTAGONISTIC', 'betrayer'),
  (gen_random_uuid(), 'ANTAGONISTIC', 'hunter'),
  (gen_random_uuid(), 'ANTAGONISTIC', 'prey')
ON CONFLICT (sub_type) DO NOTHING;

COMMIT;

-- ============================================================
-- PRESERVED METADATA (not in DBML yet)
-- ============================================================
-- Default hierarchy per sub_type (for relationship_roles.hierarchy):
--   SUPERIOR: father, mother, older_brother, older_sister,
--             grandfather, grandmother, uncle, aunt,
--             boss, mentor, master
--   INFERIOR: son, daughter, younger_brother, younger_sister,
--             subordinate, student, apprentice, servant
--   EQUAL:    cousin, lover, spouse, fiance, ex_lover, crush,
--             soulmate, unrequited_love, friend, best_friend,
--             acquaintance, neighbor, classmate, roommate,
--             ally, comrade, colleague, rival, enemy, nemesis,
--             traitor, betrayer, hunter, prey
--
-- FAMILY sub_type enum (biological/adopted/step/half/foster/
-- god/chosen): orthogonal dimension per relationship_roles;
-- current DBML relationship_categories has no column for it.
-- Sprint +1 may add column `family_subtype_enum` or extend
-- relationship_categories with `data json`.
--
-- Names/examples per sub_type (md reference table): preserved
-- verbatim in docs/schema/reference/enum-relationship-type.md.
