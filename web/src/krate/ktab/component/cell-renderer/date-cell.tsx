import { memo, useState, useEffect } from "react";
import { CalendarIcon } from "lucide-react";
import { Calendar } from "@/common/component/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import type { CellRendererProps } from "./types";

const pad2 = (n: number) => String(n).padStart(2, "0");

export const DateCell = memo(function DateCell({
  value,
  columnMeta,
  readonly,
  isActive,
  onActivate,
  onNavigate,
  onCommit,
}: CellRendererProps) {
  const [open, setOpen] = useState(false);
  const isDatetime = columnMeta.view_kind === "datetime";

  // Sync popover open state with isActive
  useEffect(() => {
    if (isActive && !readonly) {
      setOpen(true);
    } else if (!isActive) {
      setOpen(false);
    }
  }, [isActive, readonly]);

  let dateValue: Date | undefined;
  if (value instanceof Date) {
    dateValue = value;
  } else if (typeof value === "string" && value) {
    dateValue = new Date(value);
    if (isNaN(dateValue.getTime())) dateValue = undefined;
  }

  const display = dateValue
    ? isDatetime
      ? dateValue.toLocaleString("zh-CN")
      : dateValue.toLocaleDateString("zh-CN")
    : "";

  if (readonly) {
    return <div className="px-2 py-1 min-h-[28px]">{display}</div>;
  }

  const handleDateSelect = (d: Date | undefined) => {
    if (!d) return;
    if (!isDatetime) {
      onCommit(new Date(d.toISOString().split("T")[0] + "T00:00:00.000Z"));
      setOpen(false);
      return;
    }
    const time = dateValue ?? new Date();
    const next = new Date(
      d.getFullYear(),
      d.getMonth(),
      d.getDate(),
      time.getHours(),
      time.getMinutes(),
      time.getSeconds(),
    );
    onCommit(next);
  };

  const handleTimeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const [h, m] = e.target.value.split(":").map(Number);
    const base = dateValue ?? new Date();
    const next = new Date(
      base.getFullYear(),
      base.getMonth(),
      base.getDate(),
      h ?? 0,
      m ?? 0,
      base.getSeconds(),
    );
    onCommit(next);
  };

  const timeValue = dateValue
    ? `${pad2(dateValue.getHours())}:${pad2(dateValue.getMinutes())}`
    : "00:00";

  const handleOpenChange = (nextOpen: boolean) => {
    setOpen(nextOpen);
    if (!nextOpen) {
      // Popover was dismissed, navigate away if this cell is still active
      // The isActive will be cleared by the parent
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Tab") {
      e.preventDefault();
      setOpen(false);
      onNavigate(e.shiftKey ? "prev" : "next");
    } else if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      setOpen(false);
      onNavigate("down");
    }
  };

  return (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <div
          className="flex items-center gap-1 px-2 py-1 min-h-[28px] cursor-pointer hover:bg-accent/30"
          onClick={() => onActivate()}
          onKeyDown={handleKeyDown}
        >
          <CalendarIcon className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
          <span className={display ? "" : "text-muted-foreground"}>
            {display || (isDatetime ? "选择日期时间" : "选择日期")}
          </span>
        </div>
      </PopoverTrigger>
      <PopoverContent
        className="w-auto p-0"
        align="start"
        onKeyDown={handleKeyDown}
      >
        <Calendar
          mode="single"
          selected={dateValue}
          onSelect={handleDateSelect}
          autoFocus
        />
        {isDatetime && (
          <div className="border-t px-3 py-2 flex items-center gap-2">
            <span className="text-xs text-muted-foreground">时间</span>
            <input
              type="time"
              value={timeValue}
              onChange={handleTimeChange}
              className="border rounded px-2 py-0.5 text-sm bg-background"
            />
          </div>
        )}
      </PopoverContent>
    </Popover>
  );
});
