import { useEffect, useRef, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { X } from "lucide-react";

export interface DialogProps {
  open: boolean;
  onClose: () => void;
  title: string;
  wide?: boolean;
  children: ReactNode;
}

/**
 * Shared modal, ported from the `Dialog` component in ref-ui/shared.js:
 * backdrop click, Escape to close, focus trap and focus restoration.
 */
export function Dialog({ open, onClose, title, wide = false, children }: DialogProps) {
  const panelRef = useRef<HTMLElement>(null);
  const closeRef = useRef<HTMLButtonElement>(null);
  const onCloseRef = useRef(onClose);
  onCloseRef.current = onClose;

  useEffect(() => {
    if (!open) return;

    const previousOverflow = document.body.style.overflow;
    const previousFocus = document.activeElement as HTMLElement | null;
    document.body.style.overflow = "hidden";
    (closeRef.current || panelRef.current)?.focus();

    function onKeyDown(event: KeyboardEvent) {
      const panel = panelRef.current;
      if (!panel) return;
      // Only the top-most dialog reacts to keyboard shortcuts.
      const dialogs = document.querySelectorAll(".merlink-dialog-backdrop");
      if (dialogs[dialogs.length - 1] !== panel.parentElement) return;

      if (event.key === "Escape") {
        event.preventDefault();
        onCloseRef.current?.();
        return;
      }
      if (event.key !== "Tab") return;

      const selectors =
        'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';
      const focusable = [...panel.querySelectorAll<HTMLElement>(selectors)].filter(
        (element) =>
          element.getClientRects().length &&
          getComputedStyle(element).visibility !== "hidden",
      );
      if (!focusable.length) {
        event.preventDefault();
        panel.focus();
        return;
      }
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (!panel.contains(document.activeElement)) {
        event.preventDefault();
        (event.shiftKey ? last : first).focus();
      } else if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    }

    document.addEventListener("keydown", onKeyDown);
    return () => {
      document.body.style.overflow = previousOverflow;
      document.removeEventListener("keydown", onKeyDown);
      if (previousFocus?.isConnected && typeof previousFocus.focus === "function") {
        previousFocus.focus();
      }
    };
  }, [open]);

  if (!open) return null;

  return createPortal(
    <div
      className="merlink-dialog-backdrop"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) onCloseRef.current?.();
      }}
    >
      <section
        ref={panelRef}
        tabIndex={-1}
        className={`dialog-panel${wide ? " merlink-dialog-panel-wide" : ""}`}
        role="dialog"
        aria-modal="true"
        aria-label={title || "Dialog"}
      >
        <div className="merlink-dialog-head">
          <h2>{title}</h2>
          <button
            ref={closeRef}
            type="button"
            className="btn merlink-dialog-close"
            onClick={() => onCloseRef.current?.()}
            aria-label="Close dialog"
          >
            <X size={17} />
          </button>
        </div>
        <div className="merlink-dialog-body">{children}</div>
      </section>
    </div>,
    document.body,
  );
}
