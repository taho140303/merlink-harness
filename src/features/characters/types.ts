export type IntensityLevel = 'NONE' | 'LOW' | 'MODERATE' | 'HIGH' | 'ABSOLUTE';
export type RelationDimensionLevel = IntensityLevel;

// 2.3: Theo DBML
export type Hierarchy = 'SUPERIOR' | 'EQUAL' | 'INFERIOR';
export type CategoryGroup = 'FAMILY' | 'ROMANTIC' | 'SOCIAL' | 'PROFESSIONAL' | 'ANTAGONISTIC';

export type DetailTab = 'core' | 'life' | 'psych';
export type LifeSubTab = 'competence' | 'memory';

export const INTENSITY_LEVELS: IntensityLevel[] = [
  'NONE',
  'LOW',
  'MODERATE',
  'HIGH',
  'ABSOLUTE',
];

// 1.2: Chỉ có NONE, MALE, FEMALE
export const GENDER_OPTIONS = ['NONE', 'MALE', 'FEMALE'] as const;
export type Gender = (typeof GENDER_OPTIONS)[number];

export interface CharacterBasicInfo {
  name: string;
  species: string;
  biologicalSex: Gender;
  genderIdentity: Gender;
  genderExpression: Gender;
}

// 2.2: Bỏ hairHex, eyeHex, skinHex theo DBML
export interface CharacterAppearance {
  height: number;
  weight: number;
  hair: string;
  eyes: string;
  skin: string;
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

// 2.3: Theo DBML (Hierarchy + CategoryGroup)
export interface CharacterRelationship {
  id: string;
  targetId?: string;
  targetName: string;
  role: Hierarchy;
  category: CategoryGroup;
  states: RelationshipStates;
}

// 2.1: Thu gọn theo DBML (character_memories phẳng)
export interface CharacterMemory {
  id: string;
  content: string;
  sourceScene?: string;
  createdAt?: string;
}

// 2.1: Competence (nếu không có capabilities và chỉ có limitations thì là knowledge)
export interface CharacterCompetence {
  id: string;
  code?: string;
  name: string;
  domain: string;
  level?: string;
  reliability?: string;
  description: string;
  experience?: string;
  capabilities: string[];
  limitations: string[];
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

export interface CharacterProfile {
  id: string;
  code?: string;
  basicInfo: CharacterBasicInfo;
  avatarBg?: string;
  appearance: CharacterAppearance;
  distinguishingMarks: DistinguishingMark[];
  relationships: CharacterRelationship[];
  competences: CharacterCompetence[];
  memories: CharacterMemory[];
  psychological?: PsychologicalProfile;
}

// Alias hỗ trợ chuyển đổi mượt mà
export type CharacterFullProfile = CharacterProfile;
export type CharacterItem = CharacterProfile;
