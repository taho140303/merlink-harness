export type SortOption = "a-z" | "z-a" | "modified";

export interface ChapterBlock {
  id: string;
  /** Stable scene identity supplied by the read model. Preview data derives one from the chapter and block id. */
  sceneId?: string;
  text: string;
  locked: boolean;
}

export interface ChapterItem {
  id: string;
  title: string;
  date: string;
  isNew?: boolean;
  blocks?: ChapterBlock[];
}

export interface ArcItem {
  id: string;
  title: string;
  cover?: string;
  chapters: ChapterItem[];
}

export interface ProjectItem {
  id: string;
  title: string;
  status: "Drafting" | "Completed" | "In Review" | "Ongoing" | "Planning";
  cover: string;
  description: string;
  genres: string[];
  modifiedAt: string;
  details: {
    wordCount: string;
    chapters: string;
    characters: string;
    worldLore: string;
    createdDate: string;
  };
  arcs?: ArcItem[];
}
