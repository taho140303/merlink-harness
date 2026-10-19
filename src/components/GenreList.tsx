interface GenreListProps {
  genres: string[];
  columns?: number;
  maxRows?: number;
}

export function GenreList({ genres, columns = 3, maxRows = 2 }: GenreListProps) {
  if (genres.length === 0) return <span className="project-genre-empty">No genres yet</span>;
  const budget = Math.max(1, columns * maxRows);
  const overflow = genres.length > budget;
  const shown = overflow ? genres.slice(0, budget - 1) : genres;
  const hidden = overflow ? genres.slice(budget - 1) : [];
  return (
    <div className="project-genre-grid" style={{ "--genre-columns": columns } as React.CSSProperties}>
      {shown.map((genre, index) => <span className="project-genre-chip" key={`${genre}-${index}`} title={genre}>{genre}</span>)}
      {overflow && <span className="project-genre-chip project-genre-overflow" title={hidden.join(" · ")} aria-label={`More genres: ${hidden.join(" · ")}`}>...</span>}
    </div>
  );
}
