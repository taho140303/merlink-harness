import { useState } from "react";
import { MessageSquareQuote, Shield, HeartHandshake } from "lucide-react";
import { Dialog, DialogContent } from "../ui/dialog";
import { cn } from "../../../shared/utils/cn";
import type { CharacterProfile, FeedbackContext } from "./types";

interface CharacterDetailDialogProps {
  character: CharacterProfile | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onFeedback: (feedback: FeedbackContext) => void;
}

type CharTab = "core" | "experience" | "psychology";

export function CharacterDetailDialog({
  character,
  open,
  onOpenChange,
  onFeedback,
}: CharacterDetailDialogProps) {
  const [activeTab, setActiveTab] = useState<CharTab>("core");

  if (!character) return null;

  const handleFeedbackClick = (subField: string) => {
    onFeedback({
      id: `${character.id}-${subField}`,
      tag: "Character",
      title: `${character.name} (${subField})`,
    });
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl p-6">
        {/* Top Center Switcher */}
        <div className="flex flex-col items-center gap-3 border-b border-slate-100 pb-4 dark:border-slate-800">
          <div className="text-center">
            <h3 className="text-base font-semibold text-slate-900 dark:text-slate-100">
              {character.name}
            </h3>
            <p className="text-xs text-slate-500 dark:text-slate-400">{character.role}</p>
          </div>

          <div className="inline-flex rounded-xl bg-slate-100 p-1 dark:bg-slate-800">
            <button
              type="button"
              onClick={() => setActiveTab("core")}
              className={cn(
                "rounded-lg px-3 py-1.5 text-xs font-medium transition-all",
                activeTab === "core"
                  ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                  : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
              )}
            >
              Core Identity
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("experience")}
              className={cn(
                "rounded-lg px-3 py-1.5 text-xs font-medium transition-all",
                activeTab === "experience"
                  ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                  : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
              )}
            >
              Life Experience
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("psychology")}
              className={cn(
                "rounded-lg px-3 py-1.5 text-xs font-medium transition-all",
                activeTab === "psychology"
                  ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                  : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
              )}
            >
              Psychological
            </button>
          </div>
        </div>

        {/* Tab Content */}
        <div className="py-2 text-xs leading-relaxed text-slate-700 dark:text-slate-300">
          {activeTab === "core" && (
            <div className="space-y-4">
              <div className="grid grid-cols-3 gap-2 rounded-xl bg-slate-50 p-3 dark:bg-slate-950/50 border border-slate-100 dark:border-slate-800/80">
                <div>
                  <span className="text-[10px] uppercase text-slate-400">Age</span>
                  <p className="font-semibold text-slate-900 dark:text-slate-100">
                    {character.coreIdentity.age}
                  </p>
                </div>
                <div>
                  <span className="text-[10px] uppercase text-slate-400">Origin</span>
                  <p className="font-semibold text-slate-900 dark:text-slate-100">
                    {character.coreIdentity.origin}
                  </p>
                </div>
                <div>
                  <span className="text-[10px] uppercase text-slate-400">Allegiance</span>
                  <p className="font-semibold text-slate-900 dark:text-slate-100">
                    {character.coreIdentity.allegiance}
                  </p>
                </div>
              </div>

              {/* Relationships */}
              <div className="space-y-2">
                <div className="flex items-center gap-1.5 text-xs font-semibold text-slate-900 dark:text-slate-100">
                  <HeartHandshake className="h-4 w-4 text-blue-500" />
                  <span>Interpersonal Relationships</span>
                </div>
                <div className="space-y-2">
                  {character.coreIdentity.relationships.map((rel, i) => (
                    <div
                      key={i}
                      className="rounded-xl border border-slate-200 bg-white p-3 dark:border-slate-800 dark:bg-slate-900"
                    >
                      <div className="flex items-center justify-between">
                        <span className="font-semibold text-slate-900 dark:text-slate-100">
                          {rel.targetName}
                        </span>
                        <span className="rounded-md bg-blue-50 px-2 py-0.5 text-[10px] font-medium text-blue-600 dark:bg-blue-950/60 dark:text-blue-300">
                          {rel.relation}
                        </span>
                      </div>
                      <p className="mt-1 text-slate-500 dark:text-slate-400">{rel.description}</p>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {activeTab === "experience" && (
            <div className="rounded-xl border border-slate-200 bg-slate-50/50 p-4 dark:border-slate-800 dark:bg-slate-950/40">
              <p className="whitespace-pre-line">{character.lifeExperience}</p>
            </div>
          )}

          {activeTab === "psychology" && (
            <div className="rounded-xl border border-slate-200 bg-slate-50/50 p-4 dark:border-slate-800 dark:bg-slate-950/40">
              <p className="whitespace-pre-line">{character.psychological}</p>
            </div>
          )}
        </div>

        {/* Footer Actions */}
        <div className="flex items-center justify-between border-t border-slate-100 pt-3 dark:border-slate-800">
          <button
            type="button"
            onClick={() => handleFeedbackClick(activeTab)}
            className="flex items-center gap-1.5 text-xs font-medium text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 transition-colors"
          >
            <MessageSquareQuote className="h-4 w-4" />
            <span>Góp ý & sửa đổi</span>
          </button>

          <button
            type="button"
            onClick={() => onOpenChange(false)}
            className="rounded-xl bg-slate-900 px-4 py-2 text-xs font-medium text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            Đóng
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
