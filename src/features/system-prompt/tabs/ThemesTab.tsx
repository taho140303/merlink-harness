import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import { useSystemPromptCatalog } from '../catalog';
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
  WITNESS: 'bg-surface-muted text-fg-muted  ',
};

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

export function ThemesTab() {
  const { themeArguments, themes } = useSystemPromptCatalog();
  const t = useT();

  const argsForTheme = (themeId: string) =>
    themeArguments.filter((a) => a.themeId === themeId);

  const themeById = new Map(themes.map((th) => [th.id, th]));

  return (
    <div className="space-y-6">
      <SectionTitle title={t('sp.themes.title')} description={t('sp.themes.desc')} />
      {themes.length === 0 && <p className="rounded-xl border border-dashed border-line p-8 text-center text-sm text-fg-muted">Chưa có chủ đề trong cơ sở dữ liệu.</p>}

      {themes.map((th) => {
        const args = argsForTheme(th.id);
        const parent = th.parentThemeId ? themeById.get(th.parentThemeId) : null;

        return (
          <div
            key={th.id}
            className="tier-card rounded-card p-5"
          >
            <div className="flex items-start justify-between gap-3">
              <div className="min-w-0">
                <p className="font-mono text-xs text-fg-muted">
                  {th.code}
                </p>
                <h4 className="mt-1 font-serif text-base font-semibold leading-snug text-fg">
                  {th.question}
                </h4>
              </div>
              <div className="flex flex-wrap items-center gap-1.5">
                <span className="rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                  {th.scope}
                </span>
                <span
                  className={cn('rounded-full px-2 py-0.5 text-xs font-medium', RES_COLOR[th.resolutionType])}
                >
                  {humanize(th.resolutionType)}
                </span>
              </div>
            </div>

            {parent && (
              <p className="mt-2 text-xs text-fg-muted">
                ↳ {parent.code}
              </p>
            )}

            <div className="mt-3 grid gap-3 md:grid-cols-2">
              <div>
                <p className="text-xs uppercase tracking-wide text-fg-muted">
                  {t('sp.themes.resolution')}
                </p>
                <p className="mt-1 whitespace-pre-wrap font-serif text-xs leading-relaxed text-fg">
                  {th.resolution}
                </p>
              </div>
              {th.message && (
                <div className="rounded-lg border border-indigo-100 bg-indigo-50 p-3 dark:border-indigo-900/60 dark:bg-indigo-950/40">
                  <p className="text-xs uppercase tracking-wide text-indigo-700 dark:text-indigo-300">
                    {t('sp.themes.message')}
                  </p>
                  <p className="mt-1 whitespace-pre-wrap font-serif text-xs leading-relaxed text-indigo-900 dark:text-indigo-100">
                    {th.message}
                  </p>
                </div>
              )}
            </div>

            {args.length > 0 && (
              <div className="mt-4 border-t border-line pt-3 ">
                <p className="mb-2 text-xs uppercase tracking-wide text-fg-muted">
                  {t('sp.themes.arguments')}
                </p>
                <div className="space-y-1.5">
                  {args.map((a) => (
                    <div
                      key={a.id}
                      className="rounded-lg border border-line bg-surface-muted px-3 py-2  "
                    >
                      <div className="flex flex-wrap items-center gap-1.5">
                        <span className="text-xs font-semibold text-fg">
                          {a.characterName}
                        </span>
                        <span
                          className={cn(
                            'rounded-full px-2 py-0.5 text-xs font-medium',
                            ROLE_COLOR[a.role],
                          )}
                        >
                          {humanize(a.role)}
                        </span>
                        <span className="rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                          {t('sp.themes.answerMode')}: {humanize(a.answerMode)}
                        </span>
                      </div>
                      <p className="mt-1 text-xs text-fg-muted">{a.note}</p>
                      {(a.requiredKnowledge || a.requiredCompetence) && (
                        <div className="mt-1.5 flex flex-wrap gap-1.5">
                          {a.requiredKnowledge && (
                            <span className="rounded-full bg-sky-100 px-2 py-0.5 text-xs text-sky-700 dark:bg-sky-950/60 dark:text-sky-300">
                              {t('sp.themes.requiredKnowledge')}: {a.requiredKnowledge}
                            </span>
                          )}
                          {a.requiredCompetence && (
                            <span className="rounded-full bg-emerald-100 px-2 py-0.5 text-xs text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
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
