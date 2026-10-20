-- ============================================================
-- MERLINK FULL DATABASE INITIALIZATION DDL
-- Generated from: docs/schema/system.dbml (Synchronized with SQLite ~/.merlink/merlink.db)
-- Target RDBMS: PostgreSQL 13+ (Uses gen_random_uuid())
-- Order of execution:
--   1. Extensions & Custom Types (ENUMs)
--   2. Tables ordered topologically by Foreign Key dependencies
--   3. Composite & Unique Indexes
-- ============================================================

BEGIN;

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================
-- 1. ENUMS / CUSTOM TYPES (31 Enums)
-- ============================================================

DO $$ BEGIN
    CREATE TYPE "MotifsType" AS ENUM ('character_arc', 'relationship', 'symbolic', 'plot');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "NarrativeWeight" AS ENUM ('NONE', 'LOW', 'MEDIUM', 'HIGH');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "SentenceLength" AS ENUM ('VERY_SHORT', 'SHORT', 'MEDIUM', 'LONG');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "SentenceStructure" AS ENUM ('SIMPLE', 'COMPOUND', 'COMPLEX', 'FRAGMENT', 'MIXED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "TonePace" AS ENUM ('VERY_SLOW', 'SLOW', 'MODERATE', 'FAST', 'VERY_FAST');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "GrammaticalPerson" AS ENUM ('FIRST', 'SECOND', 'THIRD');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "PersonSubtype" AS ENUM (
        'FIRST_CENTRAL', 'FIRST_PERIPHERAL', 'FIRST_RETROSPECTIVE', 'FIRST_STREAM', 'FIRST_EPISTOLARY',
        'SECOND_DIRECT', 'SECOND_INVERTED', 'SECOND_HYPOTHETICAL',
        'THIRD_LIMITED', 'THIRD_OBJECTIVE', 'THIRD_OMNISCIENT', 'THIRD_FREE_INDIRECT', 'THIRD_MULTI_LIMITED'
    );
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "PovScope" AS ENUM ('CLOSE', 'NEUTRAL', 'PANORAMIC', 'EDITORIAL');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "DirectiveCategory" AS ENUM ('PROSE_PRINCIPLE', 'RESTRAINT', 'INFORMATION_CONTROL');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "ContentStatus" AS ENUM ('DRAFT', 'REVISING', 'COMPLETED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "Language" AS ENUM ('VI', 'EN', 'JA', 'ZH_HANT', 'ZH_HANS');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "SceneStatus" AS ENUM ('DRAFT', 'PLANNED', 'OUTLINED', 'BLOCKED', 'DONE');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "SceneSource" AS ENUM ('MANUAL', 'AI', 'IMPORTED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "Gender" AS ENUM ('MALE', 'FEMALE', 'OTHER');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "MeasurementUnit" AS ENUM ('CM', 'KG', 'M', 'LBS', 'INCH');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "Hierarchy" AS ENUM ('SUPERIOR', 'INFERIOR', 'EQUAL');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "RelationDimensionLevel" AS ENUM ('VERY_LOW', 'LOW', 'MODERATE', 'HIGH', 'VERY_HIGH');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "CategoryGroup" AS ENUM ('FAMILY', 'PROFESSIONAL', 'ROMANTIC', 'SOCIAL', 'CONFLICT');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "LocationType" AS ENUM ('ROOM', 'SETTLEMENT', 'LANDMARK', 'WILDERNESS', 'TRANSIT', 'ABSTRACT');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "LocationAccess" AS ENUM ('OPEN', 'RESTRICTED', 'FORBIDDEN', 'HIDDEN', 'LOST');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "LoreType" AS ENUM ('EVENT', 'FACTION', 'ITEM', 'CONCEPT', 'SPECIES', 'CULTURE', 'OTHER');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "ThemeScope" AS ENUM ('GLOBAL', 'LOCAL', 'EPISODIC');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "ThemeResolutionType" AS ENUM ('UPHELD', 'SUBVERTED', 'AMBIGUOUS', 'UNRESOLVED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "ThemeArgumentRole" AS ENUM ('THESIS', 'ANTITHESIS', 'SYNTHESIS');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "ThemeAnswerMode" AS ENUM ('DIDACTIC', 'EXPLORATORY', 'DIALECTICAL');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "MemoryType" AS ENUM ('EPISODIC', 'SEMANTIC', 'PROCEDURAL', 'TRAUMATIC', 'FALSE', 'COLLECTIVE');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "MemoryCertainty" AS ENUM ('ABSOLUTE', 'HIGH', 'MODERATE', 'DOUBTFUL', 'FABRICATED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "MemorySource" AS ENUM ('DIRECT_EXPERIENCE', 'HEARSAY', 'DISCOVERED', 'INHERITED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "MemoryStatus" AS ENUM ('ACTIVE', 'BURIED', 'REPRESSED', 'RESOLVED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "KnowledgeSourceType" AS ENUM ('STUDY', 'PRACTICE', 'INSTINCT', 'EXPERIENCE', 'TRANSMISSION');
EXCEPTION WHEN duplicate_object THEN null; END $$;

DO $$ BEGIN
    CREATE TYPE "CertaintyLevel" AS ENUM ('RUMOR', 'BELIEF', 'ASSUMPTION', 'FACT', 'VERIFIED');
EXCEPTION WHEN duplicate_object THEN null; END $$;

-- ============================================================
-- 2. MASTER & STANDALONE TABLES (Level 0 - No foreign keys)
-- ============================================================

CREATE TABLE IF NOT EXISTS motifs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    category "MotifsType" NOT NULL,
    version varchar,
    is_system boolean DEFAULT false,
    description text,
    min_chapter int,
    max_chapter int
);
CREATE INDEX IF NOT EXISTS idx_motifs_category ON motifs(category);
CREATE INDEX IF NOT EXISTS idx_motifs_is_system ON motifs(is_system);

CREATE TABLE IF NOT EXISTS genres (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    description text
);

CREATE TABLE IF NOT EXISTS states (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    description text
);

CREATE TABLE IF NOT EXISTS state_rules (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS emotional_ranges (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS components (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    description text
);

CREATE TABLE IF NOT EXISTS constraints (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    require varchar NOT NULL,
    reason varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS rhetorical_devices (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    description text
);

CREATE TABLE IF NOT EXISTS style_presets (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    sentence_length "SentenceLength" NOT NULL,
    sentence_structure "SentenceStructure" NOT NULL,
    tone_pace "TonePace" NOT NULL
);

CREATE TABLE IF NOT EXISTS narrator_configs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    person "GrammaticalPerson" NOT NULL,
    subtype "PersonSubtype" NOT NULL,
    pov_scope "PovScope" NOT NULL,
    is_system boolean DEFAULT false,
    description text,
    boundary_instruction text
);

CREATE TABLE IF NOT EXISTS narrative_directives (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    category "DirectiveCategory" NOT NULL,
    description text
);

CREATE TABLE IF NOT EXISTS relationship_categories (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    group_type "CategoryGroup" NOT NULL
);

CREATE TABLE IF NOT EXISTS relationship_roles (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    hierarchy "Hierarchy" NOT NULL,
    description text
);

CREATE TABLE IF NOT EXISTS competence_domains (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    description text
);

CREATE TABLE IF NOT EXISTS appearance_option_groups (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar NOT NULL,
    key varchar UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS symbolics (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid()
);

-- ============================================================
-- 3. PROJECTS & CHAT SESSIONS / MESSAGES (Level 1)
-- Synchronized with ~/.merlink/merlink.db
-- ============================================================

CREATE TABLE IF NOT EXISTS projects (
    id varchar PRIMARY KEY,
    code varchar UNIQUE,
    name varchar,
    title varchar,
    author varchar,
    summarize text,
    language "Language" DEFAULT 'VI',
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_projects_created_at ON projects(created_at);

CREATE TABLE IF NOT EXISTS sessions (
    id varchar PRIMARY KEY,
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title varchar,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_id);
CREATE INDEX IF NOT EXISTS idx_sessions_created_at ON sessions(created_at);

CREATE TABLE IF NOT EXISTS messages (
    id varchar PRIMARY KEY,
    session_id varchar NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    role varchar NOT NULL,
    content text,
    payload_json text,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id);
CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);

-- ============================================================
-- 4. SYSTEM PROMPT LINKING TABLES & DEPENDENTS (Level 1 & 2)
-- ============================================================

CREATE TABLE IF NOT EXISTS motif_applicable_genres (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    motif_id uuid NOT NULL REFERENCES motifs(id) ON DELETE CASCADE,
    genre_id uuid NOT NULL REFERENCES genres(id) ON DELETE CASCADE,
    CONSTRAINT uq_motif_genre UNIQUE (motif_id, genre_id)
);
CREATE INDEX IF NOT EXISTS idx_mag_genre ON motif_applicable_genres(genre_id);

CREATE TABLE IF NOT EXISTS motif_constraints (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE,
    constraint_id uuid NOT NULL REFERENCES constraints(id) ON DELETE CASCADE,
    motif_id uuid NOT NULL REFERENCES motifs(id) ON DELETE CASCADE,
    CONSTRAINT uq_motif_constraint UNIQUE (motif_id, constraint_id)
);
CREATE INDEX IF NOT EXISTS idx_mc_constraint ON motif_constraints(constraint_id);

CREATE TABLE IF NOT EXISTS state_down_by_rules (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    state_id uuid NOT NULL REFERENCES states(id) ON DELETE CASCADE,
    state_rule_id uuid NOT NULL REFERENCES state_rules(id) ON DELETE CASCADE,
    is_allowed boolean NOT NULL,
    CONSTRAINT uq_state_down_rule UNIQUE (state_id, state_rule_id)
);
CREATE INDEX IF NOT EXISTS idx_sdbr_rule ON state_down_by_rules(state_rule_id);

CREATE TABLE IF NOT EXISTS event_blocks (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code varchar UNIQUE NOT NULL,
    component_id uuid NOT NULL REFERENCES components(id) ON DELETE CASCADE,
    state_id uuid NOT NULL REFERENCES states(id) ON DELETE CASCADE,
    motif_id uuid NOT NULL REFERENCES motifs(id) ON DELETE CASCADE,
    CONSTRAINT uq_event_block UNIQUE (motif_id, state_id, component_id)
);
CREATE INDEX IF NOT EXISTS idx_eb_component ON event_blocks(component_id);
CREATE INDEX IF NOT EXISTS idx_eb_state ON event_blocks(state_id);

CREATE TABLE IF NOT EXISTS event_transitions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar,
    from_event_block_id uuid NOT NULL REFERENCES event_blocks(id) ON DELETE CASCADE,
    to_event_block_id uuid NOT NULL REFERENCES event_blocks(id) ON DELETE CASCADE,
    priority int DEFAULT 1,
    CONSTRAINT uq_event_transition UNIQUE (from_event_block_id, to_event_block_id)
);
CREATE INDEX IF NOT EXISTS idx_et_priority ON event_transitions(from_event_block_id, priority);
CREATE INDEX IF NOT EXISTS idx_et_to_block ON event_transitions(to_event_block_id);

CREATE TABLE IF NOT EXISTS event_block_emotionals (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    emotional_range_id uuid NOT NULL REFERENCES emotional_ranges(id) ON DELETE CASCADE,
    event_block_id uuid NOT NULL REFERENCES event_blocks(id) ON DELETE CASCADE,
    CONSTRAINT uq_event_block_emotional UNIQUE (event_block_id, emotional_range_id)
);
CREATE INDEX IF NOT EXISTS idx_ebe_emotional ON event_block_emotionals(emotional_range_id);

CREATE TABLE IF NOT EXISTS style_preset_rhetorical_devices (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    style_preset_id uuid NOT NULL REFERENCES style_presets(id) ON DELETE CASCADE,
    rhetorical_device_id uuid NOT NULL REFERENCES rhetorical_devices(id) ON DELETE CASCADE,
    weight "NarrativeWeight" NOT NULL,
    CONSTRAINT uq_sp_rhetorical UNIQUE (style_preset_id, rhetorical_device_id)
);
CREATE INDEX IF NOT EXISTS idx_sprd_device ON style_preset_rhetorical_devices(rhetorical_device_id);

CREATE TABLE IF NOT EXISTS style_preset_vocab_items (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    style_preset_id uuid NOT NULL REFERENCES style_presets(id) ON DELETE CASCADE,
    item varchar NOT NULL,
    CONSTRAINT uq_sp_vocab UNIQUE (style_preset_id, item)
);

CREATE TABLE IF NOT EXISTS style_preset_focus_items (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    style_preset_id uuid NOT NULL REFERENCES style_presets(id) ON DELETE CASCADE,
    item varchar NOT NULL,
    CONSTRAINT uq_sp_focus UNIQUE (style_preset_id, item)
);

CREATE TABLE IF NOT EXISTS style_preset_narratives (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    style_preset_id uuid NOT NULL REFERENCES style_presets(id) ON DELETE CASCADE,
    narrative_directive_id uuid NOT NULL REFERENCES narrative_directives(id) ON DELETE CASCADE,
    weight "NarrativeWeight" NOT NULL,
    CONSTRAINT uq_sp_narrative UNIQUE (style_preset_id, narrative_directive_id)
);
CREATE INDEX IF NOT EXISTS idx_spn_directive ON style_preset_narratives(narrative_directive_id);

CREATE TABLE IF NOT EXISTS appearance_options (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id uuid NOT NULL REFERENCES appearance_option_groups(id) ON DELETE CASCADE,
    label varchar NOT NULL,
    value varchar NOT NULL,
    CONSTRAINT uq_appearance_group_val UNIQUE (group_id, value)
);

CREATE TABLE IF NOT EXISTS symbolic_motifs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    symbolic_id uuid NOT NULL REFERENCES symbolics(id) ON DELETE CASCADE,
    motif_id uuid NOT NULL REFERENCES motifs(id) ON DELETE CASCADE
);

-- ============================================================
-- 5. PROJECT-LEVEL & WORLD BUILDING TABLES (Level 2)
-- ============================================================

CREATE TABLE IF NOT EXISTS project_motifs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    motif_id uuid NOT NULL REFERENCES motifs(id) ON DELETE CASCADE,
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    scope varchar,
    "from" varchar
);

CREATE TABLE IF NOT EXISTS project_genres (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    genre_id uuid NOT NULL REFERENCES genres(id) ON DELETE CASCADE,
    is_primary boolean DEFAULT false,
    weight int DEFAULT 1,
    CONSTRAINT uq_project_genre UNIQUE (project_id, genre_id)
);

CREATE TABLE IF NOT EXISTS world_settings (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    type "LocationType" NOT NULL,
    parent_id uuid REFERENCES world_settings(id) ON DELETE SET NULL,
    description text,
    access "LocationAccess" NOT NULL,
    atmosphere text,
    significance text
);
CREATE INDEX IF NOT EXISTS idx_ws_parent ON world_settings(parent_id);

CREATE TABLE IF NOT EXISTS lore_entries (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    title varchar NOT NULL,
    type "LoreType" NOT NULL,
    summary text,
    detail text,
    significance text
);

CREATE TABLE IF NOT EXISTS power_systems (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    source text,
    rules text,
    limitations text,
    cost text
);

CREATE TABLE IF NOT EXISTS power_tiers (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    system_id uuid NOT NULL REFERENCES power_systems(id) ON DELETE CASCADE,
    rank int NOT NULL,
    name varchar NOT NULL,
    description text,
    requirements text
);
CREATE INDEX IF NOT EXISTS idx_power_tiers_rank ON power_tiers(system_id, rank);

CREATE TABLE IF NOT EXISTS power_abilities (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    system_id uuid NOT NULL REFERENCES power_systems(id) ON DELETE CASCADE,
    tier_id uuid REFERENCES power_tiers(id) ON DELETE SET NULL,
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    description text,
    cost text,
    conditions text
);

CREATE TABLE IF NOT EXISTS themes (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    statement text,
    scope "ThemeScope" NOT NULL,
    resolution_type "ThemeResolutionType" NOT NULL,
    notes text
);

CREATE TABLE IF NOT EXISTS theme_arguments (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    theme_id uuid NOT NULL REFERENCES themes(id) ON DELETE CASCADE,
    role "ThemeArgumentRole" NOT NULL,
    viewpoint text NOT NULL,
    embodied_by text,
    narrative_function text,
    answer_mode "ThemeAnswerMode"
);
CREATE INDEX IF NOT EXISTS idx_ta_theme ON theme_arguments(theme_id);

-- ============================================================
-- 6. CHARACTERS & NARRATIVE STRUCTURE (Level 2 & 3)
-- ============================================================

CREATE TABLE IF NOT EXISTS characters (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    role varchar,
    gender "Gender" NOT NULL,
    age int,
    biography text,
    summary text,
    height numeric(5,2),
    height_unit "MeasurementUnit" DEFAULT 'CM',
    weight numeric(5,2),
    weight_unit "MeasurementUnit" DEFAULT 'KG',
    blood_type varchar,
    birthday varchar,
    birthplace varchar,
    nationality varchar,
    occupation varchar,
    psychological_profile text,
    motivation text,
    flaw text
);
CREATE INDEX IF NOT EXISTS idx_characters_project ON characters(project_id);

CREATE TABLE IF NOT EXISTS character_motifs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    motif_id uuid NOT NULL REFERENCES motifs(id) ON DELETE CASCADE,
    character_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    CONSTRAINT uq_character_motif UNIQUE (character_id, motif_id)
);
CREATE INDEX IF NOT EXISTS idx_cm_motif ON character_motifs(motif_id);

CREATE TABLE IF NOT EXISTS character_appearances (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    group_id uuid NOT NULL REFERENCES appearance_option_groups(id) ON DELETE CASCADE,
    option_id uuid NOT NULL REFERENCES appearance_options(id) ON DELETE CASCADE,
    custom_value varchar,
    CONSTRAINT uq_char_appearance UNIQUE (character_id, group_id)
);

CREATE TABLE IF NOT EXISTS character_distinguishing_marks (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    mark varchar NOT NULL,
    location varchar,
    origin text
);

CREATE TABLE IF NOT EXISTS relationships (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    character_a_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    character_b_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    category_id uuid NOT NULL REFERENCES relationship_categories(id) ON DELETE CASCADE,
    trust_level "RelationDimensionLevel" NOT NULL,
    affection_level "RelationDimensionLevel" NOT NULL,
    conflict_level "RelationDimensionLevel" NOT NULL,
    power_dynamic "Hierarchy" NOT NULL,
    is_active boolean DEFAULT true,
    established_at varchar,
    notes text,
    CONSTRAINT uq_characters_pair UNIQUE (character_a_id, character_b_id)
);

CREATE TABLE IF NOT EXISTS relationship_motifs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    motif_id uuid NOT NULL REFERENCES motifs(id) ON DELETE CASCADE,
    relationship_id uuid NOT NULL REFERENCES relationships(id) ON DELETE CASCADE,
    CONSTRAINT uq_relationship_motif UNIQUE (relationship_id, motif_id)
);
CREATE INDEX IF NOT EXISTS idx_rm_motif ON relationship_motifs(motif_id);

