import { useT } from '../../../../shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { genres, motifApplicableGenres, motifs } from '../mockData';

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

export function GenresTab() {
  const t = useT();


  return (
    <div className="space-y-8">
      <section>
        <SectionTitle title={t('sp.genres.title')} description={t('sp.genres.desc')} />
        <div className="grid grid-cols-1 gap-3 md:grid-cols-2 lg:grid-cols-3">
          {genres.map((g) => {
            const count = motifApplicableGenres.filter((mg) => mg.genreId === g.id).length;
            return (
              <div
                key={g.id}
                className="tier-card rounded-card p-4"
              >
                <div className="flex items-start justify-between gap-2">
                  <div>
                    <h4 className="font-semibold text-fg">{g.name}</h4>
                    <p className="mt-0.5 font-mono text-xs text-fg-muted">
                      {g.code}
                    </p>
                  </div>
                  <span className="shrink-0 rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                    {count} {t('sp.genres.motifsCount')}
                  </span>
                </div>
                <p className="mt-2 text-xs text-fg-muted">
                  {g.description}
                </p>
              </div>
            );
          })}
        </div>
      </section>

      <section>
        <SectionTitle
          title={t('sp.genres.applicable')}
          description={t('sp.genres.desc')}
        />
        <div className="tier-card rounded-card">
          <div className="grid grid-cols-[1fr_auto] gap-2 border-b border-line px-4 py-2 text-xs font-semibold uppercase tracking-wide text-fg-muted  -subtle">
            <span>{t('sp.common.name')}</span>
            <span>Genres</span>
          </div>
          <ul className="divide-y divide-line ">
            {motifs.map((mo) => {
              const gs = motifApplicableGenres
                .filter((mg) => mg.motifId === mo.id)
                .map((mg) => genres.find((g) => g.id === mg.genreId))
                .filter((g): g is (typeof genres)[number] => Boolean(g));
              return (
                <li key={mo.id} className="grid grid-cols-[1fr_auto] gap-2 px-4 py-2.5">
                  <div>
                    <p className="text-xs font-semibold text-fg">
                      {mo.name}
                    </p>
                    <p className="font-mono text-xs text-fg-muted">
                      {mo.code}
                    </p>
                  </div>
                  <div className="flex flex-wrap items-center justify-end gap-1">
                    {gs.length === 0 && (
                      <span className="text-xs text-fg-subtle">—</span>
                    )}
                    {gs.map((g) => (
                      <span
                        key={g.id}
                        className="rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700 dark:bg-blue-950/60 dark:text-blue-300"
                      >
                        {g.name}
                      </span>
                    ))}
                  </div>
                </li>
              );
            })}
          </ul>
        </div>
      </section>
    </div>
  );
}
