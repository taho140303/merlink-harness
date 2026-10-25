import { useParams, Link } from "react-router";
import type { Route } from "./+types/character-detail";
import { MOCK_CHARACTERS } from "../../core/components/character/mockCharacters";
import { CharacterDetailView } from "../../core/components/character/CharacterDetailView";

export function meta({ params }: Route.MetaArgs) {
  const char = MOCK_CHARACTERS.find((c) => c.id === params.characterId);
  return [{ title: char ? `Merlink - ${char.name}` : "Merlink Character Detail" }];
}

export default function CharacterDetailRoute() {
  const { characterId } = useParams();
  const character = MOCK_CHARACTERS.find((c) => c.id === characterId);

  if (!character) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <p className="text-sm text-slate-500">Không tìm thấy nhân vật này.</p>
        <Link to="/character" className="mt-3 text-xs font-semibold text-blue-600 hover:underline">
          Quay lại danh sách Character
        </Link>
      </div>
    );
  }

  return <CharacterDetailView initialCharacter={character} />;
}
