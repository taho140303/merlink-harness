export type DraftStep =
  | 'overview'
  | 'arcs'
  | 'chapter'
  | 'scene'
  | 'config'
  | 'world'
  | 'character'
  | 'characterDetail';

export interface FeedbackContext {
  id: string;
  tag: string;
  title: string;
  content?: string;
}

// --- Motif System (Self-contained) ---
export type MotifCategory = 'character_arc' | 'relationship' | 'symbolic' | 'plot';
export type MotifScope = 'project' | 'arc' | 'chapter';

export interface MotifTransition {
  id: string;
  name: string;
  targetEventCode: string;
  priority: number;
}

export interface MotifEventBlock {
  id: string;
  code: string;
  component: string; // Trigger component
  state: string;     // Resulting state
  emotionalRanges: string[];
  transitions: MotifTransition[];
  downByRules?: { ruleName: string; isAllowed: boolean }[]; // From state_down_by_rules
}

export interface MotifConstraint {
  code: string;
  require: string;
  reason: string;
}

export interface DraftMotif {
  id: string;
  code: string;
  name: string;
  category: MotifCategory;
  scope: MotifScope;
  from: string;
  version: string;
  isSystem: boolean;
  description: string;
  minChapter?: number;
  maxChapter?: number;
  
  genres: string[];        // motif_applicable_genres
  symbolics: string[];     // symbolic_motifs
  constraints: MotifConstraint[]; // motif_constraints
  eventBlocks: MotifEventBlock[];
}

// --- General Project ---
export interface DraftOverview {
  title: string;
  author: string;
  language: string;
  summarize: string;
  targetWords: string;
  motifs: DraftMotif[];
}

export type ThemeScope = 'PROJECT' | 'ARC';
export type ThemeResolutionType = 'RESOLVED' | 'OPEN' | 'AMBIGUOUS';
export type ThemeArgumentRole = 'PRIMARY' | 'ANTITHESIS' | 'CAUTIONARY' | 'WITNESS';
export type ThemeAnswerMode = 'VERBAL' | 'ACTION' | 'OUTCOME' | 'IMPLICIT';

export interface DraftThemeArgument {
  id: string;
  characterName: string;
  role: ThemeArgumentRole;
  answerMode: ThemeAnswerMode;
  note: string;
}

export interface DraftTheme {
  id: string;
  code: string;
  scope: ThemeScope;
  question: string;
  resolutionType: ThemeResolutionType;
  resolution: string;
  message?: string;
  arguments: DraftThemeArgument[];
}

export interface DraftArc {
  id: string;
  name: string;
  summarize: string;
  themes: DraftTheme[];
  motifs: DraftMotif[];
}

export interface DraftChapter {
  id: string;
  arcId: string;
  name: string;
  description: string;
  motifs: DraftMotif[];
}

export interface SceneCharacter {
  name: string;
  isPov: boolean;
}

export interface DraftScene {
  id: string;
  chapterId: string;
  name: string;
  summarize: string;
  stylePreset: string;
  narratorConfig: string;
  characters: SceneCharacter[];
  forbidden: string[];
  allowed: string[];
}

export interface DraftConfig {
  id: string;
  targetScope: string;
  theme: string;
}

export interface DraftWorld {
  id: string;
  locationName: string;
  locationType: string;
  sensory: string;
  lore: string[];
}

export interface DraftCharacter {
  id: string;
  name: string;
  role: string;
  appearances: string[];
  motifs: DraftMotif[]; // Character-specific motifs
}

export interface DraftCharacterDetail {
  id: string;
  name: string;
  personality: string;
  actionLogic: string;
  themeAlignment: string;
  sceneRole: string;
}
