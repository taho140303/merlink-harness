import { useCharacterStore } from './store';
import type { CharacterMemory } from './types';

// 2.1: Thu gọn theo DBML (character_memories phẳng)
export function MemoryCard({ memory }: { memory: CharacterMemory }) {
  const isEditing = useCharacterStore((s) => s.isEditing);
  const patch = useCharacterStore((s) => s.patchCharacter);

  const update = (content: string) =>
    patch((c) => {
      const m = c.memories.find((x) => x.id === memory.id);
      if (m) m.content = content;
    });

  return (
    <div className="rounded-xl border border-line bg-surface-card p-4 transition-colors">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0 flex-1">
          {isEditing ? (
            <textarea
              rows={3}
              value={memory.content}
              onChange={(e) => update(e.target.value)}
              className="w-full rounded-md tier-input focus-ring px-2 py-1 text-sm text-fg"
            />
          ) : (
            <p className="whitespace-pre-wrap text-sm leading-relaxed text-fg">
              {memory.content}
            </p>
          )}
          <div className="mt-2 flex flex-wrap items-center gap-2 text-xs text-fg-muted">
            {memory.sourceScene && (
              <span className="rounded-md bg-surface-muted px-2 py-0.5 font-medium text-fg-muted">
                {memory.sourceScene}
              </span>
            )}
            {memory.createdAt && (
              <span className="text-fg-subtle">
                {memory.createdAt}
              </span>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
