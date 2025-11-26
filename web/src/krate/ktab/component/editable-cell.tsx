"use client";

import type { CellContext, ColumnDef } from "@tanstack/react-table";
import { format } from "date-fns";
import React from "react";
import { Button } from "@/common/component/ui/button";
import { Calendar } from "@/common/component/ui/calendar";
import { Input } from "@/common/component/ui/input";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import { cn } from "@/lib/utils";

export type KTabRowData = {
  [key: string]: string | number | Date | null;
};

export type EditableCellProps = {
  getValue: () => any;
  column: {
    id: string;
    columnDef: ColumnDef<KTabRowData>;
  };
  table: any; // Table<KTabRowData>;
  ktabProps: {
    dataType?: "string" | "date" | "decimal";
    updateData: (rowIndex: number, columnId: string, value: any) => void;
  };
} & CellContext<KTabRowData, unknown>;

const EditableCell = ({
  getValue,
  row: { original },
  column: { id },
  table,
  ktabProps: { dataType, updateData },
}: EditableCellProps) => {
  const initialValue = getValue();
  const [value, setValue] = React.useState(initialValue);
  const { isEditing } = table.options.meta;
  const row_tid = original.row_tid as number;

  React.useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  const onBlur = () => {
    updateData(row_tid, id, value);
  };

  if (!isEditing) {
    if (dataType === "date" && value instanceof Date) {
      return <span>{format(value, "PPP")}</span>;
    }
    return <span>{value}</span>;
  }

  switch (dataType) {
    case "date":
      return (
        <Popover>
          <PopoverTrigger asChild>
            <Button
              variant={"outline"}
              className={cn(
                "w-[240px] pl-3 text-left font-normal",
                !value && "text-muted-foreground",
              )}
            >
              {value ? (
                format(new Date(value), "PPP")
              ) : (
                <span>Pick a date</span>
              )}
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-auto p-0" align="start">
            <Calendar
              mode="single"
              selected={new Date(value)}
              onSelect={(date) => {
                setValue(date);
                updateData(row_tid, id, date);
              }}
              autoFocus
            />
          </PopoverContent>
        </Popover>
      );
    case "decimal":
      return (
        <Input
          type="number"
          value={value ? (value as number) : 0}
          onChange={(e) => setValue(parseFloat(e.target.value))}
          onBlur={onBlur}
        />
      );
    default: // string
      return (
        <Input
          value={value ? (value as string) : ""}
          onChange={(e) => setValue(e.target.value)}
          onBlur={onBlur}
        />
      );
  }
};

export default EditableCell;
