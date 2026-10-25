import { useState, useMemo } from "react";
import { Search, Plus } from "lucide-react";
import { MOCK_CHARACTERS } from "./mockCharacters";
import { CharacterCard } from "./CharacterCard";
import { CreateCharacterDialog } from "./CreateCharacterDialog";
import { useT } from "../../../shared/i18n/useT";
import type { CharacterItem } from "./types";

export function CharacterListView() {
  const t = useT();
  const [characters, setCharacters] = useState<CharacterItem[]>(MOCK_CHARACTERS);
  const [searchTerm, setSearchTerm] = useState("");
  const [filterSex, setFilterSex] = useState<string>("ALL");
  const [isCreateOpen, setIsCreateOpen] = useState(false);

  const filteredCharacters = useMemo(() => {
    return characters.filter((c) => {
      const matchSearch =
        c.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        c.species.toLowerCase().includes(searchTerm.toLowerCase());
      const matchFilter = filterSex === "ALL" || c.biologicalSex === filterSex;
      return matchSearch && matchFilter;
    });
  }, [characters, searchTerm, filterSex]);

  const handleAddCharacter = (newChar: CharacterItem) => {
    setCharacters((prev) => [newChar, ...prev]);
  };

  return (
    <div className="flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8">
      <div className="flex flex-col gap-4 border-b border-slate-200 pb-6 dark:border-slate-800 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-slate-900 dark:text-slate-100 sm:text-2xl">
            {t("character.title")}
          </h1>
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
            {t("character.subtitle")}
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2.5">
          <div className="relative w-full sm:w-56">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder={t("character.searchPlaceholder")}
              className="w-full rounded-xl border border-slate-200 bg-white py-1.5 pl-9 pr-3 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
            />
          </div>

          <select
            value={filterSex}
            onChange={(e) => setFilterSex(e.target.value)}
            className="rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-2xs focus:outline-hidden dark:border-slate-800 dark:bg-slate-900 dark:text-slate-200"
          >
            <option value="ALL">{t("sex.all")}</option>
            <option value="Male">{t("sex.male")}</option>
            <option value="Female">{t("sex.female")}</option>
            <option value="Intersex">{t("sex.intersex")}</option>
            <option value="Unknown">{t("sex.unknown")}</option>
          </select>

          <button
            type="button"
            onClick={() => setIsCreateOpen(true)}
            className="flex items-center gap-1.5 rounded-xl bg-slate-900 px-3.5 py-1.5 text-xs font-medium text-white shadow-xs transition-colors hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            <Plus className="h-3.5 w-3.5" />
            <span>{t("character.create")}</span>
          </button>
        </div>
      </div>

      <div className="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
        {filteredCharacters.map((c) => (
          <CharacterCard key={c.id} character={c} />
        ))}
        {filteredCharacters.length === 0 && (
          <div className="col-span-full py-16 text-center text-xs text-slate-400">
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
