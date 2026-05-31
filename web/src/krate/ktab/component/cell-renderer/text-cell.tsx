import { useState, useRef, useEffect } from "react";
import type { CellRendererProps } from "./types";

export const TextCell = ({ value, readonly, onCommit }: CellRendererProps) => {
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (editing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [editing]);

  if (readonly || !editing) {
    return (
      <div
        className="px-2 py-1 min-h-[28px] cursor-default"
        onClick={() => {
          if (!readonly) {
            setDraft(String(value ?? ""));
            setEditing(true);
          }
        }}
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
          onCommit(draft);
          setEditing(false);
        } else if (e.key === "Escape") {
          setEditing(false);
        }
      }}
    />
  );
};
