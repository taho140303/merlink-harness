import { Info } from 'lucide-react';
import { Tooltip } from '@/shared/components/ui/tooltip';
import { cn } from '@/shared/utils/cn';

interface InfoTooltipProps {
  content: string;
  className?: string;
}

export function InfoTooltip({ content, className }: InfoTooltipProps) {
  return (
    <Tooltip content={content}>
      <span className={cn('inline-flex items-center text-fg-subtle', className)}>
        <Info aria-hidden="true" className="h-3.5 w-3.5 cursor-help" />
      </span>
    </Tooltip>
  );
}
