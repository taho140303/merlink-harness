import { useCallback, useEffect, useState } from 'react';
import { useNavigate } from 'react-router';
import { Search, RotateCcw, Trash2, MessageSquare, ArrowUpRight } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import { DeleteConfirmDialog } from '@/shared/components/ui/delete-confirm-dialog';
import { toast } from '@/shared/components/ui/toast';
import { backendClient, type BackendClient } from '@/shared/backend/client';
import { redactSensitiveString } from '@/shared/backend/adapter';
import type { SessionSummaryDto } from '@/shared/backend/generated';

type FilterType = 'ALL' | 'SESSION' | 'PROJECT' | 'CHARACTER';

function formatDate(value: string | null | undefined): string {
  if (!value) return '';
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? '' : new Intl.DateTimeFormat(undefined, { dateStyle: 'short' }).format(date);
}

export function isSessionInRecycleBin(session: SessionSummaryDto): boolean {
  return Boolean(session.deletedAt) || session.messageCount === 1;
}

interface Props { client?: BackendClient }

export function RecycleBinTab({ client = backendClient }: Props) {
  const t = useT();
  const navigate = useNavigate();
  const [filter, setFilter] = useState<FilterType>('ALL');
  const [search, setSearch] = useState('');
  const [items, setItems] = useState<SessionSummaryDto[]>([]);
  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  const [deleteIds, setDeleteIds] = useState<string[]>([]);
  const [deleteOpen, setDeleteOpen] = useState(false);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState(false);

  const loadItems = useCallback(async () => {
    setLoading(true);
    try {
      const collected: SessionSummaryDto[] = [];
      let cursor: string | undefined;
      const seen = new Set<string>();
      do {
        const page = await client.listChatSessions({ limit: 100, trashOnly: true, ...(cursor ? { cursor } : {}) });
        collected.push(...page.items.filter(isSessionInRecycleBin));
        cursor = page.nextCursor ?? undefined;
        if (cursor && seen.has(cursor)) break;
        if (cursor) seen.add(cursor);
      } while (cursor);
      setItems(collected);
      setSelectedIds((old) => old.filter((id) => collected.some((item) => item.sessionId === id)));
      setLoadError(false);
    } catch {
      setLoadError(true);
    } finally {
      setLoading(false);
    }
  }, [client]);

  useEffect(() => {
    void loadItems();
    window.addEventListener('merlink:chat-history-changed', loadItems);
    return () => window.removeEventListener('merlink:chat-history-changed', loadItems);
  }, [loadItems]);

  const filteredItems = items.filter((item) => {
    if (filter !== 'ALL' && filter !== 'SESSION') return false;
    const title = item.displayName || `Chat ${item.sessionId.slice(0, 6)}`;
    return title.toLowerCase().includes(search.toLowerCase());
  });

  const handleRestore = async (id: string) => {
    try {
      await client.manageChatSessions({ action: 'restore', sessionIds: [id] });
      await loadItems();
      window.dispatchEvent(new Event('merlink:chat-history-changed'));
    } catch (error) {
      toast({ title: redactSensitiveString(String(error)), variant: 'error' });
    }
  };

  const requestDelete = (ids: string[]) => {
    if (!ids.length) return;
    setDeleteIds(ids);
    setDeleteOpen(true);
  };

  const confirmPermanentDelete = async () => {
    await client.manageChatSessions({ action: 'permanent_delete', sessionIds: deleteIds });
    setSelectedIds((old) => old.filter((id) => !deleteIds.includes(id)));
    setDeleteIds([]);
    await loadItems();
    window.dispatchEvent(new Event('merlink:chat-history-changed'));
  };

  const toggleSelected = (id: string) => setSelectedIds((old) =>
    old.includes(id) ? old.filter((value) => value !== id) : [...old, id]);
  const allVisibleSelected = filteredItems.length > 0 && filteredItems.every((item) => selectedIds.includes(item.sessionId));
  const filterLabels: Record<FilterType, string> = {
    ALL: t('recycleBin.filter.all'),
    SESSION: t('recycleBin.filter.sessions'),
    PROJECT: t('recycleBin.filter.projects'),
    CHARACTER: t('recycleBin.filter.characters'),
  };

  return (
    <div className="space-y-5">
      <header className="flex flex-wrap items-start justify-between gap-3 pr-9">
        <div>
          <h3 className="text-base font-semibold text-fg">{t('recycleBin.title')}</h3>
          <p className="mt-1 text-xs text-fg-muted">{t('recycleBin.subtitle')}</p>
        </div>
        <div className="relative w-48">
          <Search className="absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-fg-subtle" />
          <input type="search" value={search} onChange={(event) => setSearch(event.target.value)}
            placeholder={t('recycleBin.searchPlaceholder')}
            className="tier-input focus-ring w-full rounded-xl py-1.5 pl-8 pr-3 text-xs" />
        </div>
      </header>

      <div className="flex flex-wrap items-center justify-between gap-2">
        <div className="flex w-fit items-center gap-1 rounded-xl bg-surface-muted p-0.5">
          {(['ALL', 'SESSION', 'PROJECT', 'CHARACTER'] as FilterType[]).map((value) => (
            <button key={value} type="button" onClick={() => setFilter(value)}
              className={cn('rounded-lg px-3 py-1.5 text-xs font-medium', filter === value ? 'bg-surface-card text-fg' : 'text-fg-muted hover:text-fg')}>
              {filterLabels[value]} <span className="ml-1">{value === 'ALL' || value === 'SESSION' ? items.length : 0}</span>
            </button>
          ))}
        </div>
        <button type="button" disabled={!selectedIds.length} onClick={() => requestDelete(selectedIds)}
          className="focus-ring rounded-lg px-2 py-1.5 text-xs font-medium text-rose-600 hover:bg-rose-50 disabled:opacity-40 dark:hover:bg-rose-950/50">
          {t('recycleBin.deleteSelected', { count: selectedIds.length })}
        </button>
      </div>

      {filteredItems.length > 0 && (
        <label className="flex items-center gap-2 text-xs text-fg-muted">
          <input type="checkbox" checked={allVisibleSelected} onChange={() => setSelectedIds((old) => allVisibleSelected
            ? old.filter((id) => !filteredItems.some((item) => item.sessionId === id))
            : Array.from(new Set([...old, ...filteredItems.map((item) => item.sessionId)])))} />
          {t('recycleBin.selectAll')}
        </label>
      )}

      {loading ? <p role="status" className="text-sm text-fg-muted">{t('recycleBin.loading')}</p>
        : loadError ? <p role="alert" className="text-sm text-rose-500">{t('recycleBin.loadFailed')}</p>
        : filteredItems.length === 0 ? (
          <div className="flex flex-col items-center rounded-2xl border border-dashed border-line bg-surface-muted/50 py-12 text-center">
            <Trash2 className="mb-2 h-8 w-8 text-fg-subtle" />
            <p className="text-sm font-medium text-fg-muted">{t('recycleBin.empty')}</p>
          </div>
        ) : (
          <div className="space-y-2">
            {filteredItems.map((item) => {
              const title = item.displayName || `Chat ${item.sessionId.slice(0, 6)}`;
              return (
                <div key={item.sessionId} className="tier-card flex items-center gap-3 rounded-card p-3">
                  <input type="checkbox" checked={selectedIds.includes(item.sessionId)} onChange={() => toggleSelected(item.sessionId)}
                    aria-label={`${t('recycleBin.selectSession')} ${title}`} />
                  <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-indigo-100 text-indigo-600 dark:bg-indigo-950/60 dark:text-indigo-400">
                    <MessageSquare className="h-4 w-4" aria-hidden="true" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <h4 className="truncate text-xs font-semibold text-fg">{title}</h4>
                    <p className="text-xs text-fg-muted">{item.deletedAt ? `${t('recycleBin.deletedAt')}: ${formatDate(item.deletedAt)}` : t('recycleBin.unfinished')}</p>
                  </div>
                  {item.deletedAt ? (
                    <button type="button" onClick={() => void handleRestore(item.sessionId)} title={t('recycleBin.restore')}
                      className="focus-ring rounded-lg p-1.5 text-emerald-600 hover:bg-emerald-50 dark:hover:bg-emerald-950/50">
                      <RotateCcw className="h-4 w-4" aria-hidden="true" />
                    </button>
                  ) : (
                    <button type="button" onClick={() => { navigate(`/chat/${item.sessionId}`); window.dispatchEvent(new Event('merlink:close-settings')); }} title={t('recycleBin.resume')}
                      className="focus-ring rounded-lg p-1.5 text-fg-muted hover:bg-surface-muted">
                      <ArrowUpRight className="h-4 w-4" aria-hidden="true" />
                    </button>
                  )}
                  <button type="button" onClick={() => requestDelete([item.sessionId])} title={t('recycleBin.delete')}
                    className="focus-ring rounded-lg p-1.5 text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-950/50">
                    <Trash2 className="h-4 w-4" aria-hidden="true" />
                  </button>
                </div>
              );
            })}
          </div>
        )}

      <DeleteConfirmDialog open={deleteOpen} onOpenChange={setDeleteOpen}
        title={t('recycleBin.confirmDeleteTitle')}
        description={`${deleteIds.length} ${t('recycleBin.confirmDeleteDesc')}`}
        confirmLabel={t('delete.confirm')} cancelLabel={t('common.cancel')}
        errorMessage={t('recycleBin.deleteFailed')}
        onConfirm={confirmPermanentDelete} />
    </div>
  );
}
