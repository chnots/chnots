import { memo, useEffect, useRef } from "react";
import type { CellRendererProps } from "./types";

export const CheckboxCell = memo(function CheckboxCell({
  value,
  onCommit,
  readonly,
  isActive,
  onNavigate,
}: CellRendererProps) {
  const checked = !!value && value !== 0 && value !== "0" && value !== "false";
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isActive && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isActive]);

  return (
    <div className="flex items-center justify-center w-full min-h-[28px]">
      <input
        ref={inputRef}
        type="checkbox"
        checked={checked}
        disabled={readonly}
        onChange={(e) => {
          onCommit(e.target.checked);
        }}
        onKeyDown={(e) => {
          if (e.key === "Tab") {
            e.preventDefault();
            onNavigate(e.shiftKey ? "prev" : "next");
          } else if (e.key === "Enter") {
            e.preventDefault();
            onNavigate("down");
          }
        }}
        className="h-4 w-4 rounded border-border accent-primary cursor-pointer"
      />
    </div>
  );
});
