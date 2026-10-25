import { OrbSphere } from "./OrbSphere";

export function ChatWelcome() {
  return (
    <div className="flex h-full min-h-[55vh] flex-col items-center justify-center text-center select-none">
      <div className="flex flex-col items-center space-y-6">
        <OrbSphere />
        <h1 className="text-xl font-semibold tracking-tight text-slate-800 dark:text-slate-100 sm:text-2xl">
          Tell me about the world you want to build
        </h1>
      </div>
    </div>
  );
}
