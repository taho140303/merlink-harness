// MOTIF
export type MotifCategory = 'character_arc' | 'relationship' | 'symbolic' | 'plot';

export interface Motif {
  id: string;
  code: string;
  name: string;
  category: MotifCategory;
  version: string;
  isSystem: boolean;
  description: string;
  minChapter?: number;
  maxChapter?: number;
}

export interface Genre {
  id: string;
  code: string;
  name: string;
  description: string;
}

export interface MotifApplicableGenre {
  id: string;
  motifId: string;
  genreId: string;
}

export interface Component {
  id: string;
  code: string;
  name: string;
  description: string;
}

export interface StateItem {
  id: string;
  code: string;
  name: string;
  description: string;
}

export interface StateRule {
  id: string;
  code: string;
  name: string;
}

export interface StateDownByRule {
  id: string;
  stateId: string;
  stateRuleId: string;
  isAllowed: boolean;
}

export interface EmotionalRange {
  id: string;
  code: string;
  name: string;
}

export interface EventBlock {
  id: string;
  code: string;
  componentId: string;
  stateId: string;
  motifId: string;
}

export interface EventTransition {
  id: string;
  name: string;
  fromEventBlockId: string;
  toEventBlockId: string;
  priority: number;
}

export interface EventBlockEmotional {
  id: string;
  emotionalRangeId: string;
  eventBlockId: string;
}

export interface Constraint {
  id: string;
  code: string;
  require: string;
  reason: string;
}

export interface MotifConstraint {
  id: string;
  code: string;
  constraintId: string;
  motifId: string;
}

// STYLE PRESET
export type NarrativeWeight = 'NONE' | 'LOW' | 'MEDIUM' | 'HIGH';
export type SentenceLength = 'VERY_SHORT' | 'SHORT' | 'MEDIUM' | 'LONG';
export type SentenceStructure = 'SIMPLE' | 'COMPOUND' | 'COMPLEX' | 'FRAGMENT' | 'MIXED';
export type TonePace = 'VERY_LOW' | 'LOW' | 'MODERATE' | 'FAST' | 'VERY_FAST';

export interface StylePreset {
  id: string;
  code: string;
  category: string;
  tonePace: TonePace;
  toneEmotion: string;
  isSystem: boolean;
  visual: NarrativeWeight;
  auditory: NarrativeWeight;
  tactile: NarrativeWeight;
  olfactory: NarrativeWeight;
  gustatory: NarrativeWeight;
  languageAbstract: NarrativeWeight;
  syntaxSentence: SentenceLength;
  syntaxVariation: NarrativeWeight;
  syntaxStructure: SentenceStructure;
}

export type RhetoricalUsage = 'primary' | 'secondary' | 'avoid';

export interface RhetoricalDevice {
  id: string;
  code: string;
  description: string;
}

export interface StylePresetRhetoricalDevice {
  id: string;
  presetId: string;
  rhetoricalDeviceId: string;
  usageType: RhetoricalUsage;
  customInstruction: string;
}

export interface StylePresetVocabItem {
  id: string;
  presetId: string;
  content: string;
}

export interface StylePresetFocusItem {
  id: string;
  presetId: string;
  content: string;
}

// NARRATIVE DIRECTIVES
export type GrammaticalPerson = 'FIRST_PERSON' | 'SECOND_PERSON' | 'THIRD_PERSON';
export type PersonSubtype =
  | 'PROTAGONIST'
  | 'WITNESS_PERIPHERAL'
  | 'UNRELIABLE_FIRST'
  | 'DIRECT_ADDRESS'
  | 'SELF_REFLECTION'
  | 'INTERACTIVE'
  | 'OMNISCIENT'
  | 'LIMITED'
  | 'OBJECTIVE';
export type PovScope = 'SINGLE_POV' | 'MULTIPLE_POV' | 'SHIFTING_OMNISCIENT';

export interface NarratorConfig {
  id: string;
  code: string;
  name: string;
  person: GrammaticalPerson;
  personSubtype: PersonSubtype;
  povScope: PovScope;
  isReliable: boolean;
  breaksFourthWall: boolean;
  description: string;
}

export type DirectiveCategory = 'PROSE_PRINCIPLE' | 'RESTRAINT' | 'INFORMATION_CONTROL';

export interface NarrativeDirective {
  id: string;
  code: string;
  description: string;
  category: DirectiveCategory;
  isSystem: boolean;
}

export interface StylePresetNarrative {
  id: string;
  stylePresetId: string;
  narrativeDirectiveId: string;
  weight: NarrativeWeight;
}

// THEMES
export type ThemeScope = 'PROJECT' | 'ARC';
export type ThemeResolutionType = 'RESOLVED' | 'OPEN' | 'AMBIGUOUS';
export type ThemeArgumentRole = 'PRIMARY' | 'ANTITHESIS' | 'CAUTIONARY' | 'WITNESS';
export type ThemeAnswerMode = 'VERBAL' | 'ACTION' | 'OUTCOME' | 'IMPLICIT';

export interface Theme {
  id: string;
  code: string;
  scope: ThemeScope;
  projectId: string;
  arcId?: string;
  parentThemeId?: string;
  question: string;
  resolutionType: ThemeResolutionType;
  resolution: string;
  message?: string;
  createdAt: string;
  updatedAt: string;
}

export interface ThemeArgument {
  id: string;
  themeId: string;
  characterId: string;
  characterName: string;
  role: ThemeArgumentRole;
  answerMode: ThemeAnswerMode;
  note: string;
  requiredKnowledge?: string;
  requiredCompetence?: string;
}
