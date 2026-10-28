import { useState } from 'react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import { narrativeDirectives, narratorConfigs } from '../mockData';
import type {
  DirectiveCategory,
  GrammaticalPerson,
  PovScope,
} from '../types';

const PERSON_LABEL: Record<GrammaticalPerson, string> = {
  FIRST_PERSON: 'First-person',
  SECOND_PERSON: 'Second-person',
  THIRD_PERSON: 'Third-person',
};

const SCOPE_LABEL: Record<PovScope, string> = {
  SINGLE_POV: 'Single POV',
  MULTIPLE_POV: 'Multiple POV',
  SHIFTING_OMNISCIENT: 'Shifting omniscient',
};

const PERSON_DESC: Record<GrammaticalPerson, string> = {
  FIRST_PERSON: 'Narrator is a direct participant or witness, using I/me.',
  SECOND_PERSON: 'Narrator addresses the reader or a character as "you".',
  THIRD_PERSON: 'Narrator stands outside the story, using he/she/they.',
};

const SCOPE_DESC: Record<PovScope, string> = {
  SINGLE_POV: 'One fixed viewpoint throughout the story.',
  MULTIPLE_POV: 'Rotate viewpoints between characters across chapters or scenes.',
  SHIFTING_OMNISCIENT: 'Omniscient narrator freely moves attention between characters.',
};

const CATEGORY_COLOR: Record<DirectiveCategory, string> = {
  PROSE_PRINCIPLE: 'bg-indigo-100 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300',
  RESTRAINT: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  INFORMATION_CONTROL: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
};

type PersonFilter = 'ALL' | GrammaticalPerson;

const PERSON_FILTERS: PersonFilter[] = [
  'ALL',
  'FIRST_PERSON',
  'SECOND_PERSON',
  'THIRD_PERSON',
];

function SectionTitle({ title, description }: { title: string; description: string }) {
  return (
    <div className="mb-3 flex items-start gap-2">
      <div>
        <h3 className="font-semibold text-fg">{title}</h3>
        <p className="mt-0.5 text-xs text-fg-muted">{description}</p>
      </div>
      <InfoTooltip content={description} className="mt-1" />
    </div>
  );
}

