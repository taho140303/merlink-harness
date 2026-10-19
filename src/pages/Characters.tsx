import { useEffect, useMemo, useState } from "react";
import { CheckCircle2, ChevronDown, Plus, Search, UsersRound } from "lucide-react";
import { CharacterCard } from "../components/CharacterCard";
import { CharacterDetail } from "../components/CharacterDetail";
import { CharacterForm } from "../components/CharacterForm";
import { Dialog } from "../components/Dialog";
import { mockCharacters, mockProjects } from "../data/mock";
import { STORAGE_KEY, projectName, readCustomCharacters } from "../lib/characters";
import type { Character } from "../types";
import "../styles/characters.css";

export function Characters() {
  const projects = mockProjects;
  const baseCharacters = mockCharacters;

  const [custom, setCustom] = useState<Character[]>(readCustomCharacters);
  const [search, setSearch] = useState("");
  const [story, setStory] = useState("all");
  const [genre, setGenre] = useState("all");
  const [selected, setSelected] = useState<Character | null>(null);
  const [creating, setCreating] = useState(false);
  const [toast, setToast] = useState("");

  const allCharacters = useMemo(
    () => [...baseCharacters, ...custom],
    [baseCharacters, custom],
  );

  const genres = useMemo(
    () =>
      [...new Set(projects.map((item) => item.genre).filter(Boolean))].sort((a, b) =>
        a.localeCompare(b),
      ),
    [projects],
  );

  const visible = useMemo(
    () =>
      allCharacters.filter((item) => {
        const matchStory =
          story === "all" ||
          String(item.story) === story ||
          projectName(item.story, projects) === story;
        const linkedProject = projects.find(
          (project) =>
            String(project.id) === String(item.story) || project.title === item.story,
        );
        const matchGenre = genre === "all" || linkedProject?.genre === genre;
        const q = search.trim().toLowerCase();
        const matchSearch =
          !q ||
          [
            item.name,
            item.role,
            projectName(item.story, projects),
            ...(item.tags || []),
          ]
            .join(" ")
            .toLowerCase()
            .includes(q);
        return matchStory && matchGenre && matchSearch;
      }),
    [allCharacters, story, genre, search, projects],
  );

  useEffect(() => {
    if (!toast) return;
    const timer = setTimeout(() => setToast(""), 3500);
    return () => clearTimeout(timer);
  }, [toast]);

  function create(item: Character) {
    const next = [item, ...custom];
    setCustom(next);
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
      setToast(`${item.name} has joined your character archive.`);
    } catch {
      setToast(`${item.name} was added for this session. Image storage is full.`);
    }
    setCreating(false);
    setStory("all");
    setGenre("all");
    setSearch("");
  }

  return (
    <main className="ch-page">
      <div className="page-topbar">
        <span className="page-breadcrumb">
          Workspace / <strong>Characters</strong>
        </span>
        <div className="page-topbar-actions">
          <button
            type="button"
            className="ch-create btn btn-primary"
            onClick={() => setCreating(true)}
          >
            <Plus size={16} /> New character
          </button>
        </div>
      </div>

      <section className="ch-controls" aria-label="Character filters">
        <div className="ch-search">
          <Search size={16} />
          <input
            aria-label="Search characters"
            type="search"
            placeholder="Search names, traits, or stories..."
            value={search}
            onChange={(event) => setSearch(event.target.value)}
          />
        </div>
        <div className="ch-select-wrap">
          <select
            aria-label="Filter by story"
            value={story}
            onChange={(event) => setStory(event.target.value)}
          >
            <option value="all">All stories</option>
            {projects.map((project) => (
              <option key={project.id} value={String(project.id)}>
                {project.title}
              </option>
            ))}
          </select>
          <ChevronDown size={14} />
        </div>
        <div className="ch-select-wrap">
          <select
            aria-label="Filter by genre"
            value={genre}
            onChange={(event) => setGenre(event.target.value)}
          >
            <option value="all">All genres</option>
            {genres.map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </select>
          <ChevronDown size={14} />
        </div>
      </section>

      <div className="ch-collection-line">
        <span className="ch-smallcaps">The collection</span>
        <span className="ch-rule" />
        <span>
          {visible.length} {visible.length === 1 ? "character" : "characters"}{" "}
          {visible.length !== allCharacters.length ? "found" : "on record"}
        </span>
      </div>

      {visible.length ? (
        <section className="ch-grid" aria-label="Character profiles">
          {visible.map((item) => (
            <CharacterCard
              key={item.id}
              item={item}
              index={allCharacters.indexOf(item)}
              projects={projects}
              onView={setSelected}
            />
          ))}
        </section>
      ) : (
        <div className="ch-empty">
          <div className="ch-empty-icon">
            <UsersRound size={25} />
          </div>
          <h2>No characters found</h2>
          <p>
            {allCharacters.length
              ? "Try another search or clear your filters to see your whole cast."
              : "Your archive is waiting for its first story. Create a character to begin."}
          </p>
          <button
            type="button"
            onClick={() => {
              setSearch("");
              setStory("all");
              setGenre("all");
              if (!allCharacters.length) setCreating(true);
            }}
          >
            {allCharacters.length ? "Clear filters" : "Create a character"}
          </button>
        </div>
      )}

      <Dialog
        open={Boolean(selected)}
        onClose={() => setSelected(null)}
        title="Character profile"
        wide
      >
        {selected && (
          <CharacterDetail
            item={selected}
            index={allCharacters.indexOf(selected)}
            projects={projects}
          />
        )}
      </Dialog>

      <Dialog
        open={creating}
        onClose={() => setCreating(false)}
        title="Create a character"
        wide
      >
        {creating && (
          <CharacterForm
            projects={projects}
            onClose={() => setCreating(false)}
            onCreate={create}
          />
        )}
      </Dialog>

      {toast && (
        <div className="ch-toast" role="status">
          <CheckCircle2 size={18} />
          {toast}
        </div>
      )}
    </main>
  );
}
