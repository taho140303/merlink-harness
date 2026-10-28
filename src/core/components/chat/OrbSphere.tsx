export function OrbSphere() {
  return (
    <div aria-hidden="true" className="relative flex items-center justify-center">
      {/* Glow aura phía sau */}
      <div className="animate-orb-glow absolute h-44 w-44 rounded-full bg-linear-to-tr from-fuchsia-400/40 via-sky-300/40 to-indigo-400/30 blur-2xl dark:from-fuchsia-600/30 dark:via-blue-500/20 dark:to-violet-600/30" />

      {/* Quả cầu chính với dải gradient ngọc trai / fluid orb */}
      <div className="animate-orb-float relative h-36 w-36 overflow-hidden rounded-full ring-1 ring-white/60 ">
        <div
          className="absolute inset-0 rounded-full"
          style={{
            background:
              "radial-gradient(circle at 35% 30%, #ffffff 0%, #e0e7ff 18%, #93c5fd 35%, #c084fc 65%, #f472b6 85%, #6366f1 100%)",
          }}
        />

        {/* Lớp ánh sáng phản chiếu uốn lượn */}
        <div
          className="absolute inset-1 rounded-full opacity-75 mix-blend-overlay"
          style={{
            background:
              "conic-gradient(from 140deg at 50% 50%, #ffffff 0deg, #38bdf8 110deg, #ec4899 220deg, #ffffff 360deg)",
            filter: "blur(4px)",
          }}
        />

        {/* Vệt bóng highlight trên đỉnh cầu */}
        <div className="absolute top-2 left-6 h-12 w-16 -rotate-25 rounded-full bg-white/60 blur-xs" />
      </div>
    </div>
  );
}
