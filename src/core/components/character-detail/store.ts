import { create } from 'zustand';
import type {
  CharacterFullProfile,
  DetailTab,
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
  toggleEditing: () => set((s) => ({ isEditing: !s.isEditing })),
  toggleChat: () => set((s) => ({ isChatOpen: !s.isChatOpen })),
  setActiveTab: (activeTab) => set({ activeTab }),
  setActiveLifeTab: (activeLifeTab) => set({ activeLifeTab }),
}));
