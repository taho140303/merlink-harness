import { memo, useState } from 'react';
import { Handle, Position, type NodeProps } from '@xyflow/react';
import { MoreVertical } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import type { EventBlockNodeType } from './types';

export const EventBlockNode = memo(function EventBlockNode({
  data,
  selected,
}: NodeProps<EventBlockNodeType>) {
  const t = useT();
  const [hovered, setHovered] = useState(false);

  const { eventBlock, component, state, onOpenDetail } = data;

  return (
    <div
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      className={cn(
        'group relative w-[220px] rounded-xl border bg-white px-3.5 py-3 shadow-sm transition-all dark:bg-slate-900',
        selected
          ? 'border-indigo-400 ring-2 ring-indigo-200 dark:border-indigo-500 dark:ring-indigo-900/60'
          : 'border-slate-200 hover:border-slate-300 dark:border-slate-700 dark:hover:border-slate-600',
      )}
    >
      <Handle
        type="target"
        position={Position.Left}
        className="!h-2 !w-2 !border-2 !border-white !bg-indigo-500 dark:!border-slate-900"
      />

      <p className="truncate font-mono text-[10px] font-semibold text-slate-400 dark:text-slate-500">
        {eventBlock.code}
      </p>

      <div className="mt-1.5 space-y-1">
        <div className="flex items-center gap-1.5 rounded-md bg-indigo-50/70 px-2 py-1 dark:bg-indigo-950/40">
          <span className="text-[9px] font-semibold uppercase tracking-wide text-indigo-600 dark:text-indigo-300">
            {t('sp.eventBlock.component')}
          </span>
          <span className="truncate text-[11px] font-semibold text-slate-800 dark:text-slate-200">
            {component?.name ?? '—'}
          </span>
        </div>
        <div className="flex items-center gap-1.5 rounded-md bg-emerald-50/70 px-2 py-1 dark:bg-emerald-950/40">
          <span className="text-[9px] font-semibold uppercase tracking-wide text-emerald-600 dark:text-emerald-300">
            {t('sp.eventBlock.state')}
          </span>
          <span className="truncate text-[11px] font-semibold text-slate-800 dark:text-slate-200">
            {state?.name ?? '—'}
          </span>
        </div>
      </div>

      <Handle
        type="source"
        position={Position.Right}
        className="!h-2 !w-2 !border-2 !border-white !bg-indigo-500 dark:!border-slate-900"
      />

      {/* 3-dot trigger — only visible on hover/selected */}
      <div
        className={cn(
          'absolute -top-2.5 right-2 transition-opacity',
          hovered || selected ? 'opacity-100' : 'opacity-0',
        )}
      >
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            onOpenDetail(eventBlock);
          }}
          className="flex h-6 w-6 items-center justify-center rounded-full border border-slate-200 bg-white text-slate-500 shadow-sm transition-colors hover:border-slate-300 hover:text-slate-900 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-400 dark:hover:text-slate-100"
        >
          <MoreVertical className="h-3.5 w-3.5" />
        </button>

        {/* Themed tooltip */}
        <span className="pointer-events-none absolute right-0 top-full mt-1.5 w-max max-w-[180px] rounded-md bg-slate-900 px-2 py-1 text-[10px] font-medium text-white opacity-0 shadow-lg transition-opacity group-hover:opacity-100 dark:bg-slate-100 dark:text-slate-900">
          {t('sp.canvas.openDetail')}
        </span>
      </div>
    </div>
  );
});
