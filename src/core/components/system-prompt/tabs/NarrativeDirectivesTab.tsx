import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import { narrativeDirectives, narratorConfigs } from '../mockData';
import type { DirectiveCategory, GrammaticalPerson, PovScope } from '../types';

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

function SectionTitle({ title, description }: { title: string; description: string }) {
  return (
    <div className="mb-3 flex items-start gap-2">
      <div>
        <h3 className="font-semibold text-slate-900 dark:text-slate-100">{title}</h3>
        <p className="mt-0.5 text-xs text-slate-500 dark:text-slate-400">{description}</p>
      </div>
      <InfoTooltip content={description} className="mt-1" />
    </div>
  );
}

export function NarrativeDirectivesTab() {
  const t = useT();

  const byCategory = new Map<DirectiveCategory, typeof narrativeDirectives>();
  for (const nd of narrativeDirectives) {
    if (!byCategory.has(nd.category)) byCategory.set(nd.category, []);
    byCategory.get(nd.category)!.push(nd);
  }

  return (
    <div className="space-y-8">
      {/* Narrator configs */}
      <section>
        <SectionTitle title={t('sp.narrator.title')} description={t('sp.narrator.desc')} />
        <div className="grid grid-cols-1 gap-3 md:grid-cols-2 lg:grid-cols-3">
          {narratorConfigs.map((nc) => (
            <div
              key={nc.id}
              className="rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900"
            >
              <div className="flex items-start justify-between gap-2">
                <div className="min-w-0">
                  <h4 className="font-semibold text-slate-900 dark:text-slate-100">{nc.name}</h4>
                  <p className="mt-0.5 truncate font-mono text-[11px] text-slate-500 dark:text-slate-400">
                    {nc.code}
                  </p>
                </div>
                <InfoTooltip content={nc.description} className="mt-0.5 shrink-0" />
              </div>

              <p className="mt-2 text-[11px] leading-relaxed text-slate-600 dark:text-slate-400">
                {nc.description}
              </p>

              <div className="mt-3 space-y-1.5 border-t border-slate-100 pt-2.5 text-[11px] dark:border-slate-800">
                <div className="flex items-center gap-1.5">
                  <span className="text-slate-400">{t('sp.narrator.person')}:</span>
                  <span className="font-medium text-slate-800 dark:text-slate-200">
                    {PERSON_LABEL[nc.person]}
                  </span>
                  <InfoTooltip content={PERSON_DESC[nc.person]} />
                </div>
                <div className="flex items-center gap-1.5">
                  <span className="text-slate-400">{t('sp.narrator.personSubtype')}:</span>
                  <span className="font-medium text-slate-800 dark:text-slate-200">
                    {humanize(nc.personSubtype)}
                  </span>
                </div>
                <div className="flex items-center gap-1.5">
                  <span className="text-slate-400">{t('sp.narrator.povScope')}:</span>
                  <span className="font-medium text-slate-800 dark:text-slate-200">
                    {SCOPE_LABEL[nc.povScope]}
                  </span>
                  <InfoTooltip content={SCOPE_DESC[nc.povScope]} />
                </div>
                <div className="flex flex-wrap items-center gap-1.5">
                  <span
                    className={cn(
                      'rounded-full px-2 py-0.5 text-[10px] font-medium',
                      nc.isReliable
                        ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300'
                        : 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
                    )}
                  >
                    {t('sp.narrator.reliable')}: {nc.isReliable ? t('sp.narrator.yes') : t('sp.narrator.no')}
                  </span>
                  <span
                    className={cn(
                      'rounded-full px-2 py-0.5 text-[10px] font-medium',
                      nc.breaksFourthWall
                        ? 'bg-violet-100 text-violet-700 dark:bg-violet-950/60 dark:text-violet-300'
                        : 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-300',
                    )}
                  >
                    {t('sp.narrator.breaksFourthWall')}: {nc.breaksFourthWall ? t('sp.narrator.yes') : t('sp.narrator.no')}
                  </span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* Narrative directives grouped by category */}
      <section>
        <SectionTitle title={t('sp.directives.title')} description={t('sp.directives.desc')} />
        <div className="space-y-5">
          {Array.from(byCategory.entries()).map(([category, items]) => (
            <div key={category}>
              <div className="mb-2 flex items-center gap-2">
                <span
                  className={cn('rounded-full px-2.5 py-0.5 text-[10px] font-medium', CATEGORY_COLOR[category])}
                >
                  {humanize(category)}
                </span>
              </div>
              <ul className="space-y-1.5">
                {items.map((nd) => (
                  <li
                    key={nd.id}
                    className="rounded-lg border border-slate-200 bg-white p-2.5 dark:border-slate-800 dark:bg-slate-900"
                  >
                    <p className="font-mono text-[11px] font-semibold text-slate-800 dark:text-slate-200">
                      {nd.code}
                    </p>
                    <p className="mt-0.5 text-[11px] text-slate-600 dark:text-slate-400">
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
