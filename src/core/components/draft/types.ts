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

/** Dữ liệu của khối "Tổng quan dự án" — dùng chung cho mọi phase. */
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
}
