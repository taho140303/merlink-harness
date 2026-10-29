import { useParams } from "react-router";

export function WorldbuildingView() {
  const { sub } = useParams();

  return (
    <div className="flex h-full min-h-[85vh] flex-col items-center justify-center p-6 text-center">
      <div className="w-full max-w-sm rounded-2xl border border-line bg-surface-card p-8 shadow-xs">
        <span className="rounded-md bg-surface-muted px-2 py-1 text-[11px] font-medium text-fg-muted">
          Under Construction
        </span>
        <h2 className="mt-3 text-lg font-bold tracking-tight text-fg capitalize">
          Worldbuilding: {sub}
        </h2>
        <p className="mt-1.5 text-xs text-fg-muted">
          Route: <code className="font-mono font-medium text-fg">/worldbuilding/{sub}</code>
        </p>
      </div>
    </div>
  );
}
