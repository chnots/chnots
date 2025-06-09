"use client";

import * as React from "react";
import {
  AccessorKeyColumnDef,
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
} from "@tanstack/react-table";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/common/component/ui/table";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/common/component/ui/select";
import EditableCell, { KTabRowData } from "./editable-cell";
import { genTSID } from "@/lib/id_util";
import { KTabMeta } from "../store/po";
import { ktabToStoreValue } from "../store/dto";
import { ktabCellsOverwrite } from "../store/service";

export function DataTable({
  tableMeta,
  fetchData,
  onMetaChange,
  isEditing,
}: {
  tableMeta: KTabMeta;
  onMetaChange: (tableMeta: KTabMeta) => Promise<void>;
  fetchData: (
    table_id: number,
    start: number,
    size: number,
    column_name?: string
  ) => Promise<KTabRowData[]>;
  isEditing: boolean;
}) {
  const [loading, setLoading] = React.useState(false);
  const [hasMore, setHasMore] = React.useState(true);
  const [page, setPage] = React.useState(0);
  const pageSize = 20;

  const [data, setData] = React.useState<KTabRowData[]>([]);

  const [columns, setColumns] = React.useState<
    AccessorKeyColumnDef<KTabRowData>[]
  >([]);

  React.useEffect(() => {
    if (tableMeta) {
      setColumns(
        Object.values(tableMeta.columns).map((column) => {
          return {
            accessorKey: column.name,
            header: column.name,
            cell: (props) => (
              <EditableCell
                ktabProps={{
                  dataType: column.view_kind,
                  updateData,
                }}
                {...props}
              />
            ),
            meta: {
              isEditable: true,
              dataType: column.view_kind,
            },
          };
        }, {})
      );
    }
  }, [tableMeta]);

  const updateData = React.useCallback(
    async (rowIndex: number, columnId: string, value: any) => {
      console.log("row index: ", rowIndex);
      if (tableMeta) {
        await ktabCellsOverwrite({
          table_id: tableMeta.id,
          cells: [
            {
              row_idx: rowIndex,
              column_name: columnId,
              value: ktabToStoreValue(
                tableMeta.columns[columnId].view_kind,
                value
              ),
            },
          ],
        });
        setData((old) =>
          old.map((row, index) => {
            if (index === rowIndex) {
              return {
                ...old[rowIndex]!,
                [columnId]: value,
              };
            }
            return row;
          })
        );
      }
    },
    [tableMeta]
  );

  const loadMoreData = async () => {
    if (loading || !hasMore) return;

    setLoading(true);
    try {
      if (tableMeta) {
        const newData = await fetchData(
          tableMeta.id,
          page * pageSize,
          pageSize
        );
        setData((prev) => [...prev, ...newData]);
        setPage((prev) => prev + 1);
        setHasMore(newData.length === pageSize);
      }
    } finally {
      setLoading(false);
    }
  };

  React.useEffect(() => {
    loadMoreData();
  }, []);

  /*     useEffect(() => {
      const container = tableContainerRef.current;
      if (!container) return;

      const handleScroll = () => {
        const { scrollTop, scrollHeight, clientHeight } = container;
        if (
          scrollHeight - (scrollTop + clientHeight) < 100 &&
          !loading &&
          hasMore
        ) {
          loadMoreData();
        }
      };

      container.addEventListener("scroll", handleScroll);
      return () => container.removeEventListener("scroll", handleScroll);
    }, [loading, hasMore]); */
  const [sorting, setSorting] = React.useState<SortingState>([]);

  const [newColumnName, setNewColumnName] = React.useState("");
  const [newColumnType, setNewColumnType] = React.useState<
    "string" | "date" | "decimal"
  >("string");

  const handleAddNewColumn = async () => {
    if (!newColumnName) {
      alert("Column name cannot be empty.");
      return;
    }
    const newColumnId = newColumnName.toLowerCase().replace(/\s+/g, "_");

    await onMetaChange({
      ...tableMeta,
      columns: {
        ...tableMeta.columns,
        [newColumnId]: {
          idx: genTSID(),
          name: newColumnId,
          comment: "",
          store_kind: "str",
          view_kind: newColumnType,
          required: false,
          order_by: Object.values(tableMeta.columns).length + 1,
        },
      },
    });
    setData((prev) => prev.map((row) => ({ ...row, [newColumnId]: null })));
  };

  const addNewRow = () => {
    const newRow: KTabRowData = columns.reduce<KTabRowData>(
      (acc, col) => {
        acc[col.accessorKey as string] = null;
        return acc;
      },
      { row_idx: genTSID() }
    );
    setData((prev) => [...prev, newRow]);
  };

  const table = useReactTable({
    data,
    columns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    meta: {
      isEditing,
      updateData,
    },
  });

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key !== "Tab" || !isEditing) return;

    const activeElement = document.activeElement;
    const allInputs = Array.from(
      e.currentTarget.querySelectorAll('input, button[role="combobox"]')
    );
    const currentIndex = allInputs.findIndex((el) => el === activeElement);

    if (currentIndex === allInputs.length - 1 && !e.shiftKey) {
      e.preventDefault();
      addNewRow();

      // TODO: unable to auto focus now line.
      setTimeout(() => {
        const newInputs = Array.from(
          e.currentTarget.querySelectorAll('input, button[role="combobox"]')
        );
        (newInputs[currentIndex + 1] as HTMLElement)?.focus();
      }, 100);
    }
  };

  return (
    <div onKeyDown={handleKeyDown}>
      <div className="flex items-center justify-between py-4">
        {isEditing && (
          <div className="flex items-center space-x-2">
            <Input
              placeholder="New Column Name"
              value={newColumnName}
              onChange={(e) => setNewColumnName(e.target.value)}
            />
            <Select
              value={newColumnType}
              onValueChange={(value: any) => setNewColumnType(value)}
            >
              <SelectTrigger className="w-[180px]">
                <SelectValue placeholder="Select Type" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="string">String</SelectItem>
                <SelectItem value="date">Date</SelectItem>
                <SelectItem value="decimal">Decimal</SelectItem>
              </SelectContent>
            </Select>
            <Button onClick={handleAddNewColumn}>Add Column</Button>
          </div>
        )}
      </div>
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="h-24 text-center"
                >
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
      {isEditing && (
        <div className="flex items-center justify-end space-x-2 py-4">
          <Button variant="outline" size="sm" onClick={addNewRow}>
            Add Row
          </Button>
        </div>
      )}
    </div>
  );
}
