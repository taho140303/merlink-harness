import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import { themeArguments, themes } from '../mockData';
import type { ThemeArgumentRole, ThemeResolutionType } from '../types';

const RES_COLOR: Record<ThemeResolutionType, string> = {
  RESOLVED: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
  OPEN: 'bg-sky-100 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300',
  AMBIGUOUS: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
};

const ROLE_COLOR: Record<ThemeArgumentRole, string> = {
  PRIMARY: 'bg-indigo-100 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300',
  ANTITHESIS: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  CAUTIONARY: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  WITNESS: 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-300',
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

export function ThemesTab() {
  const t = useT();

  const argsForTheme = (themeId: string) =>
    themeArguments.filter((a) => a.themeId === themeId);

  const themeById = new Map(themes.map((th) => [th.id, th]));

  return (
    <div className="space-y-6">
      <SectionTitle title={t('sp.themes.title')} description={t('sp.themes.desc')} />

      {themes.map((th) => {
        const args = argsForTheme(th.id);
        const parent = th.parentThemeId ? themeById.get(th.parentThemeId) : null;

        return (
          <div
            key={th.id}
            className="rounded-xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-800 dark:bg-slate-900"
          >
            <div className="flex items-start justify-between gap-3">
              <div className="min-w-0">
                <p className="font-mono text-[11px] text-slate-500 dark:text-slate-400">
                  {th.code}
                </p>
                <h4 className="mt-1 font-serif text-base font-semibold leading-snug text-slate-900 dark:text-slate-100">
                  {th.question}
                </h4>
              </div>
              <div className="flex flex-wrap items-center gap-1.5">
                <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                  {th.scope}
                </span>
                <span
                  className={cn('rounded-full px-2 py-0.5 text-[10px] font-medium', RES_COLOR[th.resolutionType])}
                >
                  {humanize(th.resolutionType)}
                </span>
              </div>
            </div>

            {parent && (
              <p className="mt-2 text-[11px] text-slate-500 dark:text-slate-400">
                ↳ {parent.code}
              </p>
            )}

            <div className="mt-3 grid gap-3 md:grid-cols-2">
              <div>
                <p className="text-[10px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
                  {t('sp.themes.resolution')}
                </p>
                <p className="mt-1 whitespace-pre-wrap font-serif text-[13px] leading-relaxed text-slate-800 dark:text-slate-200">
                  {th.resolution}
                </p>
              </div>
              {th.message && (
                <div className="rounded-lg border border-indigo-100 bg-indigo-50 p-3 dark:border-indigo-900/60 dark:bg-indigo-950/40">
                  <p className="text-[10px] uppercase tracking-wide text-indigo-700 dark:text-indigo-300">
                    {t('sp.themes.message')}
                  </p>
                  <p className="mt-1 whitespace-pre-wrap font-serif text-[13px] leading-relaxed text-indigo-900 dark:text-indigo-100">
                    {th.message}
                  </p>
                </div>
              )}
            </div>

            {args.length > 0 && (
              <div className="mt-4 border-t border-slate-100 pt-3 dark:border-slate-800">
                <p className="mb-2 text-[10px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
                  {t('sp.themes.arguments')}
                </p>
                <div className="space-y-1.5">
                  {args.map((a) => (
                    <div
                      key={a.id}
                      className="rounded-lg border border-slate-200 bg-slate-50/70 px-3 py-2 dark:border-slate-800 dark:bg-slate-950/40"
                    >
                      <div className="flex flex-wrap items-center gap-1.5">
                        <span className="text-[11px] font-semibold text-slate-800 dark:text-slate-200">
                          {a.characterName}
                        </span>
                        <span
                          className={cn(
                            'rounded-full px-2 py-0.5 text-[10px] font-medium',
                            ROLE_COLOR[a.role],
                          )}
                        >
                          {humanize(a.role)}
                        </span>
                        <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                          {t('sp.themes.answerMode')}: {humanize(a.answerMode)}
                        </span>
                      </div>
                      <p className="mt-1 text-[11px] text-slate-600 dark:text-slate-400">{a.note}</p>
                      {(a.requiredKnowledge || a.requiredCompetence) && (
                        <div className="mt-1.5 flex flex-wrap gap-1.5">
                          {a.requiredKnowledge && (
                            <span className="rounded-full bg-sky-100 px-2 py-0.5 text-[10px] text-sky-700 dark:bg-sky-950/60 dark:text-sky-300">
                              {t('sp.themes.requiredKnowledge')}: {a.requiredKnowledge}
                            </span>
                          )}
                          {a.requiredCompetence && (
                            <span className="rounded-full bg-emerald-100 px-2 py-0.5 text-[10px] text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
                              {t('sp.themes.requiredCompetence')}: {a.requiredCompetence}
                            </span>
                          )}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