CREATE TABLE IF NOT EXISTS relationship_states (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    relationship_id uuid NOT NULL REFERENCES relationships(id) ON DELETE CASCADE,
    state varchar NOT NULL,
    started_at timestamp DEFAULT CURRENT_TIMESTAMP,
    ended_at timestamp,
    reason text
);

CREATE TABLE IF NOT EXISTS character_competences (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    domain_id uuid NOT NULL REFERENCES competence_domains(id) ON DELETE CASCADE,
    level varchar NOT NULL,
    CONSTRAINT uq_char_competence UNIQUE (character_id, domain_id)
);

CREATE TABLE IF NOT EXISTS competence_capabilities (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    competence_id uuid NOT NULL REFERENCES character_competences(id) ON DELETE CASCADE,
    content text NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_cc_competence ON competence_capabilities(competence_id);

CREATE TABLE IF NOT EXISTS competence_limitations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    competence_id uuid NOT NULL REFERENCES character_competences(id) ON DELETE CASCADE,
    content text NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_cl_competence ON competence_limitations(competence_id);

CREATE TABLE IF NOT EXISTS character_knowledge_sources (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    source_type "KnowledgeSourceType" NOT NULL,
    certainty "CertaintyLevel" NOT NULL,
    topic text NOT NULL,
    source_detail text
);

-- ============================================================
-- 7. MEMORY SUBSYSTEM (Level 3 & 4)
-- ============================================================

CREATE TABLE IF NOT EXISTS memories (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    type "MemoryType" NOT NULL,
    certainty "MemoryCertainty" NOT NULL,
    source "MemorySource" NOT NULL,
    status "MemoryStatus" NOT NULL,
    content text NOT NULL,
    formation_date varchar,
    impact text
);

CREATE TABLE IF NOT EXISTS memory_references (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    memory_id uuid NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    reference_type varchar NOT NULL,
    reference_id uuid NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_memref_memory ON memory_references(memory_id);

CREATE TABLE IF NOT EXISTS memory_emotions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    memory_id uuid NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    emotional_range_id uuid NOT NULL REFERENCES emotional_ranges(id) ON DELETE CASCADE,
    intensity int DEFAULT 1,
    notes text,
    CONSTRAINT uq_mem_emotion UNIQUE (memory_id, emotional_range_id)
);

CREATE TABLE IF NOT EXISTS memory_connections (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    from_memory_id uuid NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    to_memory_id uuid NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    relationship_type varchar NOT NULL,
    strength int DEFAULT 1,
    CONSTRAINT uq_mem_connection UNIQUE (from_memory_id, to_memory_id)
);

CREATE TABLE IF NOT EXISTS memory_revisions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    memory_id uuid NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    previous_content text NOT NULL,
    revision_date timestamp DEFAULT CURRENT_TIMESTAMP,
    reason text
);

