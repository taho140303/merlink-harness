import type { SelectHTMLAttributes } from "react";
import { ChevronDown } from "lucide-react";

export type SelectBoxProps = SelectHTMLAttributes<HTMLSelectElement>;

/**
 * A native `<select>` wearing the project's design language (Sprint 11.13 · D6).
 *
 * The platform arrow is removed (`appearance:none` in `.merlink-select`) and a
 * lucide chevron is drawn instead, so header selects match the glass buttons
 * around them. Behaviour stays 100% native: keyboard, options and `aria-label`
 * are untouched.
 */
export function SelectBox({ className, children, ...props }: SelectBoxProps) {
  return (
    <span className="merlink-select-wrap">
      <select
        className={className ? `merlink-select ${className}` : "merlink-select"}
        {...props}
      >
        {children}
      </select>
      <ChevronDown className="merlink-select-chevron" size={14} aria-hidden="true" />
    </span>
  );
}
