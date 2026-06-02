import { memo, useState, useRef, useEffect, useCallback } from "react";
import type { CellRendererProps } from "./types";

export const NumberCell = memo(function NumberCell({
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

  useEffect(() => {
    if (isActive && !readonly) {
      setDraft(String(valueRef.current ?? ""));
      setEditing(true);
    } else if (!isActive && editing) {
      if (draft !== String(valueRef.current ?? "")) {
        commitValue(draft);
      }
      setEditing(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isActive]);

  useEffect(() => {
    if (editing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [editing]);

  const commitValue = (v: string) => {
    if (v !== "" && !isNaN(Number(v))) {
      onCommit(v);
    } else if (v === "") {
      onCommit("");
    }
  };

  const handleClick = useCallback(() => {
    if (!readonly) onActivate();
  }, [readonly, onActivate]);

  if (readonly || !editing) {
    return (
      <div
        className="px-2 py-1 min-h-[28px] cursor-default tabular-nums"
        onClick={handleClick}
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
        commitValue(draft);
        setEditing(false);
      }}
      onKeyDown={(e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          commitValue(draft);
          setEditing(false);
          onNavigate("down");
        } else if (e.key === "Tab") {
          e.preventDefault();
          commitValue(draft);
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
