import { FileText, MoreHorizontal, Calendar, Terminal, Network, StickyNote } from "lucide-react";

export function ChatWelcome() {
  return (
    <div className="mx-auto flex w-full max-w-4xl flex-col items-center justify-center space-y-8 pt-10 pb-6 text-center">
      {/* Central Orb & Greeting */}
      <div className="space-y-3">
        <div className="mx-auto h-12 w-12 rounded-full bg-linear-to-b from-blue-400 to-blue-600 shadow-lg shadow-blue-500/25 ring-4 ring-blue-100 dark:ring-blue-950/50" />
        <h1 className="text-2xl font-bold tracking-tight text-slate-900 dark:text-slate-100">
          Hi, there 👋
        </h1>
        <p className="text-xs text-slate-500 dark:text-slate-400">
          Tell us what you need, and we'll handle the rest.
        </p>
      </div>

      {/* Suggestion Cards Container */}
      <div className="grid w-full grid-cols-1 gap-4 text-left sm:grid-cols-3">
        {/* Card 1: Black Assistant Card */}
        <div className="flex flex-col justify-between rounded-2xl bg-slate-900 p-4 text-white shadow-sm dark:bg-slate-800/90 dark:border dark:border-slate-700/60">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <div className="flex h-6 w-6 items-center justify-center rounded-full bg-slate-700 text-[10px] font-bold">
                S
              </div>
              <span className="text-xs font-semibold">Sam Lee</span>
            </div>
            <span className="rounded-md bg-blue-600/90 px-2 py-0.5 text-[10px] font-medium text-white">
              Data Assistant
            </span>
          </div>
          <p className="mt-4 text-xs leading-relaxed text-slate-300">
            Designed to help manage sales processes and maximize customer engagement.
          </p>
        </div>

        {/* Card 2: Tasks List */}
        <div className="flex flex-col justify-between rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
          <div className="space-y-2.5">
            <div className="flex items-center gap-2 text-xs text-slate-700 dark:text-slate-300">
              <FileText className="h-3.5 w-3.5 text-slate-400" />
              <span className="truncate">Answer RFP documentation</span>
            </div>
            <div className="flex items-center gap-2 text-xs text-slate-700 dark:text-slate-300">
              <FileText className="h-3.5 w-3.5 text-slate-400" />
              <span className="truncate">Conduct a competitor analysis</span>
            </div>
            <div className="flex items-center gap-2 text-xs text-slate-700 dark:text-slate-300">
              <FileText className="h-3.5 w-3.5 text-slate-400" />
              <span className="truncate">Provide feedback on communication</span>
            </div>
          </div>
          <div className="mt-4 flex items-center justify-between border-t border-slate-100 pt-2 text-[10px] font-medium text-slate-400 dark:border-slate-800">
            <span>Tasks</span>
            <button type="button" className="text-blue-600 hover:underline dark:text-blue-400">
              View All
            </button>
          </div>
        </div>

        {/* Card 3: Suggested prompt */}
        <div className="flex flex-col justify-between rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
          <div className="flex items-start justify-between gap-2">
            <p className="text-xs leading-relaxed text-slate-700 dark:text-slate-300">
              What are the key benefits of Product 1 that I should highlight to potential clients?
            </p>
            <MoreHorizontal className="h-4 w-4 shrink-0 text-slate-400" />
          </div>
          <div className="mt-4 border-t border-slate-100 pt-2 text-[10px] font-medium text-slate-400 dark:border-slate-800">
            Suggested prompt
          </div>
        </div>
      </div>

      {/* Pill Actions */}
      <div className="flex flex-wrap items-center justify-center gap-2.5">
        <button
          type="button"
          className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-3.5 py-1.5 text-xs font-medium text-slate-700 shadow-xs hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
        >
          <Calendar className="h-3.5 w-3.5 text-rose-500" />
          <span>Connect Calendar</span>
        </button>
        <button
          type="button"
          className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-3.5 py-1.5 text-xs font-medium text-slate-700 shadow-xs hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
        >
          <Terminal className="h-3.5 w-3.5 text-blue-500" />
          <span>Demo Task</span>
        </button>
        <button
          type="button"
          className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-3.5 py-1.5 text-xs font-medium text-slate-700 shadow-xs hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
        >
          <Network className="h-3.5 w-3.5 text-amber-500" />
          <span>Browse Integrations</span>
        </button>
        <button
          type="button"
          className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-3.5 py-1.5 text-xs font-medium text-slate-700 shadow-xs hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
        >
          <StickyNote className="h-3.5 w-3.5 text-emerald-500" />
          <span>Shared in Notes</span>
        </button>
      </div>
    </div>
  );
}
