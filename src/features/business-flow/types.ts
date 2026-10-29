export interface BusinessFlowMeta {
  version: string;
  phase: number;
  updated_at: string;
}

export interface RequireProjectTarget {
  arc_count: number;
  chapter_count: number;
  word_per_chapter_count: number;
}

export interface RequireGenre {
  code: string;
  is_primary: boolean;
  display_name?: string;
}

export interface RequireProject {
  display_name: string;
  summary: string;
  language: string;
  target: RequireProjectTarget;
  genres: RequireGenre[];
}

export type BiologicalSex = 'NONE' | 'MALE' | 'FEMALE';

export interface RequireCharacter {
  code: string;
  display_name: string;
  biological_sex: BiologicalSex;
  motif_code: string;
}

export interface RequireSection {
  project: RequireProject;
  character: RequireCharacter[];
}

export interface OptionalOverviewProjectTarget {
  arc_tolerance?: number;
  chapter_tolerance?: number;
  word_per_chapter_tolerance?: number;
}

export interface OptionalOverviewProject {
  target?: OptionalOverviewProjectTarget;
}

export interface EventBlockItem {
  code: string;
  start_chapter: number;
  end_chapter: number;
  summary: string;
}

export interface ArcMotif {
  code: string;
  note?: string;
  event_blocks?: EventBlockItem[];
}

export interface ThemeArgumentItem {
  code?: string;
  character_display_name: string;
  role: string;
  answer_mode?: string;
  note?: string;
  require?: {
    knowledge?: string;
    competence?: string;
  };
}

export interface ArcThemes {
  code?: string;
  parent_theme_code?: string;
  question: string;
  resolution?: {
    type?: string;
    content?: string;
  };
  message?: string;
  arguments?: ThemeArgumentItem[];
}

export interface OverviewPhaseArc {
  display_name: string;
  summary: string;
  position: number;
  motif?: ArcMotif;
  themes?: ArcThemes;
}

export interface OptionalOverviewPhase {
  project?: OptionalOverviewProject;
  arcs?: OverviewPhaseArc[] | any[];
}

export interface ChapterPhaseScene {
  code: string;
  display_name: string;
  position: number;
  summary: string;
}

export interface ChapterPhaseChapter {
  display_name: string;
  position: number;
  summary: string;
  scenes?: ChapterPhaseScene[];
}

export interface ChapterPhaseArc {
  chapters?: ChapterPhaseChapter[];
}

export interface OptionalChapterPhase {
  arcs?: ChapterPhaseArc[] | any[];
}

export interface LoreEntryItem {
  code: string;
  display_name: string;
  type: string;
  description: string;
  data?: string;
}

export interface WorldSettingsItem {
  code: string;
  display_name: string;
  type: string;
  parent_code?: string;
  description: string;
  access: string;
  lore_entries?: LoreEntryItem[];
}

export interface OptionalWorldBuildingPhase {
  world_settings?: WorldSettingsItem | any;
}

export interface SceneConfigCharacterItem {
  display_name: string;
  is_pov: boolean;
}

export interface SceneConfigSceneItem {
  style_preset_code?: string;
  narrator_config_code?: string;
  lore_entries_code?: string;
  characters?: SceneConfigCharacterItem[];
}

export interface SceneConfigChapterItem {
  scenes?: SceneConfigSceneItem[];
}

export interface SceneConfigArc {
  chapters?: SceneConfigChapterItem[];
}

export interface OptionalSceneConfigPhase {
  arcs?: SceneConfigArc[] | any[];
}

export interface CharacterAppearanceGender {
  identity?: string;
  expresion?: string;
}

export interface CharacterAppearanceHair {
  color?: string;
  style?: string;
  texture?: string;
  length?: string;
}

export interface CharacterAppearanceSkin {
  tone?: string;
  texture?: string;
}

export interface CharacterAppearanceEyes {
  pupil?: {
    shape?: string;
    color?: string;
  };
  iris?: {
    texture?: any;
    color?: string;
  };
  sclera?: string;
}

