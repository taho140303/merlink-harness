import { useState } from 'react';
import { Search, RotateCcw, Trash2, FolderGit2, User } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { useT } from '../../../shared/i18n/useT';
import { DeleteConfirmDialog } from '../../../shared/components/ui/delete-confirm-dialog';

type FilterType = 'ALL' | 'PROJECT' | 'CHARACTER';

interface DeletedItem {
  id: string;
  name: string;
  type: 'PROJECT' | 'CHARACTER';
  deletedAt: string;
  details: string;
}

const MOCK_DELETED_ITEMS: DeletedItem[] = [
  { id: 'd-1', name: 'Echoes of the Void', type: 'PROJECT', deletedAt: '2026-10-25 14:30', details: 'Sci-Fi / Space Opera' },
  { id: 'd-2', name: 'Kaelen Vance', type: 'CHARACTER', deletedAt: '2026-10-24 09:15', details: 'High Human' },
  { id: 'd-3', name: 'The Sunken Leyline', type: 'PROJECT', deletedAt: '2026-10-20 11:45', details: 'Oceanpunk / Exploration' },
  { id: 'd-4', name: 'Lyra Dawnseeker', type: 'CHARACTER', deletedAt: '2026-10-18 16:20', details: 'Human / Solar Blessed' },
];

export function RecycleBinTab() {
  const t = useT();
  const [filter, setFilter] = useState<FilterType>('ALL');
  const [search, setSearch] = useState('');
  const [deleteTarget, setDeleteTarget] = useState<DeletedItem | null>(null);
  const [deleteOpen, setDeleteOpen] = useState(false);
  
  // Note: In a real app, state would come from a store or props
  const [items, setItems] = useState<DeletedItem[]>(MOCK_DELETED_ITEMS);

  const filteredItems = items.filter((item) => {
    const matchFilter = filter === 'ALL' || item.type === filter;
    const matchSearch =
      item.name.toLowerCase().includes(search.toLowerCase()) ||
      item.details.toLowerCase().includes(search.toLowerCase());
    return matchFilter && matchSearch;
  });

  const handleRestore = (id: string) => {
    // Mock restore action
    setItems((prev) => prev.filter((item) => item.id !== id));
  };

  const requestDelete = (item: DeletedItem) => {
    setDeleteTarget(item);
    setDeleteOpen(true);
  };

  const confirmPermanentDelete = () => {
    if (deleteTarget) {
      setItems((prev) => prev.filter((item) => item.id !== deleteTarget.id));
      setDeleteTarget(null);
    }
  };

  const FILTERS: FilterType[] = ['ALL', 'PROJECT', 'CHARACTER'];
  const filterLabels: Record<FilterType, string> = {
    ALL: t('recycleBin.filter.all'),
    PROJECT: t('recycleBin.filter.projects'),
    CHARACTER: t('recycleBin.filter.characters'),
  };

  return (
    <div className="space-y-8">
      <div className="flex flex-col gap-4 pr-10 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h3 className="text-base font-semibold text-fg">
            {t('recycleBin.title')}
          </h3>
          <p className="text-xs text-fg-muted">
            {t('recycleBin.subtitle')}
          </p>
        </div>

        <div className="flex items-center gap-2">
          {/* Search */}
          <div className="relative w-48">
            <Search className="absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-fg-subtle" />
            <input
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder={t('recycleBin.searchPlaceholder')}
              className="w-full rounded-xl border border-line bg-surface-card py-1.5 pl-8 pr-3 text-xs text-fg placeholder:text-fg-subtle focus-ring   "
            />
          </div>
        </div>
      </div>

      {/* Filter Switch */}
      <div className="flex w-fit items-center gap-1 rounded-xl bg-surface-muted p-0.5 ">
        {FILTERS.map((f) => {
          const count = f === 'ALL' ? items.length : items.filter((i) => i.type === f).length;
          return (
            <button
              key={f}
              type="button"
              onClick={() => setFilter(f)}
              className={cn(
                'flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs font-medium transition-all',
                filter === f
                  ? 'bg-surface-card text-fg'
                  : 'text-fg-muted hover:text-fg  ',
              )}
            >
              <span>{filterLabels[f]}</span>
              <span
                className={cn(
                  'rounded-full px-1.5 py-0.5 text-xs font-semibold',
                  filter === f
                    ? 'bg-accent text-white  '
                    : 'bg-surface-muted text-fg  ',
                )}
              >
                {count}
              </span>
            </button>
          );
        })}
      </div>

      {/* List */}
      <div className="space-y-2">
        {filteredItems.length === 0 ? (
          <div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-line bg-surface-muted/50 py-12 text-center  ">
            <Trash2 className="h-8 w-8 text-fg-subtle  mb-2" />
            <p className="text-sm font-medium text-fg-muted">
              {t('recycleBin.empty')}
            </p>
          </div>
        ) : (
          filteredItems.map((item) => (
            <div
              key={item.id}
              className="flex items-center justify-between gap-3 tier-card rounded-card p-3"
            >
              <div className="flex items-center gap-3 min-w-0">
                <div className={cn(
                  "flex h-8 w-8 shrink-0 items-center justify-center rounded-lg",
                  item.type === 'PROJECT' 
                    ? "bg-blue-100 text-blue-600 dark:bg-blue-950/60 dark:text-blue-400"
                    : "bg-indigo-100 text-indigo-600 dark:bg-indigo-950/60 dark:text-indigo-400"
                )}>
                  {item.type === 'PROJECT' ? <FolderGit2 className="h-4 w-4" /> : <User className="h-4 w-4" />}
                </div>
                <div className="min-w-0">
                  <h4 className="truncate text-xs font-semibold text-fg">
                    {item.name}
                  </h4>
                  <p className="truncate text-xs text-fg-muted">
                    {item.details}
                  </p>
                </div>
              </div>

              <div className="flex items-center gap-4 shrink-0">
                <span className="text-xs text-fg-subtle hidden sm:inline-block">
                  {t('recycleBin.deletedAt')}: {item.deletedAt}
                </span>
                <div className="flex items-center gap-1">
                  <button
                    type="button"
                    onClick={() => handleRestore(item.id)}
                    className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-medium text-emerald-600 hover:bg-emerald-50 dark:text-emerald-400 dark:hover:bg-emerald-950/50 transition-colors"
                    title={t('recycleBin.restore')}
                  >
                    <RotateCcw className="h-3.5 w-3.5" />
                    <span className="hidden sm:inline">{t('recycleBin.restore')}</span>
                  </button>
                  <button
                    type="button"
                    onClick={() => requestDelete(item)}
                    className="flex h-7 items-center gap-1 rounded-lg px-2 text-xs font-medium text-rose-600 hover:bg-rose-50 dark:text-rose-400 dark:hover:bg-rose-950/50 transition-colors"
                    title={t('recycleBin.delete')}
                  >
                    <Trash2 className="h-3.5 w-3.5" />
                    <span className="hidden sm:inline">{t('recycleBin.delete')}</span>
                  </button>
                </div>
              </div>
            </div>
          ))
        )}
      </div>

      <DeleteConfirmDialog
        open={deleteOpen}
        onOpenChange={setDeleteOpen}
        title={t('recycleBin.confirmDeleteTitle')}
        description={`${deleteTarget?.name ?? ''} — ${t('recycleBin.confirmDeleteDesc')}`}
        confirmLabel={t('delete.confirm')}
        cancelLabel={t('common.cancel')}
        onConfirm={confirmPermanentDelete}
      />
    </div>
  );
}
