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
        'group relative w-[220px] rounded-card tier-card px-3.5 py-3 transition-colors',
        selected
          ? 'border-indigo-400 ring-2 ring-indigo-200 dark:border-indigo-500 dark:ring-indigo-900/60'
          : 'border-line hover:border-line-strong -strong ',
      )}
    >
      <Handle
        type="target"
        position={Position.Left}
        className="!h-2 !w-2 !border-2 !border-line !bg-indigo-500"
      />

      <p className="truncate font-mono text-xs font-semibold text-fg-subtle">
        {eventBlock.code}
      </p>

      <div className="mt-1.5 space-y-1">
        <div className="flex items-center gap-1.5 rounded-md bg-indigo-50/70 px-2 py-1 dark:bg-indigo-950/40">
          <span className="text-xs font-semibold uppercase tracking-wide text-indigo-600 dark:text-indigo-300">
            {t('sp.eventBlock.component')}
          </span>
          <span className="truncate text-xs font-semibold text-fg">
            {component?.name ?? '—'}
          </span>
        </div>
        <div className="flex items-center gap-1.5 rounded-md bg-emerald-50/70 px-2 py-1 dark:bg-emerald-950/40">
          <span className="text-xs font-semibold uppercase tracking-wide text-emerald-600 dark:text-emerald-300">
            {t('sp.eventBlock.state')}
          </span>
          <span className="truncate text-xs font-semibold text-fg">
            {state?.name ?? '—'}
          </span>
        </div>
      </div>

      <Handle
        type="source"
        position={Position.Right}
        className="!h-2 !w-2 !border-2 !border-line !bg-indigo-500"
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
          aria-label={t('sp.canvas.openDetail')}
          title={t('sp.canvas.openDetail')}
          className="focus-ring flex h-6 w-6 items-center justify-center rounded-full border border-line bg-surface-card text-fg-muted  transition-colors hover:border-line-strong hover:text-fg -strong  -subtle "
        >
          <MoreVertical className="h-3.5 w-3.5" />
        </button>

      </div>
    </div>
  );
});
