import { ArrowUpRight, BookOpen, MessageCircle, UsersRound } from "lucide-react";
import type { CSSProperties } from "react";
import { projectName } from "../lib/characters";
import type { Character, Project } from "../types";
import { CharacterPortrait } from "./CharacterPortrait";

interface CharacterCardProps {
  item: Character;
  index: number;
  projects: Project[];
  onView: (item: Character) => void;
}

export function CharacterCard({ item, index, projects, onView }: CharacterCardProps) {
  const tagList = Array.isArray(item.tags) ? item.tags : [];
  const style = { "--ch-i": index } as CSSProperties;

  return (
    <article className="ch-card surface" style={style}>
      <CharacterPortrait item={item} index={index} />
      <div className="ch-card-body">
        <div className="ch-card-top">
          <span className="ch-role">{item.role || "Character"}</span>
          <span className="ch-age">
            {item.age ? `Age ${item.age}` : "Age unknown"}
          </span>
        </div>
        <h2 title={item.name}>{item.name}</h2>
        <p className="ch-story" title={projectName(item.story, projects)}>
          <BookOpen size={12} />
          {projectName(item.story, projects)}
        </p>
        <div className="ch-tags">
          {tagList.slice(0, 3).map((tag, i) => (
            <span className="ch-tag pill" key={`${tag}-${i}`}>
              {tag}
            </span>
          ))}
        </div>
        <div className="ch-bottom">
          <div className="ch-stats">
            <span title="Relationships">
              <UsersRound size={13} />
              {item.relationships?.length || 0}
            </span>
            <span title="Dialogue scenes">
              <MessageCircle size={13} />
              {item.dialogueCount || 0} scenes
            </span>
          </div>
          <button
            type="button"
            className="ch-view surface-interactive"
            onClick={() => onView(item)}
            aria-label={`View ${item.name}'s character profile`}
          >
            View <ArrowUpRight size={13} />
          </button>
        </div>
      </div>
    </article>
  );
}
