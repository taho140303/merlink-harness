import type { ReactNode } from 'react';
import { useCharacterDetailStore } from '../store';

export function ChatSplitView({ children }: { children: ReactNode }) {
  const isChatOpen = useCharacterDetailStore((s) => s.isChatOpen);

  return (
    <div className="flex-1 flex overflow-hidden">
      <main className="flex-1 overflow-y-auto bg-gray-50">{children}</main>
      {isChatOpen && (
        <aside className="w-[380px] shrink-0 border-l border-gray-200 bg-white flex flex-col">
          <div className="px-4 py-3 border-b border-gray-200 flex items-center justify-between">
            <h2 className="text-sm font-semibold text-gray-900">Chat</h2>
            <span className="text-[11px] text-gray-400">demo</span>
          </div>
          <div className="flex-1 overflow-y-auto p-4">
            <p className="text-sm text-gray-500 text-center py-8">
              Start a conversation with this character...
            </p>
          </div>
          <div className="border-t border-gray-200 p-3">
            <div className="flex gap-2">
              <input
                className="flex-1 text-sm border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                placeholder="Message..."
              />
              <button
                type="button"
                className="px-3 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
              >
                Send
              </button>
            </div>
          </div>
        </aside>
      )}
    </div>
  );
}
