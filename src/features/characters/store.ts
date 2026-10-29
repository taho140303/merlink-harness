import { create } from 'zustand';
import type {
  CharacterAppearance,
  CharacterBasicInfo,
  CharacterProfile,
  DetailTab,
  LifeSubTab,
} from './types';
import { MOCK_CHARACTERS_MAP, MOCK_CHARACTERS } from './mockData';

interface CharacterState {
  charactersList: CharacterProfile[];
  character: CharacterProfile;
  editSnapshot: CharacterProfile | null;
  isEditing: boolean;
  isChatOpen: boolean;
  activeTab: DetailTab;
  activeLifeTab: LifeSubTab;
  
  // 1.1: Sửa lỗi fix cứng — load nhân vật theo param ID
  loadCharacter: (id: string) => void;
  addCharacter: (newChar: CharacterProfile) => void;
  setCharacter: (c: CharacterProfile) => void;
  patchCharacter: (updater: (draft: CharacterProfile) => void) => void;
  updateBasicInfo: (partial: Partial<CharacterBasicInfo>) => void;
  updateAppearance: (partial: Partial<CharacterAppearance>) => void;
  addMark: (content: string) => void;
  removeMark: (id: string) => void;
  startEditing: () => void;
  saveEditing: () => void;
  cancelEditing: () => void;
  toggleChat: () => void;
  setActiveTab: (t: DetailTab) => void;
  setActiveLifeTab: (t: LifeSubTab) => void;
}

export const useCharacterStore = create<CharacterState>((set, get) => ({
  charactersList: MOCK_CHARACTERS,
  character: MOCK_CHARACTERS[0],
  editSnapshot: null,
  isEditing: false,
  isChatOpen: false,
  activeTab: 'core',
  activeLifeTab: 'competence',

  loadCharacter: (id: string) => {
    const list = get().charactersList;
    const found =
      list.find((c) => c.id === id || c.code === id) ||
      MOCK_CHARACTERS_MAP[id] ||
      list[0];
    if (found && found.id !== get().character.id) {
      set({ character: found, editSnapshot: null, isEditing: false });
    }
  },

  addCharacter: (newChar: CharacterProfile) => {
    set((s) => ({
      charactersList: [newChar, ...s.charactersList],
    }));
  },

  setCharacter: (character) => set({ character }),

  patchCharacter: (updater) =>
    set((s) => {
      const clone = structuredClone(s.character);
      updater(clone);
      return { character: clone };
    }),

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

  startEditing: () =>
    set((s) => ({
      isEditing: true,
      editSnapshot: structuredClone(s.character),
    })),

  saveEditing: () => set({ isEditing: false, editSnapshot: null }),

  cancelEditing: () =>
    set((s) => ({
      isEditing: false,
      character: s.editSnapshot ?? s.character,
      editSnapshot: null,
    })),

  toggleChat: () => set((s) => ({ isChatOpen: !s.isChatOpen })),
  setActiveTab: (activeTab) => set({ activeTab }),
  setActiveLifeTab: (activeLifeTab) => set({ activeLifeTab }),
}));

// Backward compatibility alias
export const useCharacterDetailStore = useCharacterStore;
