import { useState, useMemo } from "react";
import { Search, Plus, Users } from "lucide-react";
import { MOCK_CHARACTERS } from "./mockCharacters";
import { CharacterCard } from "./CharacterCard";
import type { CharacterItem } from "./types";

export function CharacterListView() {
  const [characters, setCharacters] = useState<CharacterItem[]>(MOCK_CHARACTERS);
  const [searchTerm, setSearchTerm] = useState("");
  const [filterSex, setFilterSex] = useState<string>("ALL");

  const filteredCharacters = useMemo(() => {
    return characters.filter((c) => {
      const matchSearch =
        c.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        c.species.toLowerCase().includes(searchTerm.toLowerCase());
      const matchFilter = filterSex === "ALL" || c.biologicalSex === filterSex;
      return matchSearch && matchFilter;
    });
  }, [characters, searchTerm, filterSex]);

  const handleCreateCharacter = () => {
    const newId = `char-${Date.now()}`;
    const newChar: CharacterItem = {
      id: newId,
      name: `New Character #${characters.length + 1}`,
      species: "Human",
      biologicalSex: "Unknown",
      genderIdentity: "Unspecified",
      genderExpression: "Neutral",
      avatarBg: "bg-blue-600",
      appearance: {
        height: 170,
        heightUnit: "cm",
        weight: 60,
        weightUnit: "kg",
        hairColor: "Black",
        hairHex: "#000000",
        hairNote: "",
        eyeColor: "Brown",
        eyeHex: "#5c4033",
        eyeNote: "",
        skinTone: "Fair",
        skinHex: "#f0d5be",
        skinNote: "",
        outfitStyle: "Standard traveling attire",
        defaultSummary: "A newly created character awaiting detailed lore.",
      },
      distinguishingMarks: [],
      relationships: [],
    };
    setCharacters((prev) => [newChar, ...prev]);
  };

  return (
    <div className="flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8">
      {/* Header bar */}
      <div className="flex flex-col gap-4 border-b border-slate-200 pb-6 dark:border-slate-800 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-slate-900 dark:text-slate-100 sm:text-2xl">
            Character
          </h1>
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
            Quản lý danh sách nhân vật, hồ sơ nhân trắc học và đồ thị quan hệ.
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2.5">
          {/* Thanh tìm kiếm */}
          <div className="relative w-full sm:w-56">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder="Search by name, species..."
              className="w-full rounded-xl border border-slate-200 bg-white pl-9 pr-3 py-1.5 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
            />
          </div>

          {/* Filter giới tính */}
          <select
            value={filterSex}
            onChange={(e) => setFilterSex(e.target.value)}
            className="rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-2xs focus:outline-hidden dark:border-slate-800 dark:bg-slate-900 dark:text-slate-200"
          >
            <option value="ALL">All Sex</option>
            <option value="Male">Male</option>
            <option value="Female">Female</option>
            <option value="Intersex">Intersex</option>
            <option value="Unknown">Unknown</option>
          </select>

          {/* Nút Create Character */}
          <button
            type="button"
            onClick={handleCreateCharacter}
            className="flex items-center gap-1.5 rounded-xl bg-slate-900 px-3.5 py-1.5 text-xs font-medium text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200 transition-colors"
          >
            <Plus className="h-3.5 w-3.5" />
            <span>Create Character</span>
          </button>
        </div>
      </div>

      {/* Grid danh sách */}
      <div className="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
        {filteredCharacters.map((c) => (
          <CharacterCard key={c.id} character={c} />
        ))}

        {filteredCharacters.length === 0 && (
          <div className="col-span-full py-16 text-center text-xs text-slate-400">
            Không tìm thấy nhân vật phù hợp.
          </div>
        )}
      </div>
    </div>
  );
}
