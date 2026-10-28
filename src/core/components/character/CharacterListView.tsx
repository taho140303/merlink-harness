import { useState, useMemo } from "react";
import { Search, Plus } from "lucide-react";
import { MOCK_CHARACTERS } from "./mockCharacters";
import { CharacterCard } from "./CharacterCard";
import { CreateCharacterDialog } from "./CreateCharacterDialog";
import { useT } from "../../../shared/i18n/useT";
import { Input } from "../../../shared/components/ui/input";
import { Select } from "../../../shared/components/ui/select";
import { Button } from "../../../shared/components/ui/button";
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
    <div className="flex h-full flex-col overflow-y-auto bg-surface-muted/50 p-6  sm:p-8">
      <div className="flex flex-col gap-4 border-b border-line pb-6  sm:flex-row sm:items-center sm:justify-between">
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

          <Select
            aria-label={t("sex.all")}
            value={filterSex}
            onChange={(e) => setFilterSex(e.target.value)}
            className="py-1.5 text-xs"
          >
            <option value="ALL">{t("sex.all")}</option>
            <option value="Male">{t("sex.male")}</option>
            <option value="Female">{t("sex.female")}</option>
            <option value="Intersex">{t("sex.intersex")}</option>
            <option value="Unknown">{t("sex.unknown")}</option>
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
