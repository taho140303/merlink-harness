import { Sparkles } from "lucide-react";
import { assetIsPhoto, portraitStyle } from "../lib/characters";
import type { Character } from "../types";

interface CharacterPortraitProps {
  item: Character;
  index: number;
  label?: string;
}

/** Illustrated silhouette or uploaded photo, ported from the ref-ui `Portrait` component. */
export function CharacterPortrait({ item, index, label }: CharacterPortraitProps) {
  return (
    <div
      className={
        "ch-portrait" + (assetIsPhoto(item) ? " ch-portrait--uploaded" : "")
      }
      style={portraitStyle(item, index)}
      role="img"
      aria-label={`Illustrated portrait of ${item.name}`}
    >
      <span className="ch-art-halo" />
      <span className="ch-art-figure" />
      <span className="ch-art-neck" />
      <span className="ch-art-head" />
      <span className="ch-art-hair" />
      <span className="ch-portrait-mark">{label || "Character file"}</span>
      <span className="ch-portrait-num">
        {String(index + 1).padStart(2, "0")} / ARCHIVE
      </span>
      <span className="ch-portrait-name">{item.name}</span>
      <span className="ch-portrait-seal" aria-hidden="true">
        <Sparkles size={15} />
      </span>
    </div>
  );
}