export interface CharacterAppearanceItem {
  code: string;
  species?: string;
  birth?: string;
  gender?: CharacterAppearanceGender;
  height_cm?: number;
  weight_kg?: number;
  hair?: CharacterAppearanceHair;
  skin?: CharacterAppearanceSkin;
  eyes?: CharacterAppearanceEyes;
  outfit_style?: string;
  distinguishing_marks?: string[];
  summary?: string;
}

export interface OptionalCharacterAppearancePhase {
  character?: CharacterAppearanceItem[] | any[];
}

export interface CharacterCoreTraumaAndBelief {
  the_wound?: string;
  the_lie?: string;
  fundamental_fear?: string;
}

export interface CharacterCognitiveMoralFramework {
  moral_alignment?: string;
  moral_code?: string;
  cognitive_bias?: string;
  breaking_point?: string;
}

export interface CharacterNarrativeEngine {
  external_want?: string;
  internal_need?: string;
  internal_conflict?: string;
}

export interface CharacterSocialMask {
  persona_type?: string;
  verbal_style?: string;
}

export interface CharacterDefenseMechanisms {
  primary_reaction?: string;
  stress_trigger?: string;
  coping_strategy?: string;
}

export interface CharacterBehavioralAdaptation {
  social_mask?: CharacterSocialMask;
  defense_mechanisms?: CharacterDefenseMechanisms;
}

export interface CharacterPsychologicalItem {
  code: string;
  core_trauma_and_belief?: CharacterCoreTraumaAndBelief;
  cognitive_moral_framework?: CharacterCognitiveMoralFramework;
  narrative_engine?: CharacterNarrativeEngine;
  behavioral_adaptation?: CharacterBehavioralAdaptation;
}

export interface OptionalCharacterPsychologicalPhase {
  character?: CharacterPsychologicalItem[] | any[];
}

export interface CharacterMemoryItem {
  scene_code?: string;
  content?: string;
}

export interface CompetenceDomainLevel {
  code?: string;
  display_name?: string;
  description?: string;
}

export interface CompetenceDomain {
  code?: string;
  display_name?: string;
  description?: string;
  level?: CompetenceDomainLevel;
}

export interface CharacterCompetenceItem {
  display_name?: string;
  description?: string;
  domain?: CompetenceDomain;
  capabilities?: string[];
  limitations?: string[];
}

export interface CharacterLifeExperienceItem {
  code: string;
  memories?: CharacterMemoryItem[];
  competences?: CharacterCompetenceItem[];
}

export interface OptionalCharacterLifeExperiencePhase {
  character?: CharacterLifeExperienceItem[] | any[];
}

export interface RelationshipMotifEventBlock {
  code?: string;
  component?: {
    code?: string;
    display_name?: string;
  };
  state?: {
    code?: string;
    display_name?: string;
  };
  transition?: {
    display_name?: string;
  };
}

export interface RelationshipSceneItem {
  code?: string;
  trust?: string;
  affection?: string;
  hostility?: string;
  fear?: string;
  respect?: string;
  familarity?: any;
  dependency?: string;
}

export interface RelationshipRoleItem {
  hiarachy?: string;
  category?: string;
  sub_type?: string;
  is_primmary?: boolean;
}

export interface RelationshipPhaseCharacterItem {
  from_code?: string;
  form_code?: string;
  to_code?: string;
  scenes?: RelationshipSceneItem[];
  roles?: RelationshipRoleItem[];
}

export interface RelationshipPhaseItem {
  motif?: {
    event_blocks?: RelationshipMotifEventBlock[];
  };
  character?: RelationshipPhaseCharacterItem;
}

export interface OptionalRelationshipPhase {
  relationship?: RelationshipPhaseItem[] | any[];
}

export interface OptionalSection {
  overview_phase?: OptionalOverviewPhase;
  chapter_phase?: OptionalChapterPhase;
  world_building_phase?: OptionalWorldBuildingPhase;
  scene_config_phase?: OptionalSceneConfigPhase;
  character_appearance_phase?: OptionalCharacterAppearancePhase;
  charactar_psychological_phase?: OptionalCharacterPsychologicalPhase;
  character_life_experience_phase?: OptionalCharacterLifeExperiencePhase;
  relationship_phase?: OptionalRelationshipPhase;
}

export interface BusinessFlowData {
  meta: BusinessFlowMeta;
  require: RequireSection;
  optional: OptionalSection;
}
