import type { ReactNode } from "react";

interface ComingSoonProps {
  title: string;
  description: string;
  note: string;
  icon: ReactNode;
}

/**
 * Shared placeholder for pages scheduled for a later sprint.
 * Uses the shared.css `surface` card so it matches the rest of the app shell.
 */
export function ComingSoon({ title, description, note, icon }: ComingSoonProps) {
  return (
    <div className="surface" style={{ padding: "34px 30px", maxWidth: "560px" }}>
      <div
        style={{
          display: "inline-flex",
          alignItems: "center",
          justifyContent: "center",
          width: "46px",
          height: "46px",
          borderRadius: "15px",
          background: "var(--accent-soft)",
          color: "var(--accent-strong)",
          marginBottom: "16px",
        }}
      >
        {icon}
      </div>
      <h2
        style={{
          margin: "0 0 8px",
          fontSize: "20px",
          fontWeight: 700,
          letterSpacing: "-.03em",
        }}
      >
        {title}
      </h2>
      <p
        style={{
          margin: "0 0 14px",
          color: "var(--muted)",
          fontSize: "13.5px",
          lineHeight: 1.65,
        }}
      >
        {description}
      </p>
      <span
        style={{
          display: "inline-flex",
          alignItems: "center",
          gap: "6px",
          padding: "5px 11px",
          borderRadius: "10px",
          background: "var(--glass-soft)",
          border: "1px solid var(--line)",
          color: "var(--muted)",
          fontSize: "11px",
          fontWeight: 650,
        }}
      >
        {note}
      </span>
    </div>
  );
}
