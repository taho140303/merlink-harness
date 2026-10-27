import { useState } from 'react';
import { Bookmark, MessageSquareQuote, Tags } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { Dialog, DialogContent, DialogTitle } from '../../../shared/components/ui/dialog';
import { useT } from '../../../shared/i18n/useT';
import type { DraftOverview } from './types';

const MAX_VISIBLE_GENRES = 9;

interface DraftOverviewBlockProps {
  overview: DraftOverview;
  onFeedback?: () => void;
}

/**
 * Khối tổng quan dự án dùng chung cho mọi phase của Draft Building.
 *
 * Giữ nguyên layout của khối thông tin lớn: (1) tên truyện + badge ngôn ngữ + cụm hành động,
 * (2) đoạn tóm tắt cốt truyện, (3) cụm mục tiêu (# arc / # chapter / # word-chap) đặt cạnh
 * dải thể loại, và (4) danh sách thể loại rút gọn 9 thẻ + nút mở rộng.
 * Toàn bộ bề mặt dùng design system neumorphic của dự án, không dùng glassmorphism của file mẫu.
 */
export function DraftOverviewBlock({ overview, onFeedback }: DraftOverviewBlockProps) {
  const t = useT();
  const [allGenresOpen, setAllGenresOpen] = useState(false);

  const visibleGenres = overview.genres.slice(0, MAX_VISIBLE_GENRES);
  const remainingGenres = overview.genres.length - MAX_VISIBLE_GENRES;

  const targets = [
    {
      key: 'arcs',
      label: t('draft.overview.arcs'),
      value: overview.target.arcs,
      hashClass: 'text-indigo-500 dark:text-indigo-400',
      valueClass: 'text-indigo-600 dark:text-indigo-300',
    },
    {
      key: 'chapters',
      label: t('draft.overview.chapters'),
      value: overview.target.chapters,
      hashClass: 'text-emerald-500 dark:text-emerald-400',
      valueClass: 'text-emerald-600 dark:text-emerald-300',
    },
    {
      key: 'words',
      label: t('draft.overview.wordsPerChapter'),
      value: overview.target.wordPerChapter,
      hashClass: 'text-violet-500 dark:text-violet-400',
      valueClass: 'text-violet-600 dark:text-violet-300',
    },
  ];

  return (
    <header className="@container rounded-3xl p-5 neu-flat">
      {/* (1) Tên truyện + ngôn ngữ + cụm hành động */}
      <div className="flex flex-col gap-3 @md:flex-row @md:items-center @md:justify-between">
        <div className="flex min-w-0 items-center gap-3">
          <h2 className="min-w-0 text-2xl font-black leading-tight tracking-tight text-slate-900 dark:text-slate-100">
            {overview.displayName}
          </h2>
          <span className="shrink-0 rounded-xl px-2.5 py-1 font-mono text-[11px] font-bold uppercase tracking-widest text-indigo-600 neu-pressed dark:text-indigo-400">
            {overview.language}
          </span>
        </div>

        <div className="flex items-center gap-2 self-start @md:self-auto">
          {onFeedback && (
            <button
              type="button"
              onClick={onFeedback}
              className="flex shrink-0 items-center gap-2 whitespace-nowrap rounded-xl px-3.5 py-1.5 text-xs font-bold text-slate-700 transition-all hover:neu-pressed dark:text-slate-200"
            >
              <MessageSquareQuote className="h-3.5 w-3.5 text-indigo-500" />
              <span>{t('draft.overview.feedback')}</span>
            </button>
          )}
        </div>
      </div>

      {/* (2) Tóm tắt cốt truyện */}
      <p className="mt-4 max-w-4xl text-sm leading-relaxed text-slate-600 dark:text-slate-300">
        {overview.summary}
      </p>

      {/* (3) Cụm mục tiêu + dải thể loại */}
      <div className="mt-4 flex flex-col justify-between gap-4 @lg:flex-row @lg:flex-wrap @lg:items-center">
        <div className="flex w-fit shrink-0 flex-wrap items-center gap-2 rounded-2xl p-1.5 neu-pressed">
          {targets.map((target) => (
            <div
              key={target.key}
              className="flex items-center gap-1.5 whitespace-nowrap rounded-xl border border-slate-200/70 bg-white/70 px-3 py-1.5 font-mono text-xs font-bold text-slate-700 shadow-2xs dark:border-slate-700/70 dark:bg-slate-900/70 dark:text-slate-200"
            >
              <span className={cn('font-black', target.hashClass)}>#</span>
              <span className="text-slate-400 dark:text-slate-500">{target.label}:</span>
              <span className={cn('text-sm font-extrabold', target.valueClass)}>{target.value}</span>
            </div>
          ))}
        </div>

        {/* (4) Thể loại: tối đa 9 thẻ, phần còn lại gom vào nút mở rộng */}
        <div className="flex flex-wrap items-center gap-2">
          {visibleGenres.map((genre) => (
            <span
              key={genre.name}
              className={cn(
                'flex items-center gap-1.5 whitespace-nowrap rounded-xl px-3 py-1.5 text-xs',
                genre.isPrimary
                  ? 'border border-indigo-500/30 bg-indigo-500/10 font-bold text-indigo-700 dark:text-indigo-300'
                  : 'border border-slate-200/80 bg-white/60 font-medium text-slate-600 dark:border-slate-700/80 dark:bg-slate-900/40 dark:text-slate-300'
              )}
            >
              {genre.isPrimary && <Bookmark className="h-3 w-3 fill-current" />}
              <span>{genre.name}</span>
            </span>
          ))}

          {remainingGenres > 0 && (
            <button
              type="button"
              onClick={() => setAllGenresOpen(true)}
              title={t('draft.overview.moreGenres')}
              className="flex items-center gap-1.5 rounded-xl px-3 py-1.5 text-xs font-black text-indigo-600 transition-all hover:neu-pressed dark:text-indigo-400"
            >
              <span>...</span>
              <span className="rounded-lg px-1.5 py-0.5 font-mono text-[10px] font-bold text-slate-500 neu-pressed dark:text-slate-300">
                +{remainingGenres}
              </span>
            </button>
          )}
        </div>
      </div>

      <Dialog open={allGenresOpen} onOpenChange={setAllGenresOpen}>
        <DialogContent className="max-w-md p-6">
          <div className="flex items-center gap-2 border-b border-slate-200 pb-3 dark:border-slate-800">
            <Tags className="h-4 w-4 text-indigo-500" />
            <DialogTitle className="text-sm font-bold text-slate-900 dark:text-slate-100">
              {t('draft.overview.allGenres')}
            </DialogTitle>
          </div>

          <div className="flex max-h-[50vh] flex-wrap gap-2 overflow-y-auto py-1">
            {overview.genres.map((genre) => (
              <span
                key={genre.name}
                className={cn(
                  'flex items-center gap-1.5 whitespace-nowrap rounded-xl px-3 py-1.5 text-xs',
                  genre.isPrimary
                    ? 'border border-indigo-500/30 bg-indigo-500/10 font-bold text-indigo-700 dark:text-indigo-300'
                    : 'border border-slate-200/80 bg-white/60 font-medium text-slate-600 dark:border-slate-700/80 dark:bg-slate-900/40 dark:text-slate-300'
                )}
              >
                {genre.isPrimary && <Bookmark className="h-3 w-3 fill-current" />}
                <span>{genre.name}</span>
              </span>
            ))}
          </div>

          <div className="flex justify-end border-t border-slate-200 pt-3 dark:border-slate-800">
            <button
              type="button"
              onClick={() => setAllGenresOpen(false)}
              className="rounded-xl px-4 py-1.5 text-xs font-bold text-slate-700 transition-all hover:neu-pressed dark:text-slate-200"
            >
              {t('common.close')}
            </button>
          </div>
        </DialogContent>
      </Dialog>
    </header>
  );
}
