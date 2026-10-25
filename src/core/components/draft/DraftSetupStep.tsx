import { useState, useEffect } from "react";
import { ArrowRight, MessageSquareQuote, Loader2 } from "lucide-react";
import type { SetupBlock, FeedbackContext } from "./types";

interface DraftSetupStepProps {
  onFeedback: (feedback: FeedbackContext) => void;
  onNext: () => void;
}

const SETUP_DATA: SetupBlock[] = [
  {
    id: "geo",
    tag: "Geography",
    title: "World Geography & Continental Leylines",
    loadingTitle: "Establishing world geography & continental leylines...",
    content: "Lục địa Aethelgard trôi dạt giữa các đứt gãy không gian, bao quanh bởi dải sương mù Aetheric Veil.",
  },
  {
    id: "history",
    tag: "History",
    title: "Ancient Calamity & Historical Eras",
    loadingTitle: "Chronicles of world history & cataclysms...",
    content: "300 năm trước, biến cố 'The Great Sundering' nổ ra do khai thác quá tải lõi pha lê cộng hưởng.",
  },
  {
    id: "culture",
    tag: "Culture",
    title: "Cultural Traditions & Belief Systems",
    loadingTitle: "Synthesizing cultural traditions & belief systems...",
    content: "Cư dân tôn thờ các Cột Trụ Vĩnh Cửu, lấy triết lý điều hòa dòng chảy tinh thể làm kim chỉ nam đạo đức.",
  },
  {
    id: "factions",
    tag: "Factions",
    title: "Major Political Factions & Guilds",
    loadingTitle: "Structuring major political factions & alliances...",
    content: "Liên hiệp Thợ Rèn Pha Lê đối đầu ngầm với Viện Ma Pháp Hoàng Gia để tranh quyền kiểm soát cổng vận chuyển.",
  },
];

export function DraftSetupStep({ onFeedback, onNext }: DraftSetupStepProps) {
  const [blocks, setBlocks] = useState<SetupBlock[]>(SETUP_DATA);
  const [streamedIds, setStreamedIds] = useState<string[]>([]);

  useEffect(() => {
    // Streaming từng card tuần tự, load xong thằng nào bung ngay thằng đó
    SETUP_DATA.forEach((item, index) => {
      const delay = (index + 1) * 450;
      setTimeout(() => {
        setStreamedIds((prev) => (prev.includes(item.id) ? prev : [...prev, item.id]));
      }, delay);
    });
  }, []);

  const allStreamed = streamedIds.length === SETUP_DATA.length;

  return (
    <div className="relative flex min-h-full flex-col justify-between p-6 pb-24">
      <div className="space-y-4">
        {blocks.map((block, idx) => {
          const isLoaded = streamedIds.includes(block.id);

          if (!isLoaded) {
            return (
              <div
                key={block.id}
                className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900/60 transition-all animate-pulse"
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Loader2 className="h-3.5 w-3.5 animate-spin text-blue-500" />
                    <span className="text-xs font-medium text-slate-500 dark:text-slate-400">
                      {block.loadingTitle}
                    </span>
                  </div>
                  <span className="rounded-md bg-slate-100 px-2 py-0.5 text-[10px] text-slate-400 dark:bg-slate-800">
                    {block.tag}
                  </span>
                </div>
                <div className="mt-3.5 space-y-2">
                  <div className="h-3 w-5/6 rounded-md bg-slate-100 dark:bg-slate-800" />
                  <div className="h-3 w-2/3 rounded-md bg-slate-100 dark:bg-slate-800" />
                </div>
              </div>
            );
          }

          return (
            <div
              key={block.id}
              className="group rounded-2xl border border-slate-200 bg-white p-4 shadow-xs transition-all duration-300 hover:border-slate-300 dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700 animate-in fade-in-50 zoom-in-98"
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
          );
        })}
      </div>

      {/* Floating Bottom Center Action */}
      {allStreamed && (
        <div className="fixed bottom-6 right-6 z-20 flex items-center justify-center animate-in fade-in-0 duration-300">
          <button
            type="button"
            onClick={onNext}
            className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            <span>Next: Character</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
