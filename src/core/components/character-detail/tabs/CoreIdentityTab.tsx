import { useState, type ReactNode } from 'react';
import { Plus, Trash2 } from 'lucide-react';
import { useCharacterDetailStore } from '../store';
import { FieldDisplay } from '../shared/FieldDisplay';
import { EditableField } from '../shared/EditableField';
import { RelationshipCard } from '../RelationshipCard';

function Section({
  title,
  defaultOpen = true,
  children,
}: {
  title: string;
  defaultOpen?: boolean;
  children: ReactNode;
}) {
  const [open, setOpen] = useState(defaultOpen);
  return (
    <div className="rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <button
        type="button"
        onClick={() => setOpen(!open)}
        className="flex w-full items-center justify-between px-5 py-3.5 text-left"
      >
        <span className="font-semibold text-slate-900 dark:text-slate-100">{title}</span>
        <span className="text-sm text-slate-400">{open ? '−' : '+'}</span>
      </button>
      {open && <div className="px-5 pb-5">{children}</div>}
    </div>
  );
}

export function CoreIdentityTab() {
  const character = useCharacterDetailStore((s) => s.character);
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const updateBasicInfo = useCharacterDetailStore((s) => s.updateBasicInfo);
  const updateAppearance = useCharacterDetailStore((s) => s.updateAppearance);
  const addMark = useCharacterDetailStore((s) => s.addMark);
  const removeMark = useCharacterDetailStore((s) => s.removeMark);

  return (
    <div className="space-y-4">
      <Section title="Basic Info">
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <EditableField
            label="Name"
            value={character.basicInfo.name}
            onChange={(v) => updateBasicInfo({ name: v })}
          />
          <EditableField
            label="Species"
            value={character.basicInfo.species}
            onChange={(v) => updateBasicInfo({ species: v })}
          />
          <EditableField
            label="Biological Sex"
            value={character.basicInfo.biologicalSex}
            onChange={(v) => updateBasicInfo({ biologicalSex: v })}
          />
          <EditableField
            label="Gender Identity"
            value={character.basicInfo.genderIdentity}
            onChange={(v) => updateBasicInfo({ genderIdentity: v })}
          />
          <div className="sm:col-span-2">
            <FieldDisplay
              label="Gender Expression"
              value={
                <div className="flex flex-wrap gap-1.5">
                  {character.basicInfo.genderExpression.map((tag) => (
                    <span
                      key={tag}
                      className="rounded-full bg-indigo-100 px-2.5 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              }
            />
          </div>
        </div>
      </Section>

      <Section title="Appearance">
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <EditableField
              label="Height (cm)"
              value={String(character.appearance.height)}
              onChange={(v) => updateAppearance({ height: Number(v) || 0 })}
            />
            <EditableField
              label="Weight (kg)"
              value={String(character.appearance.weight)}
              onChange={(v) => updateAppearance({ weight: Number(v) || 0 })}
            />
          </div>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
            <FieldDisplay
              label="Hair"
              value={character.appearance.hair.value}
              colorHex={character.appearance.hair.hex}
            />
            <FieldDisplay
              label="Eyes"
              value={character.appearance.eyes.value}
              colorHex={character.appearance.eyes.hex}
            />
            <FieldDisplay
              label="Skin"
              value={character.appearance.skin.value}
              colorHex={character.appearance.skin.hex}
            />
          </div>
          <EditableField
            label="Hair Notes"
            value={character.appearance.hairNote}
            onChange={(v) => updateAppearance({ hairNote: v })}
            multiline
          />
          <EditableField
            label="Eyes Notes"
            value={character.appearance.eyesNote}
            onChange={(v) => updateAppearance({ eyesNote: v })}
            multiline
          />
          <EditableField
            label="Skin Notes"
            value={character.appearance.skinNote}
            onChange={(v) => updateAppearance({ skinNote: v })}
            multiline
          />
          <EditableField
            label="Outfit Style"
            value={character.appearance.outfitStyle}
            onChange={(v) => updateAppearance({ outfitStyle: v })}
            multiline
          />
          <EditableField
            label="Default Summary"
            value={character.appearance.defaultSummary}
            onChange={(v) => updateAppearance({ defaultSummary: v })}
            multiline
          />
        </div>
      </Section>

      <Section title="Distinguishing Marks">
        <ul className="space-y-2">
          {character.distinguishingMarks.map((mark) => (
            <li
              key={mark.id}
              className="flex items-center justify-between gap-2 rounded-lg bg-slate-50 px-3 py-2 dark:bg-slate-800/60"
            >
              <span className="text-sm text-slate-800 dark:text-slate-200">{mark.content}</span>
              {isEditing && (
                <button
                  type="button"
                  onClick={() => removeMark(mark.id)}
                  className="shrink-0 text-rose-500 hover:text-rose-600"
                  title="Remove mark"
                >
                  <Trash2 className="h-4 w-4" />
                </button>
              )}
            </li>
          ))}
        </ul>
        {isEditing && (
          <button
            type="button"
            onClick={() => addMark('New mark')}
            className="mt-3 flex items-center gap-1.5 text-sm text-indigo-600 hover:underline dark:text-indigo-400"
          >
            <Plus className="h-3.5 w-3.5" />
            Add Mark
          </button>
        )}
      </Section>

      <Section title="Relationships">
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {character.relationships.map((rel) => (
            <RelationshipCard key={rel.id} relationship={rel} />
          ))}
        </div>
      </Section>
    </div>
  );
}
