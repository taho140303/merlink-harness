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

export interface DraftThemeArgument {
  id: string;
  characterName: string;
  role: string;
  reactionType: string;
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

export interface DraftCharacter {
  id: string;
  name: string;
  gender: string;
  genderBadge: string;
  motif: {
    description: string;
  };
}

export interface DraftOverview {
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
}
