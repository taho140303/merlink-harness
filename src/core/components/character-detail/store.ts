import { create } from 'zustand';
import type {
  Appearance,
  CharacterBasicInfo,
  CharacterFullProfile,
  DetailTab,
  KnowledgeSource,
  LifeSubTab,
} from './types';
import { rimuruProfile } from './mockData';

interface CharacterDetailState {
  character: CharacterFullProfile;
  isEditing: boolean;
  isChatOpen: boolean;
  activeTab: DetailTab;
  activeLifeTab: LifeSubTab;
  setCharacter: (c: CharacterFullProfile) => void;
  updateBasicInfo: (partial: Partial<CharacterBasicInfo>) => void;
  updateAppearance: (partial: Partial<Appearance>) => void;
  addMark: (content: string) => void;
  removeMark: (id: string) => void;
  addKnowledge: (entry: Omit<KnowledgeSource, 'id'>) => void;
  updateKnowledgePath: (id: string, path: string) => void;
  removeKnowledge: (id: string) => void;
  toggleEditing: () => void;
  toggleChat: () => void;
  setActiveTab: (t: DetailTab) => void;
  setActiveLifeTab: (t: LifeSubTab) => void;
}

export const useCharacterDetailStore = create<CharacterDetailState>((set) => ({
  character: rimuruProfile,
  isEditing: false,
  isChatOpen: false,
  activeTab: 'core',
  activeLifeTab: 'knowledge',
  setCharacter: (character) => set({ character }),
  updateBasicInfo: (partial) =>
    set((s) => ({
      character: { ...s.character, basicInfo: { ...s.character.basicInfo, ...partial } },
    })),
  updateAppearance: (partial) =>
    set((s) => ({
      character: { ...s.character, appearance: { ...s.character.appearance, ...partial } },
    })),
  addMark: (content) =>
    set((s) => ({
      character: {
        ...s.character,
        distinguishingMarks: [
          ...s.character.distinguishingMarks,
          { id: `mark-${Date.now()}`, content },
        ],
      },
    })),
  removeMark: (id) =>
    set((s) => ({
      character: {
        ...s.character,
        distinguishingMarks: s.character.distinguishingMarks.filter((m) => m.id !== id),
      },
    })),
  addKnowledge: (entry) =>
    set((s) => ({
      character: {
        ...s.character,
        knowledge: [...s.character.knowledge, { id: `kn-${Date.now()}`, ...entry }],
      },
    })),
  updateKnowledgePath: (id, path) =>
    set((s) => ({
      character: {
        ...s.character,
        knowledge: s.character.knowledge.map((k) => (k.id === id ? { ...k, path } : k)),
      },
    })),
  removeKnowledge: (id) =>
    set((s) => ({
      character: {
        ...s.character,
        knowledge: s.character.knowledge.filter((k) => k.id !== id),
      },
    })),
  toggleEditing: () => set((s) => ({ isEditing: !s.isEditing })),
  toggleChat: () => set((s) => ({ isChatOpen: !s.isChatOpen })),
  setActiveTab: (activeTab) => set({ activeTab }),
  setActiveLifeTab: (activeLifeTab) => set({ activeLifeTab }),
}));
