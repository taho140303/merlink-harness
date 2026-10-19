import { projectName } from "../lib/characters";
import type { Character, Project } from "../types";
import { CharacterPortrait } from "./CharacterPortrait";

interface CharacterDetailProps {
  item: Character;
  index: number;
  projects: Project[];
}

export function CharacterDetail({ item, index, projects }: CharacterDetailProps) {
  const relationships = Array.isArray(item.relationships) ? item.relationships : [];

  return (
    <div className="ch-dialog-body ch-detail">
      <CharacterPortrait item={item} index={index} label="Character portrait" />
      <div className="ch-detail-content">
        <div className="ch-detail-overline">
          <span className="ch-smallcaps">The character archive</span>
          <span className="ch-detail-index">
            PROFILE NO. {String(index + 1).padStart(2, "0")}
          </span>
        </div>
        <h2>{item.name}</h2>
        <p className="ch-detail-sub">
          {item.role || "Character"} · {projectName(item.story, projects)}
        </p>
        <div className="ch-detail-tags">
          {(item.tags || []).map((tag, i) => (
            <span className="ch-tag pill" key={`${tag}-${i}`}>
              {tag}
            </span>
          ))}
        </div>
        <hr className="ch-detail-rule" />
        <div className="ch-detail-facts">
          <div className="ch-detail-fact">
            <strong>{item.age || "—"}</strong>
            <span>Age</span>
          </div>
          <div className="ch-detail-fact">
            <strong>{item.gender || "—"}</strong>
            <span>Gender</span>
          </div>
          <div className="ch-detail-fact">
            <strong>{item.dialogueCount || 0}</strong>
            <span>Dialogue scenes</span>
          </div>
        </div>
        <section className="ch-detail-block">
          <h3>At a glance</h3>
          <p>
            {item.bio ||
              item.appearance ||
              "This character is waiting for their story to unfold."}
          </p>
        </section>
        {item.appearance && (
          <section className="ch-detail-block">
            <h3>Appearance</h3>
            <p>{item.appearance}</p>
          </section>
        )}
        <section className="ch-detail-block">
          <h3>Backstory</h3>
          <p>{item.backstory || "Their past is still being written."}</p>
        </section>
        <section className="ch-detail-block">
          <h3>Connections · {relationships.length}</h3>
          {relationships.length ? (
            <ul className="ch-relationship-list">
              {relationships.map((relationship, i) => (
                <li key={i}>
                  <strong>
                    {typeof relationship === "string"
                      ? relationship
                      : relationship.name}
                  </strong>
                  <span>
                    {typeof relationship === "string"
                      ? "Connection"
                      : relationship.relation || "Connection"}
                  </span>
                </li>
              ))}
            </ul>
          ) : (
            <p>
              No relationships mapped yet. Every great story begins with a
              connection.
            </p>
          )}
        </section>
      </div>
    </div>
  );
}
