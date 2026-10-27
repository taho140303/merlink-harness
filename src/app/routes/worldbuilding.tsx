import { useParams } from "react-router";

export function meta() {
  return [{ title: "Merlink Harness - Worldbuilding" }];
}

export default function WorldbuildingPage() {
  const { sub } = useParams();

  return (
    <div className="flex h-full min-h-[85vh] flex-col items-center justify-center p-6 text-center">
      <div className="w-full max-w-sm rounded-2xl border border-slate-200 bg-white p-8 shadow-xs dark:border-slate-800 dark:bg-slate-950">
        <span className="rounded-md bg-slate-100 px-2 py-1 text-[11px] font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">
          Under Construction
        </span>
        <h2 className="mt-3 text-lg font-bold tracking-tight text-slate-800 capitalize dark:text-slate-100">
          Worldbuilding: {sub}
        </h2>
        <p className="mt-1.5 text-xs text-slate-500 dark:text-slate-400">
          Route: <code className="font-mono font-medium text-slate-700 dark:text-slate-200">/worldbuilding/{sub}</code>
        </p>
      </div>
    </div>
  );
}
