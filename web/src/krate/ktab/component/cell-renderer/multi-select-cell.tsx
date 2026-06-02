import { memo, useState, useMemo, useEffect } from "react";
import { CheckIcon, ChevronDownIcon, PlusIcon } from "lucide-react";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import { Badge } from "@/common/component/ui/badge";
import type { CellRendererProps } from "./types";

function parseTags(value: unknown): string[] {
  if (Array.isArray(value)) return value.filter((v) => typeof v === "string");
  if (typeof value === "string") {
    if (!value) return [];
    try {
      const parsed = JSON.parse(value);
      if (Array.isArray(parsed))
        return parsed.filter((v) => typeof v === "string");
    } catch {
      return value
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean);
    }
  }
  return [];
}

function getOptions(comment: string): string[] {
  return comment
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

export const MultiSelectCell = memo(function MultiSelectCell({
  value,
  columnMeta,
  readonly,
  isActive,
  onActivate,
  onNavigate,
  onCommit,
  onColumnChange,
}: CellRendererProps) {
  const [open, setOpen] = useState(false);
  const [draft, setDraft] = useState<string[]>([]);
  const [newOption, setNewOption] = useState("");
  const [localAdds, setLocalAdds] = useState<string[]>([]);

  // Sync popover with isActive
  useEffect(() => {
    if (isActive && !readonly) {
      setDraft([...parseTags(value)]);
      setOpen(true);
    } else if (!isActive) {
      setOpen(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isActive]);

  const propOptions = useMemo(
    () => getOptions(columnMeta.comment),
    [columnMeta.comment],
  );

  const pendingOptions = useMemo(
    () => localAdds.filter((a) => !propOptions.includes(a)),
    [localAdds, propOptions],
  );

  const options = useMemo(
    () => [...propOptions, ...pendingOptions],
    [propOptions, pendingOptions],
  );

  const selected = useMemo(() => parseTags(value), [value]);
  const active = open ? draft : selected;

  const toggle = (tag: string) => {
    setDraft((prev) =>
      prev.includes(tag) ? prev.filter((s) => s !== tag) : [...prev, tag],
    );
  };

  const handleOpenChange = (nextOpen: boolean) => {
    if (nextOpen) {
      setDraft([...selected]);
    } else {
      const changed =
        draft.length !== selected.length ||
        draft.some((t) => !selected.includes(t));
      if (changed) {
        onCommit(draft);
      }
    }
    setOpen(nextOpen);
  };

  const addOption = () => {
    const name = newOption.trim();
    if (!name || propOptions.includes(name) || pendingOptions.includes(name))
      return;
    const updated = [...propOptions, ...pendingOptions, name].join(",");
    onColumnChange?.({ comment: updated });
    setLocalAdds((prev) => [...prev, name]);
    setNewOption("");
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Tab") {
      e.preventDefault();
      handleOpenChange(false);
      onNavigate(e.shiftKey ? "prev" : "next");
    } else if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleOpenChange(false);
      onNavigate("down");
    }
  };

  if (readonly) {
    return (
      <div className="flex flex-wrap gap-1 px-1 py-0.5 min-h-[28px]">
        {selected.map((tag) => (
          <Badge key={tag} variant="secondary" className="text-xs">
            {tag}
          </Badge>
        ))}
      </div>
    );
  }

  return (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <div
          className="flex items-center gap-1 px-1 py-0.5 min-h-[28px] cursor-pointer hover:bg-accent/30"
          onClick={() => onActivate()}
          onKeyDown={handleKeyDown}
        >
          <div className="flex flex-wrap gap-1 flex-1 min-w-0">
            {selected.length === 0 && (
              <span className="text-muted-foreground text-xs px-1">—</span>
            )}
            {selected.map((tag) => (
              <Badge key={tag} variant="secondary" className="text-xs">
                {tag}
              </Badge>
            ))}
          </div>
          <ChevronDownIcon className="h-3 w-3 text-muted-foreground shrink-0" />
        </div>
      </PopoverTrigger>
      <PopoverContent
        className="w-52 p-1"
        align="start"
        onKeyDown={handleKeyDown}
      >
        {options.length > 0 && (
          <div className="max-h-48 overflow-auto">
            {options.map((opt) => {
              const checked = active.includes(opt);
              return (
                <button
                  key={opt}
                  className="flex items-center gap-2 w-full px-2 py-1.5 text-sm rounded-sm hover:bg-accent/50 text-left"
                  onClick={() => toggle(opt)}
                >
                  <span className="h-4 w-4 shrink-0 flex items-center justify-center">
                    {checked && <CheckIcon className="h-3.5 w-3.5" />}
                  </span>
                  {opt}
                </button>
              );
            })}
          </div>
        )}
        <div className="flex items-center gap-1 border-t mt-1 pt-1 px-1">
          <input
            className="flex-1 text-sm bg-transparent outline-none placeholder:text-muted-foreground min-w-0"
            placeholder="添加选项..."
            value={newOption}
            onChange={(e) => setNewOption(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                addOption();
              }
            }}
          />
          <button
            className="shrink-0 p-0.5 hover:bg-accent/50 rounded"
            onClick={addOption}
          >
            <PlusIcon className="h-3.5 w-3.5" />
          </button>
        </div>
      </PopoverContent>
    </Popover>
  );
});
