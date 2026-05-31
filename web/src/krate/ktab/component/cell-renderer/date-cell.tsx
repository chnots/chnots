import { useState } from "react";
import { CalendarIcon } from "lucide-react";
import { Calendar } from "@/common/component/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import type { CellRendererProps } from "./types";

export const DateCell = ({
  value,
  columnMeta,
  readonly,
  onCommit,
}: CellRendererProps) => {
  const [open, setOpen] = useState(false);
  const isDatetime = columnMeta.view_kind === "datetime";

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

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <div className="flex items-center gap-1 px-2 py-1 min-h-[28px] cursor-pointer hover:bg-accent/30">
          <CalendarIcon className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
          <span className={display ? "" : "text-muted-foreground"}>
            {display || "选择日期"}
          </span>
        </div>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-0" align="start">
        <Calendar
          mode="single"
          selected={dateValue}
          onSelect={(d) => {
            if (d) {
              onCommit(isDatetime ? d : new Date(d.toISOString().split("T")[0] + "T00:00:00.000Z"));
            }
            setOpen(false);
          }}
          autoFocus
        />
      </PopoverContent>
    </Popover>
  );
};
