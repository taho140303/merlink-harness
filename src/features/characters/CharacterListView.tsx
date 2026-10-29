import { useState, useMemo } from "react";
import { Search, Plus } from "lucide-react";
import { useCharacterStore } from "./store";
import { CharacterCard } from "./CharacterCard";
import { CreateCharacterDialog } from "./CreateCharacterDialog";
import { MOCK_PROJECTS } from "@/features/projects/mockProjects";
import { useT } from "@/shared/i18n/useT";
import { Input } from "@/shared/components/ui/input";
import { Select } from "@/shared/components/ui/select";
import { Button } from "@/shared/components/ui/button";
import type { CharacterProfile, Gender } from "./types";

export function CharacterListView() {
  const t = useT();
  const characters = useCharacterStore((s) => s.charactersList);
  const addCharacter = useCharacterStore((s) => s.addCharacter);

  const [searchTerm, setSearchTerm] = useState("");
  const [filterSex, setFilterSex] = useState<string>("ALL");
  const [filterProject, setFilterProject] = useState<string>("ALL");
  const [isCreateOpen, setIsCreateOpen] = useState(false);

  const projectOptions = useMemo(() => {
    const map = new Map<string, string>();
    characters.forEach((c) => {
      if (c.projectId && c.projectName) {
        map.set(c.projectId, c.projectName);
      }
    });
    MOCK_PROJECTS.forEach((p) => {
      if (!map.has(p.id)) {
        map.set(p.id, p.title);
      }
    });
    return Array.from(map.entries()).map(([id, title]) => ({ id, title }));
  }, [characters]);

  const filteredCharacters = useMemo(() => {
    return characters.filter((c) => {
      const name = c.basicInfo.name.toLowerCase();
      const species = c.basicInfo.species.toLowerCase();
      const term = searchTerm.toLowerCase();

      const matchSearch = name.includes(term) || species.includes(term);
      const matchSex = filterSex === "ALL" || c.basicInfo.biologicalSex === filterSex;
      const matchProject =
        filterProject === "ALL" ||
        c.projectId === filterProject ||
        (filterProject === "proj-sample-500" && c.projectName?.includes("Tế nữ"));

      return matchSearch && matchSex && matchProject;
    });
  }, [characters, searchTerm, filterSex, filterProject]);

  const handleAddCharacter = (newChar: CharacterProfile) => {
    addCharacter(newChar);
  };

  return (
    <div className="flex h-full flex-col overflow-y-auto bg-surface-muted/50 p-6 sm:p-8">
      <div className="flex flex-col gap-4 border-b border-line pb-6 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-fg sm:text-2xl">
            {t("character.title")}
          </h1>
          <p className="mt-1 text-xs text-fg-muted">
            {t("character.subtitle")}
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2.5">
          <div className="relative w-full sm:w-56">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-fg-subtle" />
            <Input
              type="text"
              aria-label={t("character.searchPlaceholder")}
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder={t("character.searchPlaceholder")}
              className="py-1.5 pl-9 pr-3 text-xs"
            />
          </div>

          {/* Filter theo Project */}
          <Select
            aria-label={t("character.filterProject")}
            value={filterProject}
            onChange={(e) => setFilterProject(e.target.value)}
            className="py-1.5 text-xs max-w-[180px]"
          >
            <option value="ALL">{t("project.all")}</option>
            {projectOptions.map((p) => (
              <option key={p.id} value={p.id}>
                {p.title}
              </option>
            ))}
          </Select>

          {/* 1.2: Chỉ filter theo ALL, MALE, FEMALE, NONE */}
          <Select
            aria-label={t("sex.all")}
            value={filterSex}
            onChange={(e) => setFilterSex(e.target.value)}
            className="py-1.5 text-xs"
          >
            <option value="ALL">{t("sex.all")}</option>
            <option value="MALE">{t("gender.MALE")}</option>
            <option value="FEMALE">{t("gender.FEMALE")}</option>
            <option value="NONE">{t("gender.NONE")}</option>
          </Select>

          <Button
            type="button"
            onClick={() => setIsCreateOpen(true)}
            className="px-3.5 py-1.5"
          >
            <Plus className="h-3.5 w-3.5" />
            <span>{t("character.create")}</span>
          </Button>
        </div>
      </div>

      <div className="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
        {filteredCharacters.map((c) => (
          <CharacterCard key={c.id} character={c} />
        ))}
        {filteredCharacters.length === 0 && (
          <div className="tier-card rounded-card col-span-full py-16 text-center text-xs text-fg-subtle">
            {t("character.empty")}
          </div>
        )}
      </div>

      <CreateCharacterDialog
        open={isCreateOpen}
        onOpenChange={setIsCreateOpen}
        onCreate={handleAddCharacter}
      />
    </div>
  );
}
