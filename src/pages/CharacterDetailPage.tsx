import { useParams } from "react-router";
import { CharacterDetailLayout } from "@/features/characters";

export default function CharacterDetailPage() {
  const { characterId } = useParams();
  return <CharacterDetailLayout characterId={characterId} />;
}
