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
