import { useState, useEffect } from "react";
import { ArrowRight, ArrowLeft, Loader2, Eye, MessageSquareQuote } from "lucide-react";
import { CharacterDetailDialog } from "./CharacterDetailDialog";
import type { CharacterProfile, FeedbackContext } from "./types";

interface DraftCharacterStepProps {
  onFeedback: (feedback: FeedbackContext) => void;
  onNext: () => void;
  onBack: () => void;
}

const CHARACTERS_DATA: CharacterProfile[] = [
  {
    id: "char-1",
    name: "Kaelen Vance",
    role: "Arch-Magus of the Veil",
    loadingTitle: "Auditing psychological & memory vectors for Kaelen Vance...",
    coreIdentity: {
      age: "34",
      origin: "Valoria High Citadel",
      allegiance: "Imperial Aetherium",
      relationships: [
        {
          targetName: "Lyra Dawnseeker",
          relation: "Rival / Former Mentor",
          description: "Mối quan hệ phức tạp sau bất đồng trong dự án phong ấn lõi pha lê.",
        },
        {
          targetName: "Vaelin Grey",
          relation: "Informant",
          description: "Cung cấp cổ vật chợ đen để Kaelen phục dựng ma pháp cấm.",
        },
      ],
    },
    lifeExperience: "Từng dẫn đầu thám hiểm khu vực cấm Undercity và là người sống sót duy nhất sau thảm họa sụp đổ hầm mỏ số 4.",
    psychological: "Ám ảnh bởi sự hoàn hảo và nỗi sợ mất quyền kiểm soát năng lượng arcane. Luôn giữ thái độ đa nghi.",
  },
  {
    id: "char-2",
    name: "Lyra Dawnseeker",
    role: "Chief Inquisitor",
    loadingTitle: "Synthesizing demographic traits & interpersonal links for Lyra...",
    coreIdentity: {
      age: "29",
      origin: "Sunken Cathedral",
      allegiance: "The Radiant Order",
      relationships: [
        {
          targetName: "Kaelen Vance",
          relation: "Suspect of Interest",
          description: "Âm thầm theo dõi mọi động thái sử dụng tài nguyên ma thuật của Kaelen.",
        },
      ],
    },
    lifeExperience: "Gia nhập đoàn thanh tra từ năm 17 tuổi, nổi tiếng với sự cương trực và khả năng cảm nhận dị biến không gian.",
    psychological: "Ý thức công lý cực đoan, sẵn sàng hy sinh cảm xúc cá nhân vì sự ổn định của vương quốc.",
  },
];

export function DraftCharacterStep({ onFeedback, onNext, onBack }: DraftCharacterStepProps) {
  const [streamedIds, setStreamedIds] = useState<string[]>([]);
  const [selectedCharacter, setSelectedCharacter] = useState<CharacterProfile | null>(null);

  useEffect(() => {
    CHARACTERS_DATA.forEach((char, idx) => {
      const delay = (idx + 1) * 600;
      setTimeout(() => {
        setStreamedIds((prev) => (prev.includes(char.id) ? prev : [...prev, char.id]));
      }, delay);
    });
  }, []);

  const allStreamed = streamedIds.length === CHARACTERS_DATA.length;

  return (
    <div className="relative flex min-h-full flex-col justify-between p-6 pb-24">
      <div className="space-y-4">
        {CHARACTERS_DATA.map((char) => {
          const isLoaded = streamedIds.includes(char.id);

          if (!isLoaded) {
            return (
              <div
                key={char.id}
                className="flex items-center justify-between rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900/60 animate-pulse"
              >
                <div className="flex items-center gap-3">
                  <div className="h-10 w-10 rounded-xl bg-slate-100 dark:bg-slate-800 flex items-center justify-center">
                    <Loader2 className="h-4 w-4 animate-spin text-blue-500" />
                  </div>
                  <span className="text-xs font-medium text-slate-500 dark:text-slate-400">
                    {char.loadingTitle}
                  </span>
                </div>
                <div className="h-6 w-16 rounded-md bg-slate-100 dark:bg-slate-800" />
              </div>
            );
          }

          return (
            <div
              key={char.id}
              className="flex items-center justify-between rounded-2xl border border-slate-200 bg-white p-4 shadow-xs transition-all dark:border-slate-800 dark:bg-slate-900 animate-in fade-in-50 zoom-in-98"
            >
              <div className="flex items-center gap-3">
                <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-slate-100 font-bold text-slate-800 dark:bg-slate-800 dark:text-slate-200">
                  {char.name.charAt(0)}
                </div>
                <div>
                  <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">
                    {char.name}
                  </h4>
                  <p className="text-[11px] text-slate-500 dark:text-slate-400">{char.role}</p>
                </div>
              </div>

              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={() =>
                    onFeedback({
                      id: char.id,
                      tag: "Character",
                      title: char.name,
                    })
                  }
                  className="flex items-center gap-1 rounded-lg px-2.5 py-1.5 text-xs font-medium text-slate-500 hover:bg-slate-100 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-blue-400 transition-colors"
                >
                  <MessageSquareQuote className="h-3.5 w-3.5" />
                  <span>Góp ý</span>
                </button>

                <button
                  type="button"
                  onClick={() => setSelectedCharacter(char)}
                  className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-2xs hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-200 dark:hover:bg-slate-750 transition-colors"
                >
                  <Eye className="h-3.5 w-3.5" />
                  <span>View</span>
                </button>
              </div>
            </div>
          );
        })}
      </div>

      {/* Floating Bottom Navigation (Back + Next) */}
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
            onClick={onNext}
            className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            <span>Next: Plan</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}

      <CharacterDetailDialog
        character={selectedCharacter}
        open={Boolean(selectedCharacter)}
        onOpenChange={(open) => !open && setSelectedCharacter(null)}
        onFeedback={onFeedback}
      />
    </div>
  );
}
