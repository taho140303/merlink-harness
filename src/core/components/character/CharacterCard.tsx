import { useNavigate } from "react-router";
import { ArrowUpRight } from "lucide-react";
import type { CharacterItem } from "./types";

interface CharacterCardProps {
  character: CharacterItem;
}

export function CharacterCard({ character }: CharacterCardProps) {
  const navigate = useNavigate();

  return (
    <div
      onClick={() => navigate(`/character/${character.id}`)}
      className="group relative flex cursor-pointer flex-col justify-between rounded-2xl border border-slate-200 bg-white p-5 shadow-xs transition-all duration-200 hover:border-slate-300 hover:shadow-md dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700"
    >
      <div>
        <div className="flex items-center gap-3.5">
          {/* Avatar placeholder chữ cái đầu */}
          <div
            className={`flex h-11 w-11 shrink-0 items-center justify-center rounded-xl font-bold text-white shadow-xs ${character.avatarBg}`}
          >
            {character.name.charAt(0)}
          </div>

          <div className="truncate">
            <h3 className="truncate text-sm font-semibold tracking-tight text-slate-900 group-hover:text-blue-600 dark:text-slate-100 dark:group-hover:text-blue-400 transition-colors">
              {character.name}
            </h3>
            <p className="truncate text-xs text-slate-500 dark:text-slate-400">
              {character.species}
            </p>
          </div>
        </div>

        {/* Tags cơ bản */}
        <div className="mt-4 flex flex-wrap gap-1.5">
          <span className="rounded-md border border-slate-200 bg-slate-50 px-2 py-0.5 text-[11px] font-medium text-slate-600 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-300">
            {character.biologicalSex}
          </span>
          <span className="rounded-md border border-slate-200 bg-slate-50 px-2 py-0.5 text-[11px] font-medium text-slate-600 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-300">
            {character.genderIdentity}
          </span>
        </div>
      </div>

      <div className="mt-4 flex items-center justify-between border-t border-slate-100 pt-3 text-xs text-slate-400 dark:border-slate-800/80">
        <span>{character.relationships.length} Relationships</span>
        <ArrowUpRight className="h-4 w-4 opacity-0 transition-opacity group-hover:opacity-100 text-slate-700 dark:text-slate-200" />
      </div>
    </div>
  );
}
