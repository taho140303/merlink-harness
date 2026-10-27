import { CharacterListView } from "../../core/components/character/CharacterListView";

export function meta() {
  return [
    { title: "Merlink Harness - Character" },
    { name: "description", content: "Character directory and relation graphs" },
  ];
}

export default function CharacterRoute() {
  return <CharacterListView />;
}
