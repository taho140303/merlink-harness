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

export interface DraftOverview {
  title: string;
  author: string;
  language: string;
  summarize: string;
  targetWords: string;
}

export interface DraftArc {
  id: string;
  name: string;
  summarize: string;
}

export interface DraftChapter {
  id: string;
  arcId: string;
  name: string;
  description: string;
}

export interface DraftScene {
  id: string;
  chapterId: string;
  name: string;
  summarize: string;
  stylePreset: string;
  narratorConfig: string; // POV
  characters: string[];
  forbidden: string[];
  allowed: string[];
}

export interface DraftConfig {
  id: string;
  targetScope: string; // e.g. "Arc 1" or "Chapter 1"
  theme: string;
  motifs: string[];
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
  appearances: string[]; // "Chapter 1 - Scene 2"
}

export interface DraftCharacterDetail {
  id: string;
  name: string;
  personality: string;
  actionLogic: string;
  themeAlignment: string;
  sceneRole: string;
}
