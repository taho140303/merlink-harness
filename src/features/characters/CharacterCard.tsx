import { useNavigate } from "react-router";
import { ArrowUpRight } from "lucide-react";
import { useT } from "@/shared/i18n/useT";
import type { CharacterProfile } from "./types";

interface CharacterCardProps {
  character: CharacterProfile;
}

export function CharacterCard({ character }: CharacterCardProps) {
  const navigate = useNavigate();
  const t = useT();

  const name = character.basicInfo.name;
  const species = character.basicInfo.species;
  const biologicalSex = character.basicInfo.biologicalSex;
  const genderIdentity = character.basicInfo.genderIdentity;

  return (
    <div
      role="link"
      tabIndex={0}
      onClick={() => navigate(`/character/${character.id}`)}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          navigate(`/character/${character.id}`);
        }
      }}
      aria-label={`${t("character.title")}: ${name}`}
      className="group relative flex cursor-pointer flex-col justify-between tier-card rounded-card p-5 focus-ring transition-colors duration-200"
    >
      <div>
        <div className="flex items-center gap-3.5">
          <div
            className={`flex h-11 w-11 shrink-0 items-center justify-center rounded-xl font-bold text-white ${character.avatarBg || 'bg-indigo-500'}`}
          >
            {name.charAt(0)}
          </div>
          <div className="truncate">
            <h3 className="truncate text-sm font-semibold tracking-tight text-fg transition-colors group-hover:text-blue-600 dark:group-hover:text-blue-400">
              {name}
            </h3>
            <p className="truncate text-xs text-fg-muted">
              {species}
            </p>
          </div>
        </div>

        <div className="mt-4 flex flex-wrap gap-1.5">
          <span className="rounded-md border border-line bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted">
            {biologicalSex}
          </span>
          <span className="rounded-md border border-line bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted">
            {genderIdentity}
          </span>
        </div>
      </div>

      <div className="mt-4 flex items-center justify-between border-t border-line pt-3 text-xs text-fg-subtle">
        <span>
          {character.relationships.length} {t("character.relationshipsCount")}
        </span>
        <ArrowUpRight className="h-4 w-4 text-fg opacity-0 transition-opacity group-hover:opacity-100" />
      </div>
    </div>
  );
}
