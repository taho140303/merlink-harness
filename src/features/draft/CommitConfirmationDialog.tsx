import { useEffect, useMemo, useRef, useState } from 'react';
import { ChevronDown, FlaskConical, Loader2 } from 'lucide-react';
import { Dialog, DialogContent, DialogDescription, DialogTitle } from '@/shared/components/ui/dialog';
import { Button } from '@/shared/components/ui/button';
import { Badge, type BadgeVariant } from '@/shared/components/ui/badge';
import { Select } from '@/shared/components/ui/select';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import type { TKey } from '@/shared/i18n/dict';
import {
  canDismissCommitDialog,
  derivePendingPatches,
  deriveValidation,
  getCommitBlockReasons,
  reduceCommitOp,
  summarizePlan,
  type CommitBlockReason,
  type CommitOpState,
  type CommitReadinessInput,
} from './commitPreview';
import type { DraftOverview } from './types';

interface CommitConfirmationDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  overview: DraftOverview;
  /** Readiness from a future backend. Undefined today → validation/patches are "unknown". */
  readiness?: CommitReadinessInput;
}

const SIMULATED_LATENCY_MS = 1200;
const SHOW_DEV_SIM = import.meta.env.DEV;

type Fixture = 'none' | 'ready' | 'blocked' | 'patches';
const FIXTURES: Record<Fixture, CommitReadinessInput | undefined> = {
  none: undefined,
  ready: { validation: { ready: true }, pendingPatches: { count: 0 } },
  blocked: { validation: { ready: false, blockingReasons: [] }, pendingPatches: { count: 0 } },
  patches: { validation: { ready: true }, pendingPatches: { count: 3 } },
};

const BLOCK_KEYS: Record<CommitBlockReason, TKey> = {
  validationBlocked: 'commit.validation.blockedDesc',
  validationUnknown: 'commit.validation.unknownDesc',
  patchesPending: 'commit.patches.pendingDesc',
  patchesUnknown: 'commit.patches.unknownDesc',
  operationPending: 'commit.block.operationPending',
};

function Stat({ label, value, muted }: { label: string; value: string; muted?: boolean }) {
  return (
    <div className="min-w-0 rounded-xl bg-surface-muted/60 px-3 py-2">
      <dt className="text-sm text-fg-subtle">{label}</dt>
      <dd className={cn('mt-0.5 truncate text-base font-semibold', muted ? 'text-fg-muted' : 'text-fg')}>{value}</dd>
    </div>
  );
}

const VALIDATION_BADGE: Record<'ready' | 'blocked' | 'unknown', BadgeVariant> = {
  ready: 'success',
  blocked: 'danger',
  unknown: 'warning',
};

function SectionTitle({ children }: { children: string }) {
  return <h3 className="text-sm font-semibold uppercase tracking-wide text-fg-subtle">{children}</h3>;
}

