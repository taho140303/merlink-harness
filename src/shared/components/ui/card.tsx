import { forwardRef, type HTMLAttributes } from 'react';
import { cn } from '../../utils/cn';

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  tier?: 'panel' | 'card';
  padding?: 'none' | 'sm' | 'md' | 'lg';
}

const paddingClass = {
  none: '',
  sm: 'p-3',
  md: 'p-4',
  lg: 'p-5',
} as const;

export const Card = forwardRef<HTMLDivElement, CardProps>(function Card(
  { tier = 'card', padding = 'md', className, ...props },
  ref,
) {
  return (
    <div
      ref={ref}
      className={cn(
        tier === 'panel' ? 'tier-panel rounded-panel' : 'tier-card rounded-card',
        paddingClass[padding],
        className,
      )}
      {...props}
    />
  );
});
