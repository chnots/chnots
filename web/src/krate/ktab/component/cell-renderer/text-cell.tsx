import { memo, useState, useRef, useEffect, useCallback } from "react";
import type { CellRendererProps } from "./types";

export const TextCell = memo(function TextCell({
  value,
  readonly,
  isActive,
  onActivate,
  onNavigate,
  onCommit,
}: CellRendererProps) {
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const valueRef = useRef(value);
  valueRef.current = value;

  // Sync editing state with isActive prop
  useEffect(() => {
    if (isActive && !readonly) {
      setDraft(String(valueRef.current ?? ""));
      setEditing(true);
    } else if (!isActive && editing) {
      if (draft !== String(valueRef.current ?? "")) {
        onCommit(draft);
      }
      setEditing(false);
    }
    // Only react to isActive changes
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isActive]);

  useEffect(() => {
    if (editing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [editing]);

  const handleClick = useCallback(() => {
    if (!readonly) onActivate();
  }, [readonly, onActivate]);

  if (readonly || !editing) {
    return (
      <div
        className="px-2 py-1 min-h-[28px] cursor-default"
        onClick={handleClick}
      >
        {String(value ?? "")}
      </div>
    );
  }

  return (
    <input
      ref={inputRef}
      className="w-full h-full px-2 py-1 text-sm bg-transparent border-none outline-none"
      value={draft}
      onChange={(e) => setDraft(e.target.value)}
      onBlur={() => {
        onCommit(draft);
        setEditing(false);
      }}
      onKeyDown={(e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          onCommit(draft);
          setEditing(false);
          onNavigate("down");
        } else if (e.key === "Tab") {
          e.preventDefault();
          onCommit(draft);
          setEditing(false);
          onNavigate(e.shiftKey ? "prev" : "next");
        } else if (e.key === "Escape") {
          onCommit(String(valueRef.current ?? ""));
          setEditing(false);
        }
      }}
    />
  );
});
