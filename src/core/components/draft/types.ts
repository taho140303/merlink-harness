export type DraftStep = "setup" | "character" | "plan";

export interface FeedbackContext {
  id: string;
  tag: string;
  title: string;
  content?: string;
}

export interface SetupBlock {
  id: string;
  tag: string;
  title: string;
  content: string;
  loadingTitle: string;
}

export interface CharacterRelationship {
  targetName: string;
  relation: string;
  description: string;
}

export interface CharacterProfile {
  id: string;
  name: string;
  role: string;
  loadingTitle: string;
  coreIdentity: {
    age: string;
    origin: string;
    allegiance: string;
    relationships: CharacterRelationship[];
  };
  lifeExperience: string;
  psychological: string;
}

export interface PlanPhase {
  id: string;
  tag: string;
  title: string;
  content: string;
  loadingTitle: string;
}
