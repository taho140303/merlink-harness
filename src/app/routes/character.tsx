import { CharacterListPage } from "@/pages";

export function meta() {
  return [{ title: "Merlink Harness - Characters" }];
}

export default function CharacterRoute() {
  return <CharacterListPage />;
}
