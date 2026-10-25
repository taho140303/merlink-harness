import { useState, useEffect } from "react";
import { ArrowLeft, Play, Loader2, MessageSquareQuote } from "lucide-react";
import type { PlanPhase, FeedbackContext } from "./types";

interface DraftPlanStepProps {
  onFeedback: (feedback: FeedbackContext) => void;
  onBack: () => void;
  onStart: () => void;
}

const PLAN_DATA: PlanPhase[] = [
  {
    id: "act-1",
    tag: "Act I",
    title: "Inciting Spark & World Discovery",
    loadingTitle: "Generating chapter breakdown for Act I...",
    content: "Kaelen phát hiện di tích ngầm bị kích hoạt trái phép. Lyra nhận lệnh điều tra dòng năng lượng lạ.",
  },
  {
    id: "act-2",
    tag: "Act II",
    title: "Convergence of Conflicting Agendas",
    loadingTitle: "Plotting tension spikes & escalation for Act II...",
    content: "Hai thế lực chạm trán tại cổng dịch chuyển cổ xưa. Bí mật về vụ nổ Sundering dần bị phơi bày.",
  },
  {
    id: "act-3",
    tag: "Act III",
    title: "Climax & Spatial Fracture",
    loadingTitle: "Designing multi-path climax scenarios for Act III...",
    content: "Lõi pha lê vỡ vụn buộc các nhân vật phải chọn giữa sự ổn định của đế chế hoặc sự giải phóng chân tướng.",
  },
];

export function DraftPlanStep({ onFeedback, onBack, onStart }: DraftPlanStepProps) {
  const [phases, setPhases] = useState<PlanPhase[]>(PLAN_DATA);
  const [streamedIds, setStreamedIds] = useState<string[]>([]);

  useEffect(() => {
    PLAN_DATA.forEach((item, idx) => {
      const delay = (idx + 1) * 500;
      setTimeout(() => {
        setStreamedIds((prev) => (prev.includes(item.id) ? prev : [...prev, item.id]));
      }, delay);
    });
  }, []);

  const allStreamed = streamedIds.length === PLAN_DATA.length;

  return (
    <div className="relative flex min-h-full flex-col justify-between p-6 pb-24">
      <div className="space-y-4">
        {phases.map((phase, idx) => {
          const isLoaded = streamedIds.includes(phase.id);

          if (!isLoaded) {
            return (
              <div
                key={phase.id}
                className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900/60 animate-pulse"
              >
                <div className="flex items-center gap-2">
                  <Loader2 className="h-3.5 w-3.5 animate-spin text-blue-500" />
                  <span className="text-xs font-medium text-slate-500 dark:text-slate-400">
                    {phase.loadingTitle}
                  </span>
                </div>
                <div className="mt-3.5 h-3 w-4/5 rounded-md bg-slate-100 dark:bg-slate-800" />
              </div>
            );
          }

          return (
            <div
              key={phase.id}
              className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs transition-all dark:border-slate-800 dark:bg-slate-900 animate-in fade-in-50 zoom-in-98"
            >
              <div className="flex items-center justify-between">
                <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">
                  {phase.title}
                </h4>
                <span className="rounded-md bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">
                  {phase.tag}
                </span>
              </div>

              <textarea
                value={phase.content}
                onChange={(e) => {
                  const next = [...phases];
                  next[idx].content = e.target.value;
                  setPhases(next);
                }}
                rows={3}
                className="mt-3 w-full resize-none rounded-xl border border-slate-200 bg-slate-50/70 p-2.5 text-xs text-slate-800 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-950/50 dark:text-slate-200"
              />

              <div className="mt-2.5 flex justify-end">
                <button
                  type="button"
                  onClick={() =>
                    onFeedback({
                      id: phase.id,
                      tag: phase.tag,
                      title: phase.title,
                      content: phase.content,
                    })
                  }
                  className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs font-medium text-slate-500 hover:bg-slate-100 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-blue-400 transition-colors"
                >
                  <MessageSquareQuote className="h-3.5 w-3.5" />
                  <span>Góp ý & sửa đổi</span>
                </button>
              </div>
            </div>
          );
        })}
      </div>

      {/* Floating Bottom Navigation: Back + START button */}
      {allStreamed && (
        <div className="fixed bottom-6 right-6 z-20 flex items-center gap-2.5 animate-in fade-in-0 duration-300">
          <button
            type="button"
            onClick={onBack}
            className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-4 py-2.5 text-xs font-semibold text-slate-700 shadow-lg hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800 transition-all"
          >
            <ArrowLeft className="h-4 w-4" />
            <span>Back</span>
          </button>
          <button
            type="button"
            onClick={onStart}
            className="flex items-center gap-2 rounded-full bg-blue-600 px-6 py-2.5 text-xs font-semibold text-white shadow-lg shadow-blue-500/25 transition-all hover:scale-105 hover:bg-blue-500"
          >
            <Play className="h-4 w-4 fill-current" />
            <span>Start</span>
          </button>
        </div>
      )}
    </div>
  );
}
