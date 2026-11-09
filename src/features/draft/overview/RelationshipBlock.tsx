import { HeartHandshake, Shield, Sparkles } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import { useDbEnumLabel } from '@/shared/i18n/dbEnums';
import type { DraftRelationship } from '../types';

interface RelationshipBlockProps {
  relationships: DraftRelationship[];
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

const roleBadgeColor: Record<string, string> = {
  SUPERIOR: 'bg-purple-100 text-purple-700 dark:bg-purple-950/60 dark:text-purple-300',
  EQUAL: 'bg-blue-100 text-blue-700 dark:bg-blue-950/60 dark:text-blue-300',
  INFERIOR: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

const categoryBadgeColor: Record<string, string> = {
  FAMILY: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  ROMANTIC: 'bg-pink-100 text-pink-700 dark:bg-pink-950/60 dark:text-pink-300',
  SOCIAL: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  PROFESSIONAL: 'bg-surface-muted text-fg',
  ANTAGONISTIC: 'bg-red-100 text-red-700 dark:bg-red-950/60 dark:text-red-300',
};

export function RelationshipBlock({
  relationships,
  isFeedbackMode,
  onAttachFeedback,
}: RelationshipBlockProps) {
  const t = useT();
  const enumLabel = useDbEnumLabel();

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-base font-bold text-fg">
            Mạng lưới quan hệ nhân vật (Phase 6)
          </h3>
          <p className="text-xs text-fg-muted mt-0.5">
            Động lực tương tác giữa các nhân vật và diễn biến tâm lý theo từng bối cảnh.
          </p>
        </div>
        <span className="rounded-full bg-surface-muted px-2.5 py-0.5 text-xs font-semibold text-fg-muted">
          {relationships.length} mối quan hệ
        </span>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {relationships.map((rel) => {
          const handleAttach = (e: React.MouseEvent) => {
            if (isFeedbackMode && onAttachFeedback) {
              e.stopPropagation();
              onAttachFeedback(rel.id, 'relationship', `${rel.fromCharacterName} ↔ ${rel.toCharacterName}`);
            }
          };

          return (
            <div
              key={rel.id}
              onClick={handleAttach}
              className={cn(
                "tier-card rounded-card p-5 space-y-4 transition-colors duration-200",
                isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
              )}
            >
              <div className="flex items-start justify-between gap-3">
                <div className="flex items-center gap-2">
                  <HeartHandshake className="h-5 w-5 text-indigo-500 shrink-0" />
                  <div>
                    <h4 className="text-sm font-bold text-fg">
                      {rel.fromCharacterName} <span className="text-fg-subtle font-normal">↔</span> {rel.toCharacterName}
                    </h4>
                    <span className="text-xs text-fg-muted font-medium">
                      {rel.role.subType}
                    </span>
                  </div>
                </div>

                <div className="flex flex-wrap items-center gap-1.5 shrink-0">
                  <span
                    className={cn(
                      "rounded-full px-2 py-0.5 text-[11px] font-semibold",
                      roleBadgeColor[rel.role.hierarchy] || 'bg-surface-muted text-fg'
                    )}
                  >
                    {enumLabel('Hierarchy', rel.role.hierarchy)}
                  </span>
                  <span
                    className={cn(
                      "rounded-full px-2 py-0.5 text-[11px] font-semibold",
                      categoryBadgeColor[rel.role.category] || 'bg-surface-muted text-fg'
                    )}
                  >
                    {enumLabel('CategoryGroup', rel.role.category)}
                  </span>
                </div>
              </div>

              {rel.motif && (
                <div className="rounded-lg bg-surface-muted/60 p-2.5 text-xs">
                  <div className="flex items-center gap-1.5 font-semibold text-indigo-600 dark:text-indigo-400">
                    <Sparkles className="h-3.5 w-3.5" />
                    <span>Motif quan hệ:</span>
                  </div>
                  <p className="mt-1 text-fg-muted leading-relaxed">
                    {rel.motif.description}
                  </p>
                </div>
              )}

              {/* Emotional dimension state bars */}
              <div className="border-t border-line pt-3 space-y-2">
                <span className="text-[11px] font-semibold uppercase tracking-wider text-fg-subtle block">
                  Chỉ số quan hệ động lực
                </span>
                <div className="grid grid-cols-2 gap-2 text-xs">
                  {Object.entries(rel.states).map(([dim, level]) => (
                    <div key={dim} className="flex items-center justify-between rounded bg-surface-muted/40 px-2 py-1">
                      <span className="capitalize text-fg-muted text-[11px]">{dim}</span>
                      <span className="font-semibold text-fg text-[11px]">{level}</span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
