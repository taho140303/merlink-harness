import { useState, type ReactNode } from 'react';
import { Plus, Trash2 } from 'lucide-react';
import { useCharacterDetailStore } from '../store';
import { FieldDisplay } from '../shared/FieldDisplay';
import { EditableField } from '../shared/EditableField';
import { EnumSelect } from '../shared/EnumSelect';
import { RelationshipCard } from '../RelationshipCard';
import { useT } from '../../../../shared/i18n/useT';
import type { BiologicalSex, Gender } from '../types';
import { BIOLOGICAL_SEX_OPTIONS, GENDER_OPTIONS } from '../types';

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
  const t = useT();
  const character = useCharacterDetailStore((s) => s.character);
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const updateBasicInfo = useCharacterDetailStore((s) => s.updateBasicInfo);
  const updateAppearance = useCharacterDetailStore((s) => s.updateAppearance);
  const addMark = useCharacterDetailStore((s) => s.addMark);
  const removeMark = useCharacterDetailStore((s) => s.removeMark);
  const patch = useCharacterDetailStore((s) => s.patchCharacter);

  return (
    <div className="space-y-4">
      <Section title={t('detail.core.basicInfo')}>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <EditableField
            label={t('detail.core.name')}
            value={character.basicInfo.name}
            onChange={(v) => updateBasicInfo({ name: v })}
          />
          <EditableField
            label={t('detail.core.species')}
            value={character.basicInfo.species}
            onChange={(v) => updateBasicInfo({ species: v })}
          />
          <EnumSelect
            label={t('detail.core.biologicalSex')}
            value={character.basicInfo.biologicalSex}
            options={BIOLOGICAL_SEX_OPTIONS}
            onChange={isEditing ? (v) => updateBasicInfo({ biologicalSex: v as BiologicalSex }) : undefined}
          />
          <EnumSelect
            label={t('detail.core.genderIdentity')}
            value={character.basicInfo.genderIdentity}
            options={GENDER_OPTIONS}
            onChange={isEditing ? (v) => updateBasicInfo({ genderIdentity: v as Gender }) : undefined}
          />
          <EnumSelect
            label={t('detail.core.genderExpression')}
            value={character.basicInfo.genderExpression}
            options={GENDER_OPTIONS}
            onChange={isEditing ? (v) => updateBasicInfo({ genderExpression: v as Gender }) : undefined}
            className="sm:col-span-2 sm:max-w-xs"
          />
        </div>
      </Section>

      <Section title={t('detail.core.appearance')}>
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <EditableField
              label={t('detail.core.heightCm')}
              value={String(character.appearance.height)}
              onChange={(v) => updateAppearance({ height: Number(v) || 0 })}
            />
            <EditableField
              label={t('detail.core.weightKg')}
              value={String(character.appearance.weight)}
              onChange={(v) => updateAppearance({ weight: Number(v) || 0 })}
            />
          </div>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
            <FieldDisplay
              label={t('detail.core.hair')}
              value={character.appearance.hair.value}
              colorHex={character.appearance.hair.hex}
            />
            <FieldDisplay
              label={t('detail.core.eyes')}
              value={character.appearance.eyes.value}
              colorHex={character.appearance.eyes.hex}
            />
            <FieldDisplay
              label={t('detail.core.skin')}
              value={character.appearance.skin.value}
              colorHex={character.appearance.skin.hex}
            />
          </div>
          <EditableField
            label={t('detail.core.hairNotes')}
            value={character.appearance.hairNote}
            onChange={(v) => updateAppearance({ hairNote: v })}
            multiline
          />
          <EditableField
            label={t('detail.core.eyesNotes')}
            value={character.appearance.eyesNote}
            onChange={(v) => updateAppearance({ eyesNote: v })}
            multiline
          />
          <EditableField
            label={t('detail.core.skinNotes')}
            value={character.appearance.skinNote}
            onChange={(v) => updateAppearance({ skinNote: v })}
            multiline
          />
          <EditableField
            label={t('detail.core.outfitStyle')}
            value={character.appearance.outfitStyle}
            onChange={(v) => updateAppearance({ outfitStyle: v })}
            multiline
          />
          <EditableField
            label={t('detail.core.defaultSummary')}
            value={character.appearance.defaultSummary}
            onChange={(v) => updateAppearance({ defaultSummary: v })}
            multiline
          />
        </div>
      </Section>

      <Section title={t('detail.core.marks')}>
        <ul className="space-y-2">
          {character.distinguishingMarks.map((mark) => (
            <li
              key={mark.id}
              className="flex items-center justify-between gap-2 rounded-lg bg-slate-50 px-3 py-2 dark:bg-slate-800/60"
            >
              {isEditing ? (
                <input
                  value={mark.content}
                  onChange={(e) =>
                    patch((c) => {
                      const m = c.distinguishingMarks.find((x) => x.id === mark.id);
                      if (m) m.content = e.target.value;
                    })
                  }
                  className="flex-1 rounded-md border border-slate-200 bg-white px-2 py-1 text-sm dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                />
              ) : (
                <span className="text-sm text-slate-800 dark:text-slate-200">{mark.content}</span>
              )}
              {isEditing && (
                <button
                  type="button"
                  onClick={() => removeMark(mark.id)}
                  className="shrink-0 text-rose-500 hover:text-rose-600"
                  title={t('common.remove')}
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
            onClick={() => addMark(t('detail.core.newMark'))}
            className="mt-3 flex items-center gap-1.5 text-sm text-indigo-600 hover:underline dark:text-indigo-400"
          >
            <Plus className="h-3.5 w-3.5" />
            {t('detail.core.addMark')}
          </button>
        )}
      </Section>

      <Section title={t('detail.core.relationships')}>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          {character.relationships.map((rel) => (
            <RelationshipCard key={rel.id} relationship={rel} />
          ))}
        </div>
      </Section>
    </div>
  );
}
