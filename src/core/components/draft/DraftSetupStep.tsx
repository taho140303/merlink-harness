import { useState, useEffect } from "react";
import { ArrowRight, MessageSquareQuote, Loader2, Sparkles } from "lucide-react";
import type { SetupBlock, FeedbackContext } from "./types";

interface DraftSetupStepProps {
  onFeedback: (feedback: FeedbackContext) => void;
  onNext: () => void;
}

const INITIAL_BLOCKS: SetupBlock[] = [
  {
    id: "geo",
    tag: "Geography",
    title: "World Geography & Continental Leylines",
    content: "Lục địa Aethelgard trôi dạt giữa các đứt gãy không gian, bao quanh bởi dải sương mù Aetheric Veil.",
  },
  {
    id: "history",
    tag: "History",
    title: "Ancient Calamity & Historical Eras",
    content: "300 năm trước, biến cố 'The Great Sundering' nổ ra do khai thác quá tải lõi pha lê cộng hưởng.",
  },
  {
    id: "culture",
    tag: "Culture",
    title: "Cultural Traditions & Belief Systems",
    content: "Cư dân tôn thờ các Cột Trụ Vĩnh Cửu, lấy triết lý điều hòa dòng chảy tinh thể làm kim chỉ nam đạo đức.",
  },
  {
    id: "factions",
    tag: "Factions",
    title: "Major Political Factions & Guilds",
    content: "Liên hiệp Thợ Rèn Pha Lê đối đầu ngầm với Viện Ma Pháp Hoàng Gia để tranh quyền kiểm soát cổng vận chuyển.",
  },
];

const LOADING_TITLES = [
  "Establishing world geography & continental leylines...",
  "Chronicles of world history & cataclysms...",
  "Synthesizing cultural traditions & belief systems...",
  "Structuring major political factions & alliances...",
];

export function DraftSetupStep({ onFeedback, onNext }: DraftSetupStepProps) {
  const [loading, setLoading] = useState(true);
  const [blocks, setBlocks] = useState<SetupBlock[]>(INITIAL_BLOCKS);

  useEffect(() => {
    const timer = setTimeout(() => setLoading(false), 2000);
    return () => clearTimeout(timer);
  }, []);

  if (loading) {
    return (
      <div className="space-y-4 p-6">
        <div className="flex items-center gap-2 text-xs font-semibold text-blue-600 dark:text-blue-400">
          <Loader2 className="h-4 w-4 animate-spin" />
          <span>Generating world foundational parameters...</span>
        </div>

        <div className="space-y-3">
          {LOADING_TITLES.map((title, idx) => (
            <div
              key={idx}
              className="animate-pulse rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900/60"
            >
              <div className="flex items-center justify-between">
                <span className="text-xs font-medium text-slate-500 dark:text-slate-400">{title}</span>
                <span className="h-4 w-16 rounded-md bg-slate-200 dark:bg-slate-800" />
              </div>
              <div className="mt-3 space-y-2">
                <div className="h-3 w-5/6 rounded-sm bg-slate-200 dark:bg-slate-800" />
                <div className="h-3 w-3/4 rounded-sm bg-slate-200 dark:bg-slate-800" />
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="relative flex min-h-full flex-col justify-between p-6 pb-24">
      <div className="space-y-4">
        {blocks.map((block, idx) => (
          <div
            key={block.id}
            className="group rounded-2xl border border-slate-200 bg-white p-4 shadow-xs transition-colors hover:border-slate-300 dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700"
          >
            <div className="flex items-center justify-between">
              <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">
                {block.title}
              </h4>
              <span className="rounded-md bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">
                {block.tag}
              </span>
            </div>

            <textarea
              value={block.content}
              onChange={(e) => {
                const next = [...blocks];
                next[idx].content = e.target.value;
                setBlocks(next);
              }}
              rows={3}
              className="mt-3 w-full resize-none rounded-xl border border-slate-200 bg-slate-50/70 p-2.5 text-xs text-slate-800 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-950/50 dark:text-slate-200"
            />

            <div className="mt-2.5 flex justify-end">
              <button
                type="button"
                onClick={() =>
                  onFeedback({
                    id: block.id,
                    tag: block.tag,
                    title: block.title,
                    content: block.content,
                  })
                }
                className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs font-medium text-slate-500 hover:bg-slate-100 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-blue-400 transition-colors"
              >
                <MessageSquareQuote className="h-3.5 w-3.5" />
                <span>Góp ý & sửa đổi</span>
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* Floating Bottom Center Next Button */}
      <div className="fixed bottom-6 right-[calc(25%-60px)] z-20">
        <button
          type="button"
          onClick={onNext}
          className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
        >
          <span>Next: Character</span>
          <ArrowRight className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
}