-- ============================================================
-- 8. NARRATIVE PROGRESSION (Arcs, Chapters, Scenes - Level 3, 4, 5)
-- ============================================================

CREATE TABLE IF NOT EXISTS arcs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    name varchar NOT NULL,
    "order" int NOT NULL,
    summary text,
    status "ContentStatus" DEFAULT 'DRAFT'
);
CREATE INDEX IF NOT EXISTS idx_arcs_order ON arcs(project_id, "order");

CREATE TABLE IF NOT EXISTS chapters (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    arc_id uuid NOT NULL REFERENCES arcs(id) ON DELETE CASCADE,
    code varchar UNIQUE NOT NULL,
    title varchar NOT NULL,
    "order" int NOT NULL,
    summary text,
    status "ContentStatus" DEFAULT 'DRAFT'
);
CREATE INDEX IF NOT EXISTS idx_chapters_order ON chapters(arc_id, "order");

CREATE TABLE IF NOT EXISTS scenes (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id varchar NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    chapter_id uuid NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    world_setting_id uuid REFERENCES world_settings(id) ON DELETE SET NULL,
    style_preset_id uuid REFERENCES style_presets(id) ON DELETE SET NULL,
    narrator_config_id uuid REFERENCES narrator_configs(id) ON DELETE SET NULL,
    code varchar UNIQUE NOT NULL,
    title varchar NOT NULL,
    "order" int NOT NULL,
    status "SceneStatus" DEFAULT 'PLANNED',
    source "SceneSource" DEFAULT 'MANUAL',
    summary text,
    content text
);
CREATE INDEX IF NOT EXISTS idx_scenes_order ON scenes(chapter_id, "order");

CREATE TABLE IF NOT EXISTS scene_characters (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    scene_id uuid NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    character_id uuid NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    role_in_scene varchar,
    CONSTRAINT uq_scene_character UNIQUE (scene_id, character_id)
);

CREATE TABLE IF NOT EXISTS scene_versions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    scene_id uuid NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    version_number int NOT NULL,
    content text NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    summary text,
    CONSTRAINT uq_scene_version UNIQUE (scene_id, version_number)
);

COMMIT;
