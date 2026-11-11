-- 0001_schema.sql - complete physical schema (66 tables, indexes, constraints).
--
-- Consolidated baseline for Merlink Harness. Squashed from the previous 26 forward-only
-- migration files before the first public release; see CONTRIBUTING.md ("Database
-- migrations") for the policy that governs new migrations.


-- 1. motifs (docs/db.dbml:17)
CREATE TABLE motifs (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    category TEXT CHECK (category IS NULL OR category IN ('character_arc', 'relationship', 'symbolic', 'plot')),
    version TEXT,
    is_system INTEGER DEFAULT 0 CHECK (is_system IS NULL OR is_system IN (0, 1)),
    description TEXT,
    min_chapter INTEGER,
    max_chapter INTEGER
);

CREATE INDEX idx_motifs_category ON motifs(category);

CREATE INDEX idx_motifs_is_system ON motifs(is_system);

-- 2. genres (docs/db.dbml:34)
CREATE TABLE genres (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    description TEXT
);

-- 3. motif_applicable_genres (docs/db.dbml:43)
CREATE TABLE motif_applicable_genres (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    motif_id TEXT REFERENCES motifs(id),
    genre_id TEXT REFERENCES genres(id),
    UNIQUE (motif_id, genre_id)
);

CREATE INDEX idx_motif_applicable_genres_genre_id ON motif_applicable_genres(genre_id);

-- 4. states (docs/db.dbml:85)
CREATE TABLE states (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    description TEXT
);

-- 5. state_rules (docs/db.dbml:104)
CREATE TABLE state_rules (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT
);

-- 6. state_down_by_rules (docs/db.dbml:92)
CREATE TABLE state_down_by_rules (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    state_id TEXT REFERENCES states(id),
    state_rule_id TEXT REFERENCES state_rules(id),
    is_allowed INTEGER CHECK (is_allowed IS NULL OR is_allowed IN (0, 1)),
    UNIQUE (state_id, state_rule_id)
);

CREATE INDEX idx_state_down_by_rules_state_rule_id ON state_down_by_rules(state_rule_id);

-- 7. emotional_ranges (docs/db.dbml:110)
CREATE TABLE emotional_ranges (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT
);

-- 8. components (docs/db.dbml:127)
CREATE TABLE components (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    description TEXT
);

-- 9. event_blocks (docs/db.dbml:56)
CREATE TABLE event_blocks (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    component_id TEXT REFERENCES components(id),
    state_id TEXT REFERENCES states(id),
    motif_id TEXT REFERENCES motifs(id),
    UNIQUE (motif_id, state_id, component_id)
);

CREATE INDEX idx_event_blocks_component_id ON event_blocks(component_id);

CREATE INDEX idx_event_blocks_state_id ON event_blocks(state_id);

-- 10. event_transitions (docs/db.dbml:71)
CREATE TABLE event_transitions (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    display_name TEXT,
    from_event_block_id TEXT REFERENCES event_blocks(id),
    to_event_block_id TEXT REFERENCES event_blocks(id),
    priority INTEGER,
    UNIQUE (from_event_block_id, to_event_block_id)
);

CREATE INDEX idx_event_transitions_from_priority ON event_transitions(from_event_block_id, priority);

CREATE INDEX idx_event_transitions_to_event_block_id ON event_transitions(to_event_block_id);

-- 11. event_block_emotionals (docs/db.dbml:116)
CREATE TABLE event_block_emotionals (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    emotional_range_id TEXT REFERENCES emotional_ranges(id),
    event_block_id TEXT REFERENCES event_blocks(id),
    UNIQUE (event_block_id, emotional_range_id)
);

CREATE INDEX idx_event_block_emotionals_emotional_range_id ON event_block_emotionals(emotional_range_id);

-- 12. constraints (docs/db.dbml:150)
CREATE TABLE constraints (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    require TEXT NOT NULL,
    reason TEXT NOT NULL
);

-- 13. motif_constraints (docs/db.dbml:136)
CREATE TABLE motif_constraints (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    constraint_id TEXT REFERENCES constraints(id),
    motif_id TEXT REFERENCES motifs(id),
    syntax TEXT,
    constrain TEXT,
    UNIQUE (motif_id, constraint_id)
);

CREATE INDEX idx_motif_constraints_constraint_id ON motif_constraints(constraint_id);

-- 14. style_presets (docs/db.dbml:189)
CREATE TABLE style_presets (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    category TEXT,
    tone_pace TEXT DEFAULT 'VERY_LOW' CHECK (tone_pace IS NULL OR tone_pace IN ('VERY_LOW', 'LOW', 'MODERATE', 'FAST', 'VERY_FAST')),
    tone_emotion TEXT,
    is_system INTEGER DEFAULT 0 CHECK (is_system IS NULL OR is_system IN (0, 1)),
    visual TEXT DEFAULT 'NONE' CHECK (visual IS NULL OR visual IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    auditory TEXT DEFAULT 'NONE' CHECK (auditory IS NULL OR auditory IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    tactile TEXT DEFAULT 'NONE' CHECK (tactile IS NULL OR tactile IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    olfactory TEXT DEFAULT 'NONE' CHECK (olfactory IS NULL OR olfactory IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    gustatory TEXT DEFAULT 'NONE' CHECK (gustatory IS NULL OR gustatory IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    language_abstract TEXT DEFAULT 'LOW' CHECK (language_abstract IS NULL OR language_abstract IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    syntax_sentence TEXT DEFAULT 'VERY_SHORT' CHECK (syntax_sentence IS NULL OR syntax_sentence IN ('VERY_SHORT', 'SHORT', 'MEDIUM', 'LONG')),
    syntax_variation TEXT DEFAULT 'LOW' CHECK (syntax_variation IS NULL OR syntax_variation IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    syntax_structure TEXT DEFAULT 'SIMPLE' CHECK (syntax_structure IS NULL OR syntax_structure IN ('SIMPLE', 'COMPOUND', 'COMPLEX', 'FRAGMENT', 'MIXED'))
);

CREATE INDEX idx_style_presets_category ON style_presets(category);

CREATE INDEX idx_style_presets_is_system ON style_presets(is_system);

-- 15. rhetorical_devices (docs/db.dbml:213)
CREATE TABLE rhetorical_devices (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    description TEXT
);

-- 16. style_preset_rhetorical_devices (docs/db.dbml:227)
CREATE TABLE style_preset_rhetorical_devices (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    preset_id TEXT NOT NULL REFERENCES style_presets(id),
    rhetorical_device_id TEXT NOT NULL REFERENCES rhetorical_devices(id),
    usage_type TEXT CHECK (usage_type IS NULL OR usage_type IN ('primary', 'secondary', 'avoid')),
    custom_instruction TEXT,
    UNIQUE (preset_id, rhetorical_device_id, usage_type)
);

CREATE INDEX idx_style_preset_rhetorical_devices_device_id ON style_preset_rhetorical_devices(rhetorical_device_id);

-- 17. style_preset_vocab_items (docs/db.dbml:241)
CREATE TABLE style_preset_vocab_items (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    preset_id TEXT NOT NULL REFERENCES style_presets(id),
    content TEXT NOT NULL
);

CREATE INDEX idx_style_preset_vocab_items_preset_id ON style_preset_vocab_items(preset_id);

-- 18. style_preset_focus_items (docs/db.dbml:251)
CREATE TABLE style_preset_focus_items (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    preset_id TEXT NOT NULL REFERENCES style_presets(id),
    content TEXT NOT NULL
);

CREATE INDEX idx_style_preset_focus_items_preset_id ON style_preset_focus_items(preset_id);

-- 19. narrator_configs (docs/db.dbml:306)
CREATE TABLE narrator_configs (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT NOT NULL,
    description TEXT,
    person TEXT NOT NULL CHECK (person IN ('FIRST_PERSON', 'SECOND_PERSON', 'THIRD_PERSON')),
    person_subtype TEXT NOT NULL CHECK (person_subtype IN (
        'PROTAGONIST', 'WITNESS_PERIPHERAL', 'UNRELIABLE_FIRST',
        'DIRECT_ADDRESS', 'SELF_REFLECTION', 'INTERACTIVE',
        'OMNISCIENT', 'LIMITED', 'OBJECTIVE'
    )),
    pov_scope TEXT DEFAULT 'SINGLE_POV' CHECK (pov_scope IS NULL OR pov_scope IN ('SINGLE_POV', 'MULTIPLE_POV', 'SHIFTING_OMNISCIENT')),
    is_reliable INTEGER DEFAULT 1 CHECK (is_reliable IS NULL OR is_reliable IN (0, 1)),
    breaks_fourth_wall INTEGER DEFAULT 0 CHECK (breaks_fourth_wall IS NULL OR breaks_fourth_wall IN (0, 1))
);

-- 20. narrative_directives (docs/db.dbml:342)
CREATE TABLE narrative_directives (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    description TEXT,
    category TEXT NOT NULL CHECK (category IN ('PROSE_PRINCIPLE', 'RESTRAINT', 'INFORMATION_CONTROL')),
    is_system INTEGER DEFAULT 0 CHECK (is_system IS NULL OR is_system IN (0, 1))
);

CREATE INDEX idx_narrative_directives_category ON narrative_directives(category);

CREATE INDEX idx_narrative_directives_is_system ON narrative_directives(is_system);

-- 21. style_preset_narratives (docs/db.dbml:330)
CREATE TABLE style_preset_narratives (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    style_preset_id TEXT NOT NULL REFERENCES style_presets(id),
    narrative_directives_id TEXT REFERENCES narrative_directives(id),
    weight TEXT DEFAULT 'NONE' CHECK (weight IS NULL OR weight IN ('NONE', 'LOW', 'MEDIUM', 'HIGH')),
    UNIQUE (style_preset_id, narrative_directives_id)
);

CREATE INDEX idx_style_preset_narratives_directive_id ON style_preset_narratives(narrative_directives_id);

-- 22. projects (docs/db.dbml:384)
CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    summary TEXT,
    cover TEXT,
    status TEXT DEFAULT 'PLANNING' CHECK (status IS NULL OR status IN ('PLANNING', 'DRAFTING', 'ONGOING', 'IN_REVIEW', 'COMPLETED')),
    language TEXT DEFAULT 'VI' CHECK (language IS NULL OR language IN ('VI', 'EN', 'JA', 'ZH_HANT', 'ZH_HANS')),
    created_at TEXT,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE INDEX idx_projects_status ON projects(status);

CREATE INDEX idx_projects_created_at ON projects(created_at);

CREATE INDEX idx_projects_deleted_at ON projects(deleted_at);

-- 23. sessions (docs/db.dbml:403)
CREATE TABLE sessions (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id),
    display_name TEXT,
    created_at TEXT
);

CREATE INDEX idx_sessions_project_id ON sessions(project_id);

CREATE INDEX idx_sessions_created_at ON sessions(created_at);

-- 24. messages (docs/db.dbml:416)
CREATE TABLE messages (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    session_id TEXT REFERENCES sessions(id),
    turn_id TEXT,
    role TEXT,
    content TEXT,
    payload_json TEXT CHECK (payload_json IS NULL OR json_valid(payload_json) = 1),
    created_at TEXT
);

CREATE INDEX idx_messages_session ON messages(session_id);

CREATE INDEX idx_messages_turn_id ON messages(turn_id);

CREATE INDEX idx_messages_created_at ON messages(created_at);

-- 25. project_genres (docs/db.dbml:433)
CREATE TABLE project_genres (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    project_id TEXT REFERENCES projects(id),
    genre_id TEXT REFERENCES genres(id),
    is_primary INTEGER CHECK (is_primary IS NULL OR is_primary IN (0, 1)),
    UNIQUE (project_id, genre_id)
);

CREATE INDEX idx_project_genres_genre_id ON project_genres(genre_id);

CREATE INDEX idx_project_genres_project_primary ON project_genres(project_id, is_primary);

-- 26. arcs (docs/db.dbml:475)
CREATE TABLE arcs (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    project_id TEXT REFERENCES projects(id),
    position INTEGER,
    summary TEXT,
    deleted_at TEXT
);

CREATE INDEX idx_arcs_project_position ON arcs(project_id, position);

CREATE INDEX idx_arcs_deleted_at ON arcs(deleted_at);

-- 27. arc_motifs (docs/db.dbml:446)
CREATE TABLE arc_motifs (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    arc_id TEXT NOT NULL UNIQUE REFERENCES arcs(id),
    motif_id TEXT NOT NULL REFERENCES motifs(id),
    note TEXT
);

CREATE INDEX idx_arc_motifs_motif_id ON arc_motifs(motif_id);

-- 28. chapters (docs/db.dbml:490)
CREATE TABLE chapters (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    arc_id TEXT REFERENCES arcs(id),
    position INTEGER,
    summary TEXT,
    created_at TEXT,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE INDEX idx_chapters_arc_position ON chapters(arc_id, position);

CREATE INDEX idx_chapters_story_order ON chapters(position);

CREATE INDEX idx_chapters_deleted_at ON chapters(deleted_at);

-- 29. arc_event_blocks (docs/db.dbml:458)
CREATE TABLE arc_event_blocks (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    arc_motif_id TEXT NOT NULL REFERENCES arc_motifs(id),
    event_block_id TEXT NOT NULL REFERENCES event_blocks(id),
    start_chapter_id TEXT NOT NULL REFERENCES chapters(id),
    end_chapter_id TEXT NOT NULL REFERENCES chapters(id),
    summary TEXT,
    UNIQUE (arc_motif_id, event_block_id)
);

CREATE INDEX idx_arc_event_blocks_event_block_id ON arc_event_blocks(event_block_id);

CREATE INDEX idx_arc_event_blocks_start_chapter_id ON arc_event_blocks(start_chapter_id);

CREATE INDEX idx_arc_event_blocks_end_chapter_id ON arc_event_blocks(end_chapter_id);

-- 30. world_settings (docs/db.dbml:889)
CREATE TABLE world_settings (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE NOT NULL,
    project_id TEXT REFERENCES projects(id),
    display_name TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('REALM', 'WORLD', 'REGION', 'COUNTRY', 'CITY', 'TOWN', 'FACILITY', 'STREET', 'ROOM')),
    parent_id TEXT REFERENCES world_settings(id),
    description TEXT,
    access TEXT DEFAULT 'PUBLIC' CHECK (access IS NULL OR access IN ('PUBLIC', 'PRIVATE', 'SECRET', 'LOCKED', 'GUARDED', 'RESTRICTED')),
    spatial_features TEXT,
    sensory_visual TEXT,
    sensory_auditory TEXT,
    sensory_tactile TEXT,
    sensory_olfactory TEXT,
    sensory_gustatory TEXT,
    created_at TEXT,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE INDEX idx_world_settings_project_id ON world_settings(project_id);

CREATE INDEX idx_world_settings_type ON world_settings(type);

CREATE INDEX idx_world_settings_parent_id ON world_settings(parent_id);

CREATE INDEX idx_world_settings_parent_type ON world_settings(parent_id, type);

CREATE INDEX idx_world_settings_project_parent ON world_settings(project_id, parent_id);

CREATE INDEX idx_world_settings_deleted_at ON world_settings(deleted_at);

-- 31. lore_entries (docs/db.dbml:933)
CREATE TABLE lore_entries (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('CULTURE', 'HISTORY', 'FACTION', 'ITEM')),
    display_name TEXT NOT NULL,
    description TEXT,
    world_setting_id TEXT REFERENCES world_settings(id),
    data TEXT CHECK (data IS NULL OR json_valid(data) = 1),
    deleted_at TEXT
);

CREATE INDEX idx_lore_entries_type ON lore_entries(type);

CREATE INDEX idx_lore_entries_world_setting_id ON lore_entries(world_setting_id);

CREATE INDEX idx_lore_entries_deleted_at ON lore_entries(deleted_at);

-- 32. scenes (docs/db.dbml:548)
CREATE TABLE scenes (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    chapter_id TEXT REFERENCES chapters(id),
    world_setting_id TEXT NOT NULL REFERENCES world_settings(id),
    style_preset_id TEXT REFERENCES style_presets(id),
    narrator_config_id TEXT REFERENCES narrator_configs(id),
    summary TEXT,
    content TEXT,
    word_count INTEGER,
    position INTEGER,
    is_locked INTEGER DEFAULT 0 CHECK (is_locked IS NULL OR is_locked IN (0, 1)),
    deleted_at TEXT
);

CREATE INDEX idx_scenes_chapter_position ON scenes(chapter_id, position);

CREATE INDEX idx_scenes_style_preset_id ON scenes(style_preset_id);

CREATE INDEX idx_scenes_narrator_config_id ON scenes(narrator_config_id);

CREATE INDEX idx_scenes_world_setting_id ON scenes(world_setting_id);

CREATE INDEX idx_scenes_deleted_at ON scenes(deleted_at);

-- 33. scene_lore_entries (docs/db.dbml:572)
CREATE TABLE scene_lore_entries (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    scene_id TEXT NOT NULL REFERENCES scenes(id),
    lore_entry_id TEXT NOT NULL REFERENCES lore_entries(id),
    usage TEXT DEFAULT 'REFERENCE' CHECK (usage IS NULL OR usage IN (
        'SETTING', 'REFERENCE', 'FORESHADOW', 'PAYOFF', 'REVEAL', 'CONFLICT', 'CONSTRAINT', 'RED_HERRING'
    )),
    reveal_to_reader TEXT DEFAULT 'PARTIAL' CHECK (reveal_to_reader IS NULL OR reveal_to_reader IN ('NONE', 'HINT', 'PARTIAL', 'FULL')),
    reveal_to_pov TEXT DEFAULT 'NONE' CHECK (reveal_to_pov IS NULL OR reveal_to_pov IN ('NONE', 'HINT', 'PARTIAL', 'FULL')),
    UNIQUE (scene_id, lore_entry_id)
);

CREATE INDEX idx_scene_lore_entries_lore_entry_id ON scene_lore_entries(lore_entry_id);

CREATE INDEX idx_scene_lore_entries_usage ON scene_lore_entries(usage);

-- 34. characters (docs/db.dbml:660)
CREATE TABLE characters (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT,
    nickname TEXT,
    birth TEXT,
    species TEXT,
    biological_sex TEXT CHECK (biological_sex IS NULL OR biological_sex IN ('NONE', 'MALE', 'FEMALE')),
    gender_identity TEXT CHECK (gender_identity IS NULL OR gender_identity IN ('NONE', 'MALE', 'FEMALE')),
    gender_expression TEXT CHECK (gender_expression IS NULL OR gender_expression IN ('NONE', 'MALE', 'FEMALE')),
    deleted_at TEXT,
    CHECK (display_name IS NOT NULL OR nickname IS NOT NULL)
);

CREATE INDEX idx_characters_display_name ON characters(display_name);

CREATE INDEX idx_characters_nickname ON characters(nickname);

CREATE INDEX idx_characters_species ON characters(species);

CREATE INDEX idx_characters_deleted_at ON characters(deleted_at);

-- 35. character_motifs (docs/db.dbml:355)
CREATE TABLE character_motifs (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    motif_id TEXT REFERENCES motifs(id),
    character_id TEXT REFERENCES characters(id),
    description TEXT,
    UNIQUE (character_id, motif_id)
);

CREATE INDEX idx_character_motifs_motif_id ON character_motifs(motif_id);

-- 36. scene_characters (docs/db.dbml:590)
CREATE TABLE scene_characters (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    character_id TEXT REFERENCES characters(id),
    scene_id TEXT REFERENCES scenes(id),
    is_pov INTEGER DEFAULT 0 CHECK (is_pov IS NULL OR is_pov IN (0, 1)),
    UNIQUE (scene_id, character_id)
);

CREATE INDEX idx_scene_characters_character_id ON scene_characters(character_id);

-- 37. scene_versions (docs/db.dbml:602, SF-07)
CREATE TABLE scene_versions (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    parent_id TEXT NOT NULL REFERENCES scenes(id),
    version_number INTEGER,
    scene_source TEXT CHECK (scene_source IS NULL OR scene_source IN ('AI_GENERATED', 'HUMAN_EDITED', 'RESTORE')),
    summary TEXT,
    content TEXT,
    word_count INTEGER,
    created_at TEXT,
    UNIQUE (parent_id, version_number)
);

CREATE INDEX idx_scene_versions_parent_id ON scene_versions(parent_id);

CREATE INDEX idx_scene_versions_created_at ON scene_versions(created_at);

-- 38. chapter_save_receipts (docs/db.dbml:618, SF-02)
CREATE TABLE chapter_save_receipts (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    chapter_id TEXT NOT NULL REFERENCES chapters(id),
    client_save_id TEXT NOT NULL,
    payload_hash TEXT NOT NULL,
    response_json TEXT NOT NULL CHECK (json_valid(response_json) = 1),
    created_at TEXT,
    UNIQUE (chapter_id, client_save_id)
);

CREATE INDEX idx_chapter_save_receipts_chapter_id ON chapter_save_receipts(chapter_id);

-- 39. project_characters (docs/db.dbml:682)
CREATE TABLE project_characters (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    project_id TEXT NOT NULL REFERENCES projects(id),
    character_id TEXT NOT NULL REFERENCES characters(id),
    role TEXT,
    notes TEXT,
    created_at TEXT,
    deleted_at TEXT,
    UNIQUE (project_id, character_id)
);

CREATE INDEX idx_project_characters_project_id ON project_characters(project_id);

CREATE INDEX idx_project_characters_character_id ON project_characters(character_id);

CREATE INDEX idx_project_characters_deleted_at ON project_characters(deleted_at);

-- 40. appearance_option_groups (docs/db.dbml:701)
CREATE TABLE appearance_option_groups (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT NOT NULL
);

-- 41. appearance_options (docs/db.dbml:707)
CREATE TABLE appearance_options (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    group_id TEXT NOT NULL REFERENCES appearance_option_groups(id),
    code TEXT NOT NULL,
    display_name TEXT NOT NULL,
    hex TEXT,
    is_system INTEGER DEFAULT 0 CHECK (is_system IS NULL OR is_system IN (0, 1)),
    UNIQUE (group_id, code)
);

-- 42. character_appearances (docs/db.dbml:720)
CREATE TABLE character_appearances (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    character_id TEXT NOT NULL UNIQUE REFERENCES characters(id),
    height INTEGER,
    height_unit TEXT DEFAULT 'CM' CHECK (height_unit IS NULL OR height_unit IN (
        'MM', 'CM', 'M', 'HA', 'KM', 'IN', 'FT', 'ML', 'L', 'MG', 'G', 'KG', 'LBS'
    )),
    weight INTEGER,
    weight_unit TEXT DEFAULT 'KG' CHECK (weight_unit IS NULL OR weight_unit IN (
        'MM', 'CM', 'M', 'HA', 'KM', 'IN', 'FT', 'ML', 'L', 'MG', 'G', 'KG', 'LBS'
    )),
    hair_color_id TEXT REFERENCES appearance_options(id),
    hair_length_id TEXT REFERENCES appearance_options(id),
    hair_style_id TEXT REFERENCES appearance_options(id),
    hair_texture_id TEXT REFERENCES appearance_options(id),
    pupil_shape_id TEXT REFERENCES appearance_options(id),
    iris_color_id TEXT REFERENCES appearance_options(id),
    sclera_color_id TEXT REFERENCES appearance_options(id),
    skin_tone_id TEXT REFERENCES appearance_options(id),
    skin_texture_id TEXT REFERENCES appearance_options(id),
    hair_note TEXT,
    eyes_note TEXT,
    skin_note TEXT,
    outfit_style TEXT,
    default_summary TEXT
);

-- 43. character_psychological (docs/db.dbml:747)
CREATE TABLE character_psychological (
    character_id TEXT PRIMARY KEY NOT NULL REFERENCES characters(id) CHECK (length(character_id) = 36),
    the_wound TEXT,
    the_lie TEXT,
    fundamental_fear TEXT,
    moral_alignment TEXT,
    moral_code TEXT,
    cognitive_bias TEXT,
    breaking_point TEXT,
    external_want TEXT,
    internal_need TEXT,
    internal_conflict TEXT,
    persona_type TEXT,
    verbal_style TEXT,
    primary_reaction TEXT,
    stress_trigger TEXT,
    coping_strategy TEXT
);

-- 44. character_distinguishing_marks (docs/db.dbml:770)
CREATE TABLE character_distinguishing_marks (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    character_id TEXT NOT NULL REFERENCES characters(id),
    location TEXT,
    content TEXT NOT NULL
);

CREATE INDEX idx_character_distinguishing_marks_character_id ON character_distinguishing_marks(character_id);

-- 45. relationships (docs/db.dbml:795)
CREATE TABLE relationships (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    motif_id TEXT REFERENCES motifs(id),
    from_character_id TEXT REFERENCES characters(id),
    to_character_id TEXT REFERENCES characters(id),
    UNIQUE (from_character_id, to_character_id)
);

CREATE INDEX idx_relationships_to_character_id ON relationships(to_character_id);

-- 46. relationship_categories (docs/db.dbml:815)
CREATE TABLE relationship_categories (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    category TEXT CHECK (category IS NULL OR category IN ('FAMILY', 'ROMANTIC', 'SOCIAL', 'PROFESSIONAL', 'ANTAGONISTIC')),
    sub_type TEXT,
    UNIQUE (category, sub_type)
);

CREATE INDEX idx_relationship_categories_category ON relationship_categories(category);

-- 47. relationship_roles (docs/db.dbml:827)
CREATE TABLE relationship_roles (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    hierarchy TEXT DEFAULT 'EQUAL' CHECK (hierarchy IS NULL OR hierarchy IN ('SUPERIOR', 'EQUAL', 'INFERIOR')),
    category_id TEXT REFERENCES relationship_categories(id),
    relationship_id TEXT REFERENCES relationships(id),
    is_primary INTEGER CHECK (is_primary IS NULL OR is_primary IN (0, 1))
);

CREATE INDEX idx_relationship_roles_relationship_id ON relationship_roles(relationship_id);

CREATE INDEX idx_relationship_roles_category_id ON relationship_roles(category_id);

CREATE INDEX idx_relationship_roles_rel_primary ON relationship_roles(relationship_id, is_primary);

-- 48. relationship_states (docs/db.dbml:841)
CREATE TABLE relationship_states (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    relationship_id TEXT REFERENCES relationships(id),
    scene_id TEXT REFERENCES scenes(id),
    trust TEXT CHECK (trust IS NULL OR trust IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE')),
    affection TEXT CHECK (affection IS NULL OR affection IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE')),
    hostility TEXT CHECK (hostility IS NULL OR hostility IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE')),
    fear TEXT CHECK (fear IS NULL OR fear IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE')),
    respect TEXT CHECK (respect IS NULL OR respect IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE')),
    familiarity TEXT CHECK (familiarity IS NULL OR familiarity IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE')),
    dependency TEXT CHECK (dependency IS NULL OR dependency IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE')),
    created_at TEXT
);

CREATE INDEX idx_relationship_states_rel_created ON relationship_states(relationship_id, created_at);

-- 49. themes (docs/db.dbml:989, CI-07)
CREATE TABLE themes (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    arc_id TEXT REFERENCES arcs(id),
    parent_theme_id TEXT REFERENCES themes(id),
    question TEXT NOT NULL,
    resolution_type TEXT DEFAULT 'OPEN' CHECK (resolution_type IS NULL OR resolution_type IN ('RESOLVED', 'OPEN', 'AMBIGUOUS')),
    resolution TEXT,
    message TEXT,
    created_at TEXT,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE INDEX idx_themes_arc_id ON themes(arc_id);

CREATE INDEX idx_themes_parent_theme_id ON themes(parent_theme_id);

CREATE INDEX idx_themes_deleted_at ON themes(deleted_at);

-- 50. theme_arguments (docs/db.dbml:1014)
CREATE TABLE theme_arguments (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    theme_id TEXT NOT NULL REFERENCES themes(id),
    character_id TEXT NOT NULL REFERENCES characters(id),
    role TEXT NOT NULL CHECK (role IN ('PRIMARY', 'ANTITHESIS', 'CAUTIONARY', 'WITNESS')),
    answer_mode TEXT CHECK (answer_mode IS NULL OR answer_mode IN ('VERBAL', 'ACTION', 'OUTCOME', 'IMPLICIT')),
    note TEXT,
    required_knowledge TEXT,
    required_competence TEXT,
    UNIQUE (theme_id, character_id)
);

CREATE INDEX idx_theme_arguments_character_id ON theme_arguments(character_id);

CREATE INDEX idx_theme_arguments_theme_role ON theme_arguments(theme_id, role);

-- 51. character_memories (docs/db.dbml:1039)
CREATE TABLE character_memories (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    character_id TEXT NOT NULL REFERENCES characters(id),
    source_scene_id TEXT REFERENCES scenes(id),
    content TEXT NOT NULL,
    created_at TEXT
);

CREATE INDEX idx_character_memories_source_scene_id ON character_memories(source_scene_id);

-- 52. competence_domains (docs/db.dbml:1058)
CREATE TABLE competence_domains (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    is_system INTEGER DEFAULT 0 CHECK (is_system IS NULL OR is_system IN (0, 1))
);

-- 53. domain_level (docs/db.dbml:1070)
CREATE TABLE domain_level (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    competence_domain_id TEXT NOT NULL REFERENCES competence_domains(id),
    code TEXT NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    UNIQUE (competence_domain_id, code)
);

CREATE INDEX idx_domain_level_competence_domain_id ON domain_level(competence_domain_id);

-- 54. character_competences (docs/db.dbml:1087)
CREATE TABLE character_competences (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    character_id TEXT NOT NULL REFERENCES characters(id),
    domain_id TEXT NOT NULL REFERENCES competence_domains(id),
    level_id TEXT REFERENCES domain_level(id),
    scene_id TEXT NOT NULL REFERENCES scenes(id),
    display_name TEXT NOT NULL,
    description TEXT,
    reliability TEXT,
    experience TEXT
);

CREATE INDEX idx_character_competences_character_id ON character_competences(character_id);

CREATE INDEX idx_character_competences_domain_id ON character_competences(domain_id);

CREATE INDEX idx_character_competences_level_id ON character_competences(level_id);

CREATE INDEX idx_character_competences_char_domain ON character_competences(character_id, domain_id);

-- 55. competence_capabilities (docs/db.dbml:1116)
CREATE TABLE competence_capabilities (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    competence_id TEXT NOT NULL REFERENCES character_competences(id),
    content TEXT NOT NULL
);

CREATE INDEX idx_competence_capabilities_competence_id ON competence_capabilities(competence_id);

-- 56. competence_limitations (docs/db.dbml:1130)
CREATE TABLE competence_limitations (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    competence_id TEXT NOT NULL REFERENCES character_competences(id),
    content TEXT NOT NULL
);

CREATE INDEX idx_competence_limitations_competence_id ON competence_limitations(competence_id);

-- 57. symbolics (docs/db.dbml:1157)
CREATE TABLE symbolics (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE,
    display_name TEXT NOT NULL,
    description TEXT,
    is_system INTEGER DEFAULT 0 CHECK (is_system IS NULL OR is_system IN (0, 1))
);

CREATE INDEX idx_symbolics_is_system ON symbolics(is_system);

-- 58. symbolic_motifs (docs/db.dbml:1146)
CREATE TABLE symbolic_motifs (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    symbolic_id TEXT REFERENCES symbolics(id),
    motif_id TEXT REFERENCES motifs(id),
    UNIQUE (symbolic_id, motif_id)
);

CREATE INDEX idx_symbolic_motifs_motif_id ON symbolic_motifs(motif_id);

-- 59. prompt_blocks (docs/db.dbml:1180)
CREATE TABLE prompt_blocks (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    prompt_code TEXT UNIQUE NOT NULL,
    display_name TEXT,
    heading_level INTEGER NOT NULL CHECK (heading_level BETWEEN 1 AND 6),
    content_format TEXT NOT NULL DEFAULT 'MARKDOWN' CHECK (content_format IN ('JSON', 'MARKDOWN')),
    content TEXT NOT NULL
);

CREATE INDEX idx_prompt_blocks_prompt_code ON prompt_blocks(prompt_code);

-- 60. tools (docs/db.dbml:1196)
CREATE TABLE tools (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    code TEXT UNIQUE NOT NULL,
    display_name TEXT,
    description TEXT,
    input_schema TEXT CHECK (input_schema IS NULL OR json_valid(input_schema) = 1)
);

CREATE INDEX idx_tools_code ON tools(code);

-- 61. prompt_block_tools (docs/db.dbml:1210)
CREATE TABLE prompt_block_tools (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    prompt_block_id TEXT NOT NULL REFERENCES prompt_blocks(id),
    tool_id TEXT NOT NULL REFERENCES tools(id),
    UNIQUE (prompt_block_id, tool_id)
);

CREATE INDEX idx_prompt_block_tools_tool_id ON prompt_block_tools(tool_id);

-- 62. draft_sessions (docs/db.dbml:1256)
CREATE TABLE draft_sessions (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    session_id TEXT NOT NULL UNIQUE REFERENCES sessions(id),
    project_id TEXT REFERENCES projects(id),
    head_version_seq INTEGER,
    status TEXT DEFAULT 'editing' CHECK (status IS NULL OR status IN ('editing', 'committing', 'committed')),
    created_at TEXT,
    updated_at TEXT
);

CREATE INDEX idx_draft_sessions_project_id ON draft_sessions(project_id);

CREATE INDEX idx_draft_sessions_status ON draft_sessions(status);

-- 63. draft_versions (docs/db.dbml:1273, SF-01)
CREATE TABLE draft_versions (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    seq INTEGER NOT NULL,
    seal_key TEXT,
    phase INTEGER,
    subphase INTEGER,
    data_json TEXT NOT NULL CHECK (json_valid(data_json) = 1),
    data_hash TEXT NOT NULL,
    reason TEXT NOT NULL CHECK (reason IN ('ai_patch', 'manual_edit', 'phase_confirm', 'regenerate', 'restore', 'workflow_round')),
    parent_seq INTEGER,
    turn_id TEXT,
    chat_message_id TEXT REFERENCES messages(id),
    created_at TEXT,
    UNIQUE (draft_session_id, seq),
    UNIQUE (draft_session_id, seal_key),
    CHECK ((phase = 5 AND subphase IN (1, 2, 3)) OR (phase <> 5 AND subphase IS NULL))
);

CREATE INDEX idx_draft_versions_draft_session_id ON draft_versions(draft_session_id);

CREATE INDEX idx_draft_versions_turn_id ON draft_versions(turn_id);

CREATE INDEX idx_draft_versions_chat_message_id ON draft_versions(chat_message_id);

-- 64. workflow_task_receipts (docs/db.dbml:1326 - Pre-B+ 11 legacy columns)
CREATE TABLE workflow_task_receipts (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    workflow_run_id TEXT NOT NULL,
    plan_generation INTEGER NOT NULL,
    task_identity TEXT NOT NULL,
    input_scope_fingerprint TEXT NOT NULL,
    output_scope_fingerprint TEXT NOT NULL,
    base_version_seq INTEGER NOT NULL,
    resulting_version_seq INTEGER,
    had_mutations INTEGER NOT NULL DEFAULT 0 CHECK (had_mutations IN (0, 1)),
    created_at TEXT,
    UNIQUE (draft_session_id, workflow_run_id, plan_generation, task_identity),
    CHECK ((had_mutations = 1 AND resulting_version_seq IS NOT NULL) OR (had_mutations = 0 AND resulting_version_seq IS NULL))
);

CREATE INDEX idx_workflow_task_receipts_session_gen_task ON workflow_task_receipts(draft_session_id, plan_generation, task_identity);

CREATE INDEX idx_workflow_task_receipts_draft_session_id ON workflow_task_receipts(draft_session_id);

-- 65. pending_patches (docs/db.dbml:1357 - Pre-B+ 13 legacy columns)
CREATE TABLE pending_patches (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    base_version_seq INTEGER,
    target_type TEXT NOT NULL,
    target_code TEXT NOT NULL,
    field_name TEXT,
    operation TEXT NOT NULL CHECK (operation IN ('set', 'delete')),
    previous_value TEXT CHECK (previous_value IS NULL OR json_valid(previous_value) = 1),
    new_value TEXT CHECK (new_value IS NULL OR json_valid(new_value) = 1),
    target_source TEXT CHECK (target_source IS NULL OR target_source IN ('user', 'user_edited', 'ai_generated')),
    status TEXT DEFAULT 'pending' CHECK (status IS NULL OR status IN ('pending', 'confirmed', 'rejected', 'expired')),
    created_at TEXT,
    resolved_at TEXT
);

CREATE INDEX idx_pending_patches_session_status ON pending_patches(draft_session_id, status);


-- 66. workflow_round_receipts (docs/db.dbml:1300-1325)
CREATE TABLE workflow_round_receipts (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    workflow_run_id TEXT NOT NULL,
    plan_generation INTEGER NOT NULL CHECK (plan_generation > 0),
    round_index INTEGER NOT NULL CHECK (round_index >= 0),
    round_seal_key TEXT NOT NULL CHECK (length(round_seal_key) > 0),
    task_count INTEGER NOT NULL CHECK (task_count >= 0),
    parallel_limit INTEGER NOT NULL CHECK (parallel_limit >= 1),
    plan_digest TEXT NOT NULL CHECK (length(plan_digest) > 0),
    request_digest TEXT NOT NULL CHECK (length(request_digest) > 0),
    base_version_seq INTEGER NOT NULL,
    base_data_hash TEXT NOT NULL CHECK (length(base_data_hash) > 0),
    final_data_hash TEXT NOT NULL CHECK (length(final_data_hash) > 0),
    resulting_version_seq INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (draft_session_id, base_version_seq) REFERENCES draft_versions(draft_session_id, seq),
    FOREIGN KEY (draft_session_id, resulting_version_seq) REFERENCES draft_versions(draft_session_id, seq),
    UNIQUE (draft_session_id, workflow_run_id, plan_generation, round_index),
    UNIQUE (draft_session_id, round_seal_key),
    UNIQUE (id, draft_session_id, workflow_run_id, plan_generation, base_version_seq),
    CHECK ((resulting_version_seq IS NULL AND base_data_hash = final_data_hash) OR (resulting_version_seq IS NOT NULL AND base_data_hash <> final_data_hash))
);

-- Rebuild workflow_task_receipts to add SF-06 B+ columns and constraints (docs/db.dbml:1326-1350)
CREATE TABLE workflow_task_receipts_new (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    workflow_run_id TEXT NOT NULL,
    plan_generation INTEGER NOT NULL,
    task_identity TEXT NOT NULL,
    input_scope_fingerprint TEXT NOT NULL,
    output_scope_fingerprint TEXT NOT NULL,
    base_version_seq INTEGER NOT NULL,
    resulting_version_seq INTEGER,
    had_mutations INTEGER NOT NULL DEFAULT 0 CHECK (had_mutations IN (0, 1)),
    round_receipt_id TEXT REFERENCES workflow_round_receipts(id),
    task_index INTEGER,
    evidence_json TEXT CHECK (evidence_json IS NULL OR json_valid(evidence_json) = 1),
    created_at TEXT,
    FOREIGN KEY (round_receipt_id, draft_session_id, workflow_run_id, plan_generation, base_version_seq)
        REFERENCES workflow_round_receipts(id, draft_session_id, workflow_run_id, plan_generation, base_version_seq),
    UNIQUE (draft_session_id, workflow_run_id, plan_generation, task_identity),
    UNIQUE (round_receipt_id, task_index),
    CHECK ((had_mutations = 1 AND resulting_version_seq IS NOT NULL) OR (had_mutations = 0 AND resulting_version_seq IS NULL)),
    CHECK (
        (round_receipt_id IS NULL AND task_index IS NULL AND evidence_json IS NULL) OR
        (round_receipt_id IS NOT NULL AND task_index IS NOT NULL AND task_index >= 0 AND evidence_json IS NOT NULL AND json_valid(evidence_json) = 1)
    )
);

DROP TABLE workflow_task_receipts;

ALTER TABLE workflow_task_receipts_new RENAME TO workflow_task_receipts;

CREATE INDEX idx_workflow_task_receipts_session_gen_task ON workflow_task_receipts(draft_session_id, plan_generation, task_identity);

CREATE INDEX idx_workflow_task_receipts_draft_session_id ON workflow_task_receipts(draft_session_id);

-- Rebuild pending_patches to add eligibility_json (docs/db.dbml:1357-1372)
CREATE TABLE pending_patches_new (
    id TEXT PRIMARY KEY NOT NULL CHECK (length(id) = 36),
    draft_session_id TEXT NOT NULL REFERENCES draft_sessions(id),
    base_version_seq INTEGER,
    target_type TEXT NOT NULL,
    target_code TEXT NOT NULL,
    field_name TEXT,
    operation TEXT NOT NULL CHECK (operation IN ('set', 'delete')),
    previous_value TEXT CHECK (previous_value IS NULL OR json_valid(previous_value) = 1),
    new_value TEXT CHECK (new_value IS NULL OR json_valid(new_value) = 1),
    target_source TEXT CHECK (target_source IS NULL OR target_source IN ('user', 'user_edited', 'ai_generated')),
    eligibility_json TEXT CHECK (eligibility_json IS NULL OR json_valid(eligibility_json) = 1),
    status TEXT DEFAULT 'pending' CHECK (status IS NULL OR status IN ('pending', 'confirmed', 'rejected', 'expired')),
    created_at TEXT,
    resolved_at TEXT
);

DROP TABLE pending_patches;

ALTER TABLE pending_patches_new RENAME TO pending_patches;

CREATE INDEX idx_pending_patches_session_status ON pending_patches(draft_session_id, status);

-- Add durable soft-delete state without changing or removing existing sessions.
ALTER TABLE sessions ADD COLUMN deleted_at TEXT;

CREATE INDEX idx_sessions_deleted_at ON sessions(deleted_at);

PRAGMA user_version = 1;
