import { useEffect, useMemo, useRef, useState } from 'react';
import { Loader2 } from 'lucide-react';
import { Dialog, DialogContent, DialogDescription, DialogTitle } from '@/shared/components/ui/dialog';
import { Button } from '@/shared/components/ui/button';
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

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-baseline justify-between gap-3 text-xs">
      <dt className="text-fg-muted">{label}</dt>
      <dd className="min-w-0 break-words text-right font-medium">{value}</dd>
    </div>
  );
}

export function CommitConfirmationDialog({ open, onOpenChange, overview, readiness }: CommitConfirmationDialogProps) {
  const t = useT();
  const [op, setOp] = useState<CommitOpState>({ status: 'idle' });
  const busyRef = useRef(false); // synchronous duplicate-submit guard
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [fixture, setFixture] = useState<Fixture>('none');
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

  const num = (v: number | null) => (v === null ? t('commit.unavailable') : String(v));

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

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent
        className="flex max-h-[85vh] max-w-lg flex-col overflow-hidden p-0"
        onEscapeKeyDown={(e) => { if (pending) e.preventDefault(); }}
        onPointerDownOutside={(e) => { if (pending) e.preventDefault(); }}
        onInteractOutside={(e) => { if (pending) e.preventDefault(); }}
      >
        <div className="min-h-0 flex-1 space-y-4 overflow-y-auto p-6 pr-12">
          <div>
            <DialogTitle className="text-base">{t('commit.title')}</DialogTitle>
            <DialogDescription className="mt-1 text-xs">{t('commit.desc')}</DialogDescription>
          </div>

          <p className="rounded-control border border-border px-3 py-2 text-xs text-fg-muted">
            <span className="mr-1 font-semibold">{t('commit.previewBadge')}.</span>
            {t('commit.backendUnavailable')}
          </p>

          <section aria-label={t('commit.planSummary')} className="space-y-1.5">
            <h3 className="text-xs font-semibold uppercase tracking-wide text-fg-muted">{t('commit.planSummary')}</h3>
            <dl className="space-y-1">
              <Row label={t('commit.project')} value={plan.projectName ?? t('commit.unavailable')} />
              {plan.description && <Row label={t('commit.description')} value={plan.description} />}
              <Row label={t('commit.arcs')} value={num(plan.arcs)} />
              <Row label={t('commit.chapters')} value={num(plan.chapters)} />
              <Row label={t('commit.scenes')} value={num(plan.scenes)} />
              <Row label={t('commit.characters')} value={num(plan.characters)} />
              <Row label={t('commit.worldbuilding')} value={plan.worldbuildingAvailable ? t('commit.available') : t('commit.unavailable')} />
              <Row label={t('commit.relationships')} value={plan.relationships === null ? t('commit.notEvaluated') : String(plan.relationships)} />
            </dl>
          </section>

          <section className="space-y-1.5">
            <h3 className="text-xs font-semibold uppercase tracking-wide text-fg-muted">{t('commit.validation')}</h3>
            <dl><Row label={t(`commit.validation.${validation.status}` as TKey)} value="" /></dl>
            <p className="text-xs text-fg-muted">{t(validationDesc)}</p>
          </section>

          <section className="space-y-1.5">
            <h3 className="text-xs font-semibold uppercase tracking-wide text-fg-muted">{t('commit.patches')}</h3>
            <dl><Row label={patchValue} value="" /></dl>
          </section>

          {!eligible && !pending && op.status === 'idle' && (
            <ul className="list-disc space-y-0.5 pl-4 text-xs text-fg-muted" role="status">
              {blockReasons.map((r) => <li key={r}>{t(BLOCK_KEYS[r])}</li>)}
            </ul>
          )}

          {pending && (
            <p className="flex items-center gap-2 text-xs" role="status">
              <Loader2 className="h-4 w-4 animate-spin" aria-hidden="true" />
              {t('commit.pending')}
            </p>
          )}
          {op.status === 'error' && <p className="text-xs text-red-600 dark:text-red-400" role="alert">{t('commit.error')}</p>}
          {op.status === 'success-preview' && (
            <div className="rounded-control border border-border px-3 py-2 text-xs" role="status">
              <p className="font-semibold">{t('commit.successTitle')}</p>
              <p className="mt-0.5 text-fg-muted">{t('commit.success')}</p>
            </div>
          )}

          {SHOW_DEV_SIM && (
            <fieldset disabled={pending} className="space-y-1.5 rounded-control border border-dashed border-border p-3 text-xs">
              <legend className="px-1 text-fg-muted">{t('commit.sim.title')}</legend>
              <label className="flex items-center justify-between gap-2">
                {t('commit.sim.readiness')}
                <select className="rounded border border-border bg-transparent px-1 py-0.5" value={fixture} onChange={(e) => setFixture(e.target.value as Fixture)}>
                  {(Object.keys(FIXTURES) as Fixture[]).map((f) => (
                    <option key={f} value={f}>{t(`commit.sim.fixture.${f}` as TKey)}</option>
                  ))}
                </select>
              </label>
              <label className="flex items-center justify-between gap-2">
                {t('commit.sim.outcome')}
                <select className="rounded border border-border bg-transparent px-1 py-0.5" value={outcome} onChange={(e) => setOutcome(e.target.value as 'success' | 'error')}>
                  <option value="success">{t('commit.sim.outcome.success')}</option>
                  <option value="error">{t('commit.sim.outcome.error')}</option>
                </select>
              </label>
            </fieldset>
          )}
        </div>

        <div className="flex shrink-0 justify-end gap-2 border-t border-border px-6 py-3">
          <Button variant="ghost" size="sm" disabled={pending} onClick={() => handleOpenChange(false)}>
            {op.status === 'success-preview' ? t('commit.close') : t('commit.cancel')}
          </Button>
          {op.status !== 'success-preview' && (
            <Button variant="primary" size="sm" disabled={!eligible} onClick={handleConfirm}>
              {pending && <Loader2 className="mr-1 h-3.5 w-3.5 animate-spin" aria-hidden="true" />}
              {op.status === 'error' ? t('commit.retry') : t('commit.confirm')}
            </Button>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
