import type { CSSProperties } from "react";

export interface ProjectCoverProps {
  projectId: string;
  title: string;
  genres?: string[];
  variant?: "card" | "detail";
}

const palettes = [
  "linear-gradient(135deg, #8d829f, #dbc7b8 56%, #333d6f)",
  "linear-gradient(145deg, #183e65, #5ca6a3 56%, #d4ad7c)",
  "linear-gradient(135deg, #97647d, #d7a1a6 55%, #394562)",
  "linear-gradient(135deg, #343f70, #888ac1 56%, #d9b5a8)",
  "linear-gradient(135deg, #526774, #b4bbb3 52%, #bea486)",
  "linear-gradient(135deg, #435970, #88a4ad 54%, #efbd95)",
];

function hashString(value: string): number {
  let hash = 0;
  for (let index = 0; index < value.length; index += 1) hash = (Math.imul(hash, 31) + value.charCodeAt(index)) | 0;
  return hash >>> 0;
}

export function ProjectCover({ projectId, title, genres, variant = "card" }: ProjectCoverProps) {
  const caption = variant === "detail" ? title : (genres?.[0] ?? title);
  return (
    <div
      className={`project-cover project-cover--${variant}`}
      role="img"
      aria-label={`Cover artwork for ${title}`}
      style={{ background: palettes[hashString(projectId) % palettes.length] } as CSSProperties}
    >
      <span className="project-cover-label">MERLINK · STORY FILE</span>
      <span className="project-cover-caption">{caption}</span>
    </div>
  );
}
