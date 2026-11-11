export type DraftStep =
  | 'overview'
  | 'chapter'
  | 'world'
  | 'scene'
  | 'character'
  | 'relationship';

export type CharacterSubStep = 'appearance' | 'psychological' | 'lifeExperience';

export interface FeedbackContext {
  id: string;
  tag: string;
  title: string;
  content?: string;
}

export interface DraftGenre {
  name: string;
  isPrimary: boolean;
}

export interface DraftSegment {
  id: string;
  chapterRange: string;
  phaseNumber: number;
  summary: string;
}

export type DraftThemeArgumentRole = 'PRIMARY' | 'ANTITHESIS' | 'CAUTIONARY' | 'WITNESS';
export type DraftThemeReactionType = 'VERBAL' | 'ACTION' | 'OUTCOME' | 'IMPLICIT';

export interface DraftThemeArgument {
  id: string;
  characterName: string;
  role: 'PRIMARY' | 'ANTITHESIS' | 'CAUTIONARY' | 'WITNESS';
  reactionType: 'VERBAL' | 'ACTION' | 'OUTCOME' | 'IMPLICIT';
  note?: string;
  requiredKnowledge?: string;
  requiredCompetence?: string;
}

export interface DraftTheme {
  question: string;
  message: string;
  resolution: string;
  arguments: DraftThemeArgument[];
}

export interface DraftScene {
  code: string;
  displayName: string;
  position: number;
  summary: string;
}

export interface DraftChapter {
  displayName: string;
  position: number;
  summary: string;
  scenes: DraftScene[];
}

export interface DraftArc {
  id: string;
  number: number;
  title: string;
  subnote: string;
  segmentsCount: number;
  summary: string;
  motif: {
    code: string;
    displayName: string;
    category: string;
    note?: string;
  };
  theme: DraftTheme;
  segments: DraftSegment[];
  chapters?: DraftChapter[];
}

export interface CharacterAppearance {
  species: string;
  birth: string;
  gender: { identity: string; expression: string };
  heightCm: number;
  weightKg: number;
  hair: { color: string; style: string; texture: string; length: string };
  skin: { tone: string; texture: string };
  eyes: { 
    pupil: { shape: string; color: string }; 
    iris: { texture: string | null; color: string }; 
    sclera: string; 
  };
  outfitStyle: string;
  distinguishingMarks: string[];
  summary: string;
}

export interface CharacterCompetence {
  id: string;
  name: string;
  domain: string;
  description: string;
  sceneLearned: string;
  capabilities: string[];
  limitations: string[];
}

export interface CharacterMemory {
  id: string;
  type: 'Episodic' | 'Semantic' | 'Procedural';
  status: 'Active' | 'Suppressed' | 'Forgotten' | 'Distorted';
  content: string;
  sourceScene: string;
}

export interface CharacterPsychological {
  coreTrauma: { wound: string; lie: string; fear: string };
  moralFramework: { alignment: string; code: string; breakingPoint: string };
  narrativeEngine: { want: string; need: string; conflict: string };
}

export interface DraftCharacter {
  id: string;
  name: string;
  nickname?: string | null;
  gender: string;
  genderBadge: string;
  motif: {
    description: string;
  };
  appearance: CharacterAppearance;
  competences: CharacterCompetence[];
  memories: CharacterMemory[];
  psychological: CharacterPsychological;
}

export interface DraftLoreEntry {
  code: string;
  displayName: string;
  type: 'ITEM' | 'HISTORY' | 'FACTION' | 'CULTURE';
  description: string;
  data: string;
}

export interface DraftWorldSettings {
  code: string;
  displayName: string;
  type: string;
  parentCode?: string;
  description: string;
  access: string;
  loreEntries: DraftLoreEntry[];
}

export interface DraftVersion {
  version: string;
  updatedAt: string;
}

export interface SceneCharacter {
  characterId?: string; // ID dùng cho query và deep-link — KHÔNG hiển thị trên UI
  characterCode?: string; // Slug ngữ nghĩa — KHÔNG hiển thị trên UI
  displayName: string;
  isPov: boolean;
}

export type SceneLoreUsage =
  | 'SETTING'
  | 'REFERENCE'
  | 'FORESHADOW'
  | 'PAYOFF'
  | 'REVEAL'
  | 'CONFLICT'
  | 'CONSTRAINT'
  | 'RED_HERRING';

export type SceneLoreReveal = 'NONE' | 'HINT' | 'PARTIAL' | 'FULL';

export interface SceneLoreEntry {
  code: string;
  displayName?: string;
  usage: SceneLoreUsage;
  revealToReader: SceneLoreReveal;
  revealToPov: SceneLoreReveal;
}

export interface SceneConfig {
  code: string;
  displayName: string;
  position: number;
  summary: string;
  worldSettingId?: string; // ID địa điểm từ bước 3 (World building) — KHÔNG hiển thị trên UI
  worldSettingCode?: string; // Slug địa điểm từ bước 3 (World building)
  stylePresetCode: string;
  narratorConfigCode: string;
  loreEntriesCode?: string | null;
  loreEntries?: Array<{
    code: string;
    displayName?: string;
    usage: 'SETTING' | 'REFERENCE' | 'FORESHADOW' | 'PAYOFF' | 'REVEAL' | 'CONFLICT' | 'CONSTRAINT' | 'RED_HERRING';
    revealToReader: 'NONE' | 'HINT' | 'PARTIAL' | 'FULL';
    revealToPov: 'NONE' | 'HINT' | 'PARTIAL' | 'FULL';
  }>;
  characters: SceneCharacter[];
}

export interface ChapterConfig {
  displayName: string;
  position: number;
  scenes: SceneConfig[];
}

export interface ArcSceneConfig {
  displayName: string;
  position: number;
  chapters: ChapterConfig[];
}

export interface SceneConfigPhase {
  arcs: ArcSceneConfig[];
}

export interface DraftRelationship {
  id: string;
  fromCharacterName: string;
  toCharacterName: string;
  role: {
    hierarchy: 'SUPERIOR' | 'EQUAL' | 'INFERIOR';
    category: 'FAMILY' | 'ROMANTIC' | 'SOCIAL' | 'PROFESSIONAL' | 'ANTAGONISTIC';
    subType: string;
    isPrimary: boolean;
  };
  states: {
    trust: string;
    affection: string;
    hostility: string;
    fear: string;
    respect: string;
    familiarity: string;
    dependency: string;
  };
  motif?: {
    code: string;
    description: string;
  };
}

export interface DraftOverview {
  currentPhase: string;
  version: string;
  updatedAt: string;
  versionHistory?: DraftVersion[];
  displayName: string;
  language: string;
  summary: string;
  target: {
    arcs: number;
    chapters: number;
    wordPerChapter: number;
  };
  genres: DraftGenre[];
  arcs: DraftArc[];
  characters: DraftCharacter[];
  worldSettings?: DraftWorldSettings;
  /** All Phase 3 world settings (the single field above stays for compatibility). */
  worldSettingsList?: DraftWorldSettings[];
  sceneConfig?: SceneConfigPhase;
  relationships?: DraftRelationship[];
}
