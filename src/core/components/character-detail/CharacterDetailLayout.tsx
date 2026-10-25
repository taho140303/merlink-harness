import { useCharacterDetailStore } from './store';
import { ChatSplitView } from './shared/ChatSplitView';
import { CoreIdentityTab } from './tabs/CoreIdentityTab';
import { LifeExperienceTab } from './tabs/LifeExperienceTab';
import { PsychologicalTab } from './tabs/PsychologicalTab';
import type { DetailTab } from './types';

const tabLabels: Record<DetailTab, string> = {
  core: 'Core Identity',
  life: 'Life Experience',
  psych: 'Psychological Profile',
};

export function CharacterDetailLayout() {
  const character = useCharacterDetailStore((s) => s.character);
  const activeTab = useCharacterDetailStore((s) => s.activeTab);
  const setActiveTab = useCharacterDetailStore((s) => s.setActiveTab);
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const toggleEditing = useCharacterDetailStore((s) => s.toggleEditing);
  const isChatOpen = useCharacterDetailStore((s) => s.isChatOpen);
  const toggleChat = useCharacterDetailStore((s) => s.toggleChat);

  return (
    <div className="min-h-screen flex flex-col bg-gray-50">
      <header className="bg-white border-b border-gray-200">
        <div className="px-6 pt-5 pb-3 flex items-start justify-between gap-4">
          <div className="flex items-center gap-3">
            <div className="h-12 w-12 rounded-full bg-linear-to-br from-indigo-400 to-purple-500 flex items-center justify-center text-white font-bold text-lg">
              {character.basicInfo.name.charAt(0)}
            </div>
            <div>
              <h1 className="text-xl font-bold text-gray-900">
                {character.basicInfo.name}
              </h1>
              <p className="text-xs text-gray-500">{character.basicInfo.species}</p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              type="button"
              onClick={toggleEditing}
              className={`px-3 py-1.5 text-sm rounded-lg font-medium transition-colors ${
                isEditing
                  ? 'bg-amber-500 text-white hover:bg-amber-600'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              {isEditing ? 'Done' : 'Edit'}
            </button>
            <button
              type="button"
              onClick={toggleChat}
              className={`px-3 py-1.5 text-sm rounded-lg font-medium transition-colors ${
                isChatOpen
                  ? 'bg-indigo-600 text-white hover:bg-indigo-700'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              {isChatOpen ? 'Close Chat' : 'Open Chat'}
            </button>
          </div>
        </div>
        <nav className="px-6 flex gap-1 pb-2 overflow-x-auto">
          {(Object.keys(tabLabels) as DetailTab[]).map((tab) => (
            <button
              key={tab}
              type="button"
              onClick={() => setActiveTab(tab)}
              className={`shrink-0 px-3 py-2 text-sm rounded-lg font-medium transition-colors ${
                activeTab === tab
                  ? 'bg-indigo-50 text-indigo-700'
                  : 'text-gray-600 hover:bg-gray-100'
              }`}
            >
              {tabLabels[tab]}
            </button>
          ))}
        </nav>
      </header>

      <ChatSplitView>
        <div className="p-6 max-w-6xl mx-auto w-full">
          {activeTab === 'core' && <CoreIdentityTab />}
          {activeTab === 'life' && <LifeExperienceTab />}
          {activeTab === 'psych' && <PsychologicalTab />}
        </div>
      </ChatSplitView>
    </div>
  );
}
