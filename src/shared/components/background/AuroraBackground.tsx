export function AuroraBackground() {
  return (
    <div className="pointer-events-none fixed inset-0 z-[-1] overflow-hidden">
      {/* Organic Base Overlay */}
      <div className="absolute inset-0 bg-white/20 mix-blend-screen dark:bg-[#0a0a0f]/40 dark:mix-blend-multiply transition-colors duration-1000" />

      {/* Blob 1 - Electric Blue */}
      <div className="absolute -top-32 -left-20 h-[42vw] min-h-[340px] w-[42vw] min-w-[340px] rounded-[38%_62%_63%_37%/41%_44%_56%_59%] bg-gradient-to-tr from-blue-400 via-sky-400 to-indigo-300 opacity-60 blur-[60px] animate-chaotic-1 dark:from-blue-700 dark:via-indigo-600 dark:to-cyan-400 dark:opacity-50 lg:blur-[80px] will-change-transform transform-gpu" />

      {/* Blob 2 - Hot Neon Pink */}
      <div className="absolute top-[28%] -right-28 h-[46vw] min-h-[380px] w-[46vw] min-w-[380px] rounded-[58%_42%_33%_67%/53%_55%_45%_47%] bg-gradient-to-bl from-pink-400 via-rose-300 to-pink-200 opacity-60 blur-[65px] animate-chaotic-2 dark:from-pink-600 dark:via-rose-600 dark:to-fuchsia-500 dark:opacity-45 lg:blur-[90px] will-change-transform transform-gpu" />

      {/* Blob 3 - Deep Vivid Purple */}
      <div className="absolute -bottom-28 left-[22%] h-[48vw] min-h-[360px] w-[48vw] min-w-[360px] rounded-[44%_56%_49%_51%/62%_36%_64%_38%] bg-gradient-to-r from-purple-400 via-violet-300 to-indigo-300 opacity-60 blur-[60px] animate-chaotic-3 dark:from-purple-700 dark:via-violet-600 dark:to-fuchsia-700 dark:opacity-45 lg:blur-[80px] will-change-transform transform-gpu" />

      {/* Blob 4 - Dynamic Convergence Node */}
      <div className="absolute left-[30%] top-[40%] h-[32vw] min-h-[260px] w-[32vw] min-w-[260px] rounded-[60%_40%_50%_50%/40%_60%_50%_50%] bg-gradient-to-tr from-sky-300 via-pink-300 to-purple-200 opacity-50 blur-[55px] animate-chaotic-4 dark:from-indigo-500 dark:via-pink-500 dark:to-purple-600 dark:opacity-35 lg:blur-[70px] will-change-transform transform-gpu" />

      {/* Subtle noise texture */}
      <div className="absolute inset-0 bg-[radial-gradient(#000_1px,transparent_1px)] opacity-[0.035] dark:bg-[radial-gradient(#fff_1px,transparent_1px)] dark:opacity-[0.05] [background-size:16px_16px]" />
    </div>
  );
}
