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
          <h3 className="text-base font-semibold text-slate-900 dark:text-slate-100">
            {t('recycleBin.title')}
          </h3>
          <p className="text-xs text-slate-500 dark:text-slate-400">
            {t('recycleBin.subtitle')}
          </p>
        </div>

        <div className="flex items-center gap-2">
          {/* Search */}
          <div className="relative w-48">
            <Search className="absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
            <input
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder={t('recycleBin.searchPlaceholder')}
              className="w-full rounded-xl border border-slate-200 bg-white py-1.5 pl-8 pr-3 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-100"
            />
          </div>
        </div>
      </div>

      {/* Filter Switch */}
      <div className="flex w-fit items-center gap-1 rounded-xl bg-slate-100 p-0.5 dark:bg-slate-800">
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
                  ? 'bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white'
                  : 'text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200',
              )}
            >
              <span>{filterLabels[f]}</span>
              <span
                className={cn(
                  'rounded-full px-1.5 py-0.5 text-[10px] font-semibold',
                  filter === f
                    ? 'bg-slate-900 text-white dark:bg-slate-900 dark:text-white'
                    : 'bg-slate-200 text-slate-700 dark:bg-slate-700 dark:text-slate-300',
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
          <div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-slate-200 bg-slate-50/50 py-12 text-center dark:border-slate-800 dark:bg-slate-900/20">
            <Trash2 className="h-8 w-8 text-slate-300 dark:text-slate-600 mb-2" />
            <p className="text-sm font-medium text-slate-500 dark:text-slate-400">
              {t('recycleBin.empty')}
            </p>
          </div>
        ) : (
          filteredItems.map((item) => (
            <div
              key={item.id}
              className="flex items-center justify-between gap-3 rounded-xl border border-slate-200 bg-white p-3 shadow-sm dark:border-slate-800 dark:bg-slate-900"
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
                  <h4 className="truncate text-xs font-semibold text-slate-900 dark:text-slate-100">
                    {item.name}
                  </h4>
                  <p className="truncate text-[11px] text-slate-500 dark:text-slate-400">
                    {item.details}
                  </p>
                </div>
              </div>

              <div className="flex items-center gap-4 shrink-0">
                <span className="text-[10px] text-slate-400 hidden sm:inline-block">
                  {t('recycleBin.deletedAt')}: {item.deletedAt}
                </span>
                <div className="flex items-center gap-1">
                  <button
                    type="button"
                    onClick={() => handleRestore(item.id)}
                    className="flex h-7 items-center gap-1 rounded-lg px-2 text-[11px] font-medium text-emerald-600 hover:bg-emerald-50 dark:text-emerald-400 dark:hover:bg-emerald-950/50 transition-colors"
                    title={t('recycleBin.restore')}
                  >
                    <RotateCcw className="h-3.5 w-3.5" />
                    <span className="hidden sm:inline">{t('recycleBin.restore')}</span>
                  </button>
                  <button
                    type="button"
                    onClick={() => requestDelete(item)}
                    className="flex h-7 items-center gap-1 rounded-lg px-2 text-[11px] font-medium text-rose-600 hover:bg-rose-50 dark:text-rose-400 dark:hover:bg-rose-950/50 transition-colors"
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
