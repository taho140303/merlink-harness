export type MeasurementUnit = "cm" | "m" | "ft" | "in" | "kg" | "lbs";

export type RelationshipStateLevel = "NONE" | "LOW" | "MODERATE" | "HIGH" | "ABSOLUTE";

export interface RelationshipStates {
  trust: RelationshipStateLevel;
  affection: RelationshipStateLevel;
  hostility: RelationshipStateLevel;
  fear: RelationshipStateLevel;
  respect: RelationshipStateLevel;
  familiarity: RelationshipStateLevel;
  dependency: RelationshipStateLevel;
}

export interface CharacterRelationship {
  id: string;
  targetName: string;
  category: string;
  role: "superior" | "equal" | "inferior";
  states: RelationshipStates;
}

export interface DistinguishingMark {
  id: string;
  location: string;
  description: string;
}

export interface CharacterAppearance {
  height: number;
  heightUnit: MeasurementUnit;
  weight: number;
  weightUnit: MeasurementUnit;
  hairColor: string;
  hairHex: string;
  hairNote: string;
  eyeColor: string;
  eyeHex: string;
  eyeNote: string;
  skinTone: string;
  skinHex: string;
  skinNote: string;
  outfitStyle: string;
  defaultSummary: string;
}

export interface CharacterItem {
  id: string;
  name: string;
  species: string;
  biologicalSex: "Male" | "Female" | "Intersex" | "Unknown";
  genderIdentity: string;
  genderExpression: string;
  avatarBg: string;
  appearance: CharacterAppearance;
  distinguishingMarks: DistinguishingMark[];
  relationships: CharacterRelationship[];
  lockedFields?: Record<string, boolean>;
}
