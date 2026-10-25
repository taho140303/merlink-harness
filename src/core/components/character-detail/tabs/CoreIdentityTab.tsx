import { useState, type ReactNode } from 'react';
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
    <div className="rounded-xl border border-gray-200 bg-white shadow-sm">
      <button
        type="button"
        onClick={() => setOpen(!open)}
        className="w-full flex items-center justify-between px-5 py-3.5 text-left"
      >
        <span className="font-semibold text-gray-900">{title}</span>
        <span className="text-gray-400 text-sm">{open ? '−' : '+'}</span>
      </button>
      {open && <div className="px-5 pb-5">{children}</div>}
    </div>
  );
}

export function CoreIdentityTab() {
  const character = useCharacterDetailStore((s) => s.character);
  const isEditing = useCharacterDetailStore((s) => s.isEditing);

  return (
    <div className="space-y-4">
      <Section title="Basic Info">
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <EditableField label="Name" value={character.basicInfo.name} />
          <EditableField label="Species" value={character.basicInfo.species} />
          <EditableField label="Biological Sex" value={character.basicInfo.biologicalSex} />
          <EditableField label="Gender Identity" value={character.basicInfo.genderIdentity} />
          <div className="sm:col-span-2">
            <FieldDisplay
              label="Gender Expression"
              value={
                <div className="flex flex-wrap gap-1.5">
                  {character.basicInfo.genderExpression.map((tag) => (
                    <span
                      key={tag}
                      className="px-2.5 py-0.5 text-xs rounded-full bg-indigo-100 text-indigo-700 font-medium"
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
            <EditableField label="Height (cm)" value={String(character.appearance.height)} />
            <EditableField label="Weight (kg)" value={String(character.appearance.weight)} />
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
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
          <EditableField label="Hair Notes" value={character.appearance.hairNote} multiline />
          <EditableField label="Eyes Notes" value={character.appearance.eyesNote} multiline />
          <EditableField label="Skin Notes" value={character.appearance.skinNote} multiline />
          <EditableField label="Outfit Style" value={character.appearance.outfitStyle} multiline />
          <EditableField
            label="Default Summary"
            value={character.appearance.defaultSummary}
            multiline
          />
        </div>
      </Section>

      <Section title="Distinguishing Marks">
        <ul className="space-y-2">
          {character.distinguishingMarks.map((mark) => (
            <li
              key={mark.id}
              className="flex items-center justify-between gap-2 bg-gray-50 rounded-lg px-3 py-2"
            >
              <span className="text-sm text-gray-800">{mark.content}</span>
              {isEditing && (
                <button
                  type="button"
                  className="text-red-500 text-xs hover:underline shrink-0"
                >
                  Remove
                </button>
              )}
            </li>
          ))}
        </ul>
        {isEditing && (
          <button
            type="button"
            className="mt-3 text-sm text-indigo-600 hover:underline"
          >
            + Add Mark
          </button>
        )}
      </Section>

      <Section title="Relationships">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {character.relationships.map((rel) => (
            <RelationshipCard key={rel.id} relationship={rel} />
          ))}
        </div>
      </Section>
    </div>
  );
}
