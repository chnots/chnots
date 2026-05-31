import { useState, useRef, useEffect } from "react";
import type { CellRendererProps } from "./types";

export const NumberCell = ({ value, readonly, onCommit }: CellRendererProps) => {
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
        className="px-2 py-1 min-h-[28px] cursor-default tabular-nums"
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
      type="text"
      inputMode="decimal"
      className="w-full h-full px-2 py-1 text-sm bg-transparent border-none outline-none tabular-nums"
      value={draft}
      onChange={(e) => {
        const v = e.target.value;
        if (v === "" || v === "-" || v === "." || /^-?\d*\.?\d*$/.test(v)) {
          setDraft(v);
        }
      }}
      onBlur={() => {
        if (draft !== "" && !isNaN(Number(draft))) {
          onCommit(draft);
        } else if (draft === "") {
          onCommit("");
        }
        setEditing(false);
      }}
      onKeyDown={(e) => {
        if (e.key === "Enter") {
          if (draft !== "" && !isNaN(Number(draft))) {
            onCommit(draft);
          }
          setEditing(false);
        } else if (e.key === "Escape") {
          setEditing(false);
        }
      }}
    />
  );
};