export function CommitConfirmationDialog({ open, onOpenChange, overview, readiness }: CommitConfirmationDialogProps) {
  const t = useT();
  const [op, setOp] = useState<CommitOpState>({ status: 'idle' });
  const busyRef = useRef(false); // synchronous duplicate-submit guard
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [fixture, setFixture] = useState<Fixture>('none');
  const [simOpen, setSimOpen] = useState(false);
  const simActive = SHOW_DEV_SIM && fixture !== 'none';
  const [outcome, setOutcome] = useState<'success' | 'error'>('success');

  useEffect(() => () => { if (timer.current) clearTimeout(timer.current); }, []);

  // Summary is derived from the current draft on every render (reopen-safe).
  const plan = useMemo(() => summarizePlan(overview), [overview]);
  const effective = SHOW_DEV_SIM && fixture !== 'none' ? FIXTURES[fixture] : readiness;
  const validation = deriveValidation(effective);
  const patches = derivePendingPatches(effective);
  const pending = op.status === 'pending';
  const blockReasons = getCommitBlockReasons(validation, patches, pending);
  const eligible = blockReasons.length === 0;


  const handleOpenChange = (next: boolean) => {
    if (!next && !canDismissCommitDialog(op)) return;
    if (!next) setOp((s) => reduceCommitOp(s, 'reset'));
    onOpenChange(next);
  };

  const handleConfirm = () => {
    if (!eligible || busyRef.current) return;
    busyRef.current = true;
    setOp((s) => reduceCommitOp(s, 'submit'));
    const willFail = SHOW_DEV_SIM && outcome === 'error';
    timer.current = setTimeout(() => {
      busyRef.current = false;
      setOp((s) => reduceCommitOp(s, willFail ? 'fail' : 'succeed'));
    }, SIMULATED_LATENCY_MS);
  };

  const validationDesc: TKey =
    validation.status === 'ready' ? 'commit.validation.readyDesc'
    : validation.status === 'blocked' ? 'commit.validation.blockedDesc'
    : 'commit.validation.unknownDesc';
  const patchValue =
    patches.status === 'unknown' ? t('commit.patches.unknown')
    : patches.count === 0 ? t('commit.patches.zero')
    : t('commit.patches.known', { n: patches.count });

  const unavailable = t('commit.unavailable');
  const patchVariant: BadgeVariant = patches.status === 'unknown' ? 'warning' : patches.count === 0 ? 'success' : 'danger';

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent
        className="flex max-h-[85vh] max-w-[44rem] flex-col gap-0 overflow-hidden p-0"
        onEscapeKeyDown={(e) => { if (pending) e.preventDefault(); }}
        onPointerDownOutside={(e) => { if (pending) e.preventDefault(); }}
        onInteractOutside={(e) => { if (pending) e.preventDefault(); }}
      >
        <div className="shrink-0 border-b border-line px-6 py-5 pr-14">
          <DialogTitle className="text-lg">{t('commit.title')}</DialogTitle>
          <DialogDescription className="mt-1 text-sm">{t('commit.desc')}</DialogDescription>
        </div>

        <div className="min-h-0 flex-1 space-y-4 overflow-y-auto px-6 py-5">
          {simActive ? (
            <div className="flex items-start gap-2 rounded-xl bg-amber-100 px-3 py-2 text-sm font-medium text-amber-700 dark:bg-amber-950/50 dark:text-amber-300" role="alert">
              <FlaskConical className="mt-0.5 h-4 w-4 shrink-0" aria-hidden="true" />
              {t('commit.sim.banner')}
            </div>
          ) : (
            <div className="flex items-start gap-2 text-sm text-fg-muted">
              <Badge variant="neutral" className="shrink-0 text-sm">{t('commit.previewBadge')}</Badge>
              <span>{t('commit.backendUnavailable')}</span>
            </div>
          )}

          <section aria-label={t('commit.planSummary')} className="tier-card space-y-3 rounded-card p-4">
            <SectionTitle>{t('commit.planSummary')}</SectionTitle>
            <div className="min-w-0">
              <p className="break-words text-base font-semibold text-fg">{plan.projectName ?? unavailable}</p>
              {plan.description && <p className="mt-1 line-clamp-3 text-sm text-fg-muted">{plan.description}</p>}
            </div>
            <dl className="grid grid-cols-2 gap-3 sm:grid-cols-3">
              <Stat label={t('commit.arcs')} value={plan.arcs === null ? unavailable : String(plan.arcs)} muted={plan.arcs === null} />
              <Stat label={t('commit.chapters')} value={plan.chapters === null ? unavailable : String(plan.chapters)} muted={plan.chapters === null} />
              <Stat label={t('commit.scenes')} value={plan.scenes === null ? unavailable : String(plan.scenes)} muted={plan.scenes === null} />
              <Stat label={t('commit.characters')} value={plan.characters === null ? unavailable : String(plan.characters)} muted={plan.characters === null} />
              <Stat label={t('commit.worldbuilding')} value={plan.worldbuildingAvailable ? t('commit.available') : unavailable} muted={!plan.worldbuildingAvailable} />
              <Stat label={t('commit.relationships')} value={plan.relationships === null ? t('commit.notEvaluated') : String(plan.relationships)} muted={plan.relationships === null} />
            </dl>
          </section>

          <section className="tier-card space-y-2 rounded-card p-4">
            <div className="flex items-center justify-between gap-2">
              <SectionTitle>{t('commit.validation')}</SectionTitle>
              <Badge variant={VALIDATION_BADGE[validation.status]} className="text-sm">{t(`commit.validation.${validation.status}` as TKey)}</Badge>
            </div>
            <p className="text-sm text-fg-muted">{t(validationDesc)}</p>
          </section>

          <section className="tier-card space-y-2 rounded-card p-4">
            <div className="flex items-center justify-between gap-2">
              <SectionTitle>{t('commit.patches')}</SectionTitle>
              <Badge variant={patchVariant} className="text-sm">{patchValue}</Badge>
            </div>
            {patches.status === 'unknown' && <p className="text-sm text-fg-muted">{t('commit.patches.unknownDesc')}</p>}
            {patches.status === 'known' && patches.count > 0 && <p className="text-sm text-fg-muted">{t('commit.patches.pendingDesc')}</p>}
          </section>

          {!eligible && !pending && op.status === 'idle' && (
            <ul className="list-disc space-y-0.5 pl-4 text-sm text-fg-muted" role="status">
              {blockReasons.map((r) => <li key={r}>{t(BLOCK_KEYS[r])}</li>)}
            </ul>
          )}

          {pending && (
            <p className="flex items-center gap-2 text-sm text-fg-muted" role="status">
              <Loader2 className="h-4 w-4 animate-spin" aria-hidden="true" />
              {t('commit.pending')}
            </p>
          )}
          {op.status === 'error' && (
            <p className="rounded-xl bg-rose-100 px-3 py-2 text-sm font-medium text-rose-700 dark:bg-rose-950/50 dark:text-rose-300" role="alert">{t('commit.error')}</p>
          )}
          {op.status === 'success-preview' && (
            <div className="rounded-xl bg-emerald-100 px-3 py-2 text-sm text-emerald-700 dark:bg-emerald-950/50 dark:text-emerald-300" role="status">
              <p className="font-semibold">{t('commit.successTitle')}</p>
              <p className="mt-0.5">{t('commit.success')}</p>
            </div>
          )}

          {SHOW_DEV_SIM && (
            <div className="rounded-card border border-dashed border-amber-400/70 bg-amber-100/30 dark:bg-amber-950/20">
              <button
                type="button"
                aria-expanded={simOpen}
                onClick={() => setSimOpen((v) => !v)}
                className="focus-ring flex w-full items-center gap-2 rounded-card px-3 py-2 text-sm font-medium text-amber-700 dark:text-amber-300"
              >
                <FlaskConical className="h-4 w-4" aria-hidden="true" />
                <span className="flex-1 text-left">{t('commit.sim.title')}</span>
                <ChevronDown className={cn('h-4 w-4 transition-transform', simOpen && 'rotate-180')} aria-hidden="true" />
              </button>
              {simOpen && (
                <fieldset disabled={pending} className="grid gap-3 px-3 pb-3">
                  <div className="flex flex-col gap-1">
                    <label htmlFor="commit-sim-readiness" className="text-sm font-medium text-fg-muted">{t('commit.sim.readiness')}</label>
                    <Select id="commit-sim-readiness" className="text-base" value={fixture} onChange={(e) => setFixture(e.target.value as Fixture)}>
                      {(Object.keys(FIXTURES) as Fixture[]).map((f) => (
                        <option key={f} value={f}>{t(`commit.sim.fixture.${f}` as TKey)}</option>
                      ))}
                    </Select>
                  </div>
                  <div className="flex flex-col gap-1">
                    <label htmlFor="commit-sim-outcome" className="text-sm font-medium text-fg-muted">{t('commit.sim.outcome')}</label>
                    <Select id="commit-sim-outcome" className="text-base" value={outcome} onChange={(e) => setOutcome(e.target.value as 'success' | 'error')}>
                      <option value="success">{t('commit.sim.outcome.success')}</option>
                      <option value="error">{t('commit.sim.outcome.error')}</option>
                    </Select>
                  </div>
                </fieldset>
              )}
            </div>
          )}
        </div>

        <div className="flex shrink-0 justify-end gap-2 border-t border-line px-6 py-4">
          <Button variant="ghost" size="sm" className="h-9 px-4 text-base" disabled={pending} onClick={() => handleOpenChange(false)}>
            {op.status === 'success-preview' ? t('commit.close') : t('commit.cancel')}
          </Button>
          {op.status !== 'success-preview' && (
            <Button variant="primary" size="sm" className="h-9 px-4 text-base" disabled={!eligible} onClick={handleConfirm}>
              {pending && <Loader2 className="h-3.5 w-3.5 animate-spin" aria-hidden="true" />}
              {op.status === 'error' ? t('commit.retry') : t('commit.confirm')}
            </Button>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