export function NarrativeDirectivesTab() {
  const t = useT();
  const [personFilter, setPersonFilter] = useState<PersonFilter>('ALL');

  const filteredConfigs =
    personFilter === 'ALL'
      ? narratorConfigs
      : narratorConfigs.filter((nc) => nc.person === personFilter);

  const byCategory = new Map<DirectiveCategory, typeof narrativeDirectives>();
  for (const nd of narrativeDirectives) {
    if (!byCategory.has(nd.category)) byCategory.set(nd.category, []);
    byCategory.get(nd.category)!.push(nd);
  }

  const filterLabel = (f: PersonFilter) =>
    f === 'ALL' ? t('sp.narrator.filter.all') : t(`sp.narrator.filter.${f}` as const);

  return (
    <div className="space-y-8">
      {/* Narrator configs */}
      <section>
        <div className="mb-3 flex flex-wrap items-start justify-between gap-3">
          <SectionTitle title={t('sp.narrator.title')} description={t('sp.narrator.desc')} />

          {/* Person filter */}
          <div className="flex items-center gap-1 rounded-xl bg-surface-muted p-0.5 ">
            {PERSON_FILTERS.map((f) => {
              const count =
                f === 'ALL'
                  ? narratorConfigs.length
                  : narratorConfigs.filter((nc) => nc.person === f).length;
              return (
                <button
                  key={f}
                  type="button"
                  onClick={() => setPersonFilter(f)}
                  className={cn(
                    'flex items-center gap-1 rounded-lg px-2.5 py-1.5 text-xs font-medium transition-colors',
                    personFilter === f
                      ? 'bg-surface-card text-fg   '
                      : 'text-fg-muted hover:text-fg -subtle ',
                  )}
                >
                  <span>{filterLabel(f)}</span>
                  <span
                    className={cn(
                      'rounded-full px-1.5 py-0.5 text-xs font-semibold',
                      personFilter === f
                        ? 'bg-accent text-white  '
                        : 'bg-surface-muted text-fg-muted  ',
                    )}
                  >
                    {count}
                  </span>
                </button>
              );
            })}
          </div>
        </div>

        {filteredConfigs.length === 0 ? (
          <div className="rounded-xl border border-dashed border-line-strong bg-surface-card/60 p-10 text-center text-xs text-fg-subtle -strong ">
            {t('sp.narrator.empty')}
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-3 md:grid-cols-2 lg:grid-cols-3">
            {filteredConfigs.map((nc) => (
              <div
                key={nc.id}
                className="tier-card rounded-card p-4"
              >
                <div className="flex items-start justify-between gap-2">
                  <div className="min-w-0">
                    <h4 className="font-semibold text-fg">
                      {nc.name}
                    </h4>
                    <p className="mt-0.5 truncate font-mono text-xs text-fg-muted">
                      {nc.code}
                    </p>
                  </div>
                  <InfoTooltip content={nc.description} className="mt-0.5 shrink-0" />
                </div>

                <p className="mt-2 text-xs leading-relaxed text-fg-muted">
                  {nc.description}
                </p>

                <div className="mt-3 space-y-1.5 border-t border-line pt-2.5 text-xs ">
                  <div className="flex items-center gap-1.5">
                    <span className="text-fg-subtle">{t('sp.narrator.person')}:</span>
                    <span className="font-medium text-fg">
                      {PERSON_LABEL[nc.person]}
                    </span>
                    <InfoTooltip content={PERSON_DESC[nc.person]} />
                  </div>
                  <div className="flex items-center gap-1.5">
                    <span className="text-fg-subtle">{t('sp.narrator.personSubtype')}:</span>
                    <span className="font-medium text-fg">
                      {humanize(nc.personSubtype)}
                    </span>
                  </div>
                  <div className="flex items-center gap-1.5">
                    <span className="text-fg-subtle">{t('sp.narrator.povScope')}:</span>
                    <span className="font-medium text-fg">
                      {SCOPE_LABEL[nc.povScope]}
                    </span>
                    <InfoTooltip content={SCOPE_DESC[nc.povScope]} />
                  </div>
                  <div className="flex flex-wrap items-center gap-1.5 pt-0.5">
                    <span
                      className={cn(
                        'rounded-full px-2 py-0.5 text-xs font-medium',
                        nc.isReliable
                          ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300'
                          : 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
                      )}
                    >
                      {t('sp.narrator.reliable')}: {nc.isReliable ? t('sp.narrator.yes') : t('sp.narrator.no')}
                    </span>
                    <span
                      className={cn(
                        'rounded-full px-2 py-0.5 text-xs font-medium',
                        nc.breaksFourthWall
                          ? 'bg-violet-100 text-violet-700 dark:bg-violet-950/60 dark:text-violet-300'
                          : 'bg-surface-muted text-fg-muted  ',
                      )}
                    >
                      {t('sp.narrator.breaksFourthWall')}: {nc.breaksFourthWall ? t('sp.narrator.yes') : t('sp.narrator.no')}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </section>

      {/* Narrative directives grouped by category */}
      <section>
        <SectionTitle title={t('sp.directives.title')} description={t('sp.directives.desc')} />
        <div className="space-y-5">
          {Array.from(byCategory.entries()).map(([category, items]) => (
            <div key={category}>
              <div className="mb-2 flex items-center gap-2">
                <span
                  className={cn(
                    'rounded-full px-2.5 py-0.5 text-xs font-medium',
                    CATEGORY_COLOR[category],
                  )}
                >
                  {humanize(category)}
                </span>
              </div>
              <ul className="space-y-1.5">
                {items.map((nd) => (
                  <li
                    key={nd.id}
                    className="tier-card rounded-lg p-2.5"
                  >
                    <p className="font-mono text-xs font-semibold text-fg">
                      {nd.code}
                    </p>
                    <p className="mt-0.5 text-xs text-fg-muted">
                      {nd.description}
                    </p>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
