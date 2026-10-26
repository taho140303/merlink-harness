import { User, Shield, Activity } from 'lucide-react';
import { Dialog, DialogContent } from '../../../../shared/components/ui/dialog';
import { MotifBlock } from './MotifBlock';
import type { DraftCharacter, DraftCharacterDetail, DraftStep, FeedbackContext } from '../types';

interface CharacterDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  character: DraftCharacter | null;
  detail?: DraftCharacterDetail;
  currentPhase: DraftStep;
  onFeedback?: (f: FeedbackContext) => void;
}

export function CharacterDialog({
  open,
  onOpenChange,
  character,
  detail,
  currentPhase,
  onFeedback,
}: CharacterDialogProps) {
  if (!character) return null;

  // Xác định xem giai đoạn hiện tại đã tới bước hiển thị Detail chưa
  const phasesOrder: DraftStep[] = ['overview', 'arcs', 'chapter', 'scene', 'config', 'world', 'character', 'characterDetail'];
  const currentIndex = phasesOrder.indexOf(currentPhase);
  const detailIndex = phasesOrder.indexOf('characterDetail');
  const showDetail = currentIndex >= detailIndex;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl p-6">
        {/* Header */}
        <div className="flex items-start gap-3 border-b border-slate-100 pb-4 dark:border-slate-800">
          <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-indigo-50 text-indigo-600 dark:bg-indigo-950/50 dark:text-indigo-400">
            <User className="h-5 w-5" />
          </div>
          <div>
            <h3 className="text-base font-bold text-slate-900 dark:text-slate-100">
              {character.name}
            </h3>
            <p className="mt-0.5 text-xs font-medium text-slate-500 dark:text-slate-400">
              {character.role}
            </p>
          </div>
        </div>

        <div className="mt-4 space-y-5">
          {/* Motifs liên quan đến nhân vật */}
          <div>
            <div className="flex items-center gap-2 mb-1">
              <Shield className="h-4 w-4 text-slate-400" />
              <h4 className="text-sm font-semibold text-slate-900 dark:text-slate-100">Character Arc Motifs</h4>
            </div>
            <MotifBlock motifs={character.motifs} onFeedback={onFeedback} />
            {character.motifs.length === 0 && (
              <p className="text-[11px] text-slate-400 italic">Chưa có motif nào được gắn.</p>
            )}
          </div>

          {/* Detail thông tin (dựng lên sau) */}
          <div>
            <div className="flex items-center gap-2 mb-2 border-t border-slate-100 pt-4 dark:border-slate-800">
              <Activity className="h-4 w-4 text-slate-400" />
              <h4 className="text-sm font-semibold text-slate-900 dark:text-slate-100">Psychological & Action Logic</h4>
            </div>
            
            {showDetail && detail ? (
              <div className="space-y-3 rounded-xl border border-slate-200 bg-slate-50 p-3.5 dark:border-slate-800 dark:bg-slate-900/50">
                <div>
                  <span className="text-[10px] uppercase tracking-wide text-slate-400 block mb-0.5">Tính cách</span>
                  <p className="text-xs text-slate-700 dark:text-slate-300">{detail.personality}</p>
                </div>
                <div>
                  <span className="text-[10px] uppercase tracking-wide text-slate-400 block mb-0.5">Hành động</span>
                  <p className="text-xs text-slate-700 dark:text-slate-300">{detail.actionLogic}</p>
                </div>
                <div className="rounded-lg bg-indigo-50/50 border border-indigo-100 p-2.5 dark:bg-indigo-950/30 dark:border-indigo-900/60">
                  <span className="text-[10px] uppercase tracking-wide text-indigo-700 dark:text-indigo-300 block mb-0.5">Theme Alignment</span>
                  <p className="text-xs text-indigo-900 dark:text-indigo-100">{detail.themeAlignment}</p>
                </div>
                <div>
                  <span className="text-[10px] uppercase tracking-wide text-slate-400 block mb-0.5">Vai trò trong Scene</span>
                  <p className="text-xs text-slate-700 dark:text-slate-300">{detail.sceneRole}</p>
                </div>
              </div>
            ) : (
              <div className="flex flex-col items-center justify-center rounded-xl border border-dashed border-slate-200 bg-slate-50/50 p-6 text-center dark:border-slate-800 dark:bg-slate-900/20">
                <Activity className="h-6 w-6 text-slate-300 dark:text-slate-600 mb-2 animate-pulse" />
                <p className="text-xs font-medium text-slate-500 dark:text-slate-400">
                  Thông tin chi tiết chưa được tạo.
                </p>
                <p className="text-[10px] text-slate-400 mt-1">
                  Sẽ được mô phỏng ở bước <strong>Character Detail</strong>.
                </p>
              </div>
            )}
          </div>
        </div>

        <div className="mt-6 flex justify-end">
          <button
            type="button"
            onClick={() => onOpenChange(false)}
            className="rounded-xl bg-slate-900 px-4 py-2 text-xs font-semibold text-white hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            Đóng
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
