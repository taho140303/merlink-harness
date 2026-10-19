import type { CSSProperties } from "react";
import type { Character, Project } from "../types";

export const STORAGE_KEY = "merlink-custom-characters";

export interface PortraitPalette {
  art: string;
  cloak: string;
  skin: string;
  shadow: string;
  hair: string;
  hairLight: string;
  tilt: string;
}

/** Nine rotating palettes for the illustrated silhouettes (verbatim from ref-ui). */
export const PALETTES: PortraitPalette[] = [
  { art: "linear-gradient(148deg,#a3b9b8 2%,#70899a 43%,#273b5a 100%)", cloak: "#25344a", skin: "#d4ab8c", shadow: "#9c796e", hair: "#252e38", hairLight: "#526275", tilt: "-9deg" },
  { art: "linear-gradient(150deg,#d3a9ab 0%,#a36e83 45%,#474661 100%)", cloak: "#4b374a", skin: "#e5b597", shadow: "#a87773", hair: "#282737", hairLight: "#6a4b5b", tilt: "7deg" },
  { art: "linear-gradient(145deg,#b2bfc5 0%,#778fa1 43%,#344f64 100%)", cloak: "#263a45", skin: "#be9376", shadow: "#78574f", hair: "#232e32", hairLight: "#566c73", tilt: "-4deg" },
  { art: "linear-gradient(150deg,#d8bfad 0%,#a48684 45%,#665b73 100%)", cloak: "#533e4b", skin: "#dfb996", shadow: "#9a6f67", hair: "#342d31", hairLight: "#765c5b", tilt: "6deg" },
  { art: "linear-gradient(145deg,#a8bad0 0%,#7c89ae 44%,#343d63 100%)", cloak: "#313455", skin: "#c4a28c", shadow: "#816b6d", hair: "#292a3d", hairLight: "#565678", tilt: "-7deg" },
  { art: "linear-gradient(148deg,#c6ccc5 0%,#8ca9a6 43%,#476277 100%)", cloak: "#375363", skin: "#d1a88e", shadow: "#876c68", hair: "#25313a", hairLight: "#566b69", tilt: "9deg" },
  { art: "linear-gradient(150deg,#d6b1bd 0%,#aa8ca6 42%,#5a5975 100%)", cloak: "#44394e", skin: "#e0b79e", shadow: "#987478", hair: "#333044", hairLight: "#6a566e", tilt: "-8deg" },
  { art: "linear-gradient(148deg,#bec7bb 0%,#8c9b8c 45%,#4d686a 100%)", cloak: "#324545", skin: "#cba586", shadow: "#846b61", hair: "#283537", hairLight: "#586965", tilt: "8deg" },
  { art: "linear-gradient(150deg,#bac4d5 0%,#858caf 45%,#495376 100%)", cloak: "#313b58", skin: "#d9ab94", shadow: "#956f75", hair: "#2c3045", hairLight: "#555b78", tilt: "-5deg" },
];

export const PERSONALITY_TRAITS = [
  "Curious",
  "Resilient",
  "Guarded",
  "Loyal",
  "Ambitious",
  "Compassionate",
  "Witty",
  "Secretive",
  "Brave",
  "Patient",
  "Restless",
  "Hopeful",
];

/** Maximum portrait upload size (1.5 MB) enforced by the form. */
export const MAX_UPLOAD_BYTES = 1_500_000;

export interface CharacterFormValues {
  name: string;
  role: string;
  age: string;
  gender: string;
  story: string;
  tags: string[];
  appearance: string;
  backstory: string;
  bio: string;
  uploadedImage: string;
}

export const EMPTY_FORM: CharacterFormValues = {
  name: "",
  role: "Main",
  age: "",
  gender: "",
  story: "",
  tags: [],
  appearance: "",
  backstory: "",
  bio: "",
  uploadedImage: "",
};

export type PortraitStyle = CSSProperties & Record<`--ch-${string}`, string>;

export function assetIsPhoto(item: Pick<Character, "image" | "uploadedImage">): boolean {
  return Boolean(
    item.uploadedImage ||
      /^data:image\//i.test(item.image || "") ||
      /^(?:https?:|\.\/|assets\/)/i.test(item.image || ""),
  );
}

export function projectName(story: string, projects: Project[]): string {
  return (
    projects.find(
      (project) =>
        String(project.id) === String(story) || project.title === story,
    )?.title ||
    story ||
    "Unassigned story"
  );
}

export function readCustomCharacters(): Character[] {
  try {
    const saved = JSON.parse(
      localStorage.getItem(STORAGE_KEY) || "[]",
    ) as unknown;
    return Array.isArray(saved) ? (saved as Character[]) : [];
  } catch {
    return [];
  }
}

export function portraitStyle(item: Character, index: number): PortraitStyle {
  const palette = PALETTES[index % PALETTES.length];
  const image = item.uploadedImage || item.image;
  return {
    background: image
      ? assetIsPhoto(item)
        ? `url("${image}") center / cover`
        : image
      : palette.art,
    "--ch-default-art": palette.art,
    "--ch-cloak": palette.cloak,
    "--ch-skin": palette.skin,
    "--ch-skin-shadow": palette.shadow,
    "--ch-hair": palette.hair,
    "--ch-hair-light": palette.hairLight,
    "--ch-tilt": palette.tilt,
  };
}
