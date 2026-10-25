export type IntensityLevel = 'NONE' | 'LOW' | 'MODERATE' | 'HIGH' | 'ABSOLUTE';
export type Hierarchy = 'Superior' | 'Equal' | 'Inferior';
export type RelationshipCategory = 'Family' | 'Romantic' | 'Social' | 'Professional' | 'Rival';
export type KnowledgeCertainty = 'Low' | 'Medium' | 'High';
export type KnowledgeType = 'Initial' | 'Lore';
export type MemoryType = 'Episodic' | 'Semantic' | 'Procedural';
export type MemoryStatus = 'Active' | 'Suppressed' | 'Forgotten' | 'Distorted';
export type DetailTab = 'core' | 'life' | 'psych';
export type LifeSubTab = 'knowledge' | 'competence' | 'memory';

export interface ColorOption {
  value: string;
  hex?: string;
}

export interface CharacterBasicInfo {
  name: string;
  species: string;
  biologicalSex: string;
  genderIdentity: string;
  genderExpression: string[];
}

export interface Appearance {
  height: number;
  weight: number;
  hair: ColorOption;
  eyes: ColorOption;
  skin: ColorOption;
  hairNote: string;
  eyesNote: string;
  skinNote: string;
  outfitStyle: string;
  defaultSummary: string;
}

export interface DistinguishingMark {
  id: string;
  content: string;
}

export interface RelationshipStates {
  trust: IntensityLevel;
  affection: IntensityLevel;
  hostility: IntensityLevel;
  fear: IntensityLevel;
  respect: IntensityLevel;
  familiarity: IntensityLevel;
  dependency: IntensityLevel;
}

export interface Relationship {
  id: string;
  personName: string;
  role: Hierarchy;
  category: RelationshipCategory;
  states: RelationshipStates;
}

export interface KnowledgeSource {
  id: string;
  type: KnowledgeType;
  path: string;
  certainty: KnowledgeCertainty;
}

export interface Competence {
  id: string;
  name: string;
  domain: string;
  level: string;
  reliability: KnowledgeCertainty;
  description: string;
  experience: string;
  capabilities: string[];
  limitations: string[];
}

export interface MemoryEmotion {
  name: string;
  intensity: number;
}

export interface MemoryReference {
  role: string;
  name: string;
}

export interface MemoryRevision {
  revisionNumber: number;
  reason: string;
  change: string;
}

export interface MemoryConnection {
  label: string;
  targetMemoryId: string;
  targetSummary: string;
}

export interface Memory {
  id: string;
  summary: string;
  type: MemoryType;
  status: MemoryStatus;
  recollection: string;
  interpretation: string;
  certainty: KnowledgeCertainty;
  knowledgeSource: string;
  occurredAt: string;
  perceivedAt: string;
  references: MemoryReference[];
  emotions: MemoryEmotion[];
  connections: MemoryConnection[];
  revisions: MemoryRevision[];
}

export interface PsychologicalProfile {
  coreTrauma: {
    theWound: string;
    theLie: string;
    fundamentalFear: string;
  };
  cognitiveMoral: {
    moralAlignment: string;
    moralCode: string;
    cognitiveBias: string;
    breakingPoint: string;
  };
  narrativeEngine: {
    externalWant: string;
    internalNeed: string;
    internalConflict: string;
  };
  behavioralAdaptation: {
    socialMask: { personaType: string; verbalStyle: string };
    defenseMechanisms: {
      primaryReaction: string;
      stressTrigger: string;
      copingStrategy: string;
    };
  };
}

export interface CharacterFullProfile {
  id: string;
  basicInfo: CharacterBasicInfo;
  appearance: Appearance;
  distinguishingMarks: DistinguishingMark[];
  relationships: Relationship[];
  knowledge: KnowledgeSource[];
  competences: Competence[];
  memories: Memory[];
  psychological: PsychologicalProfile;
}
