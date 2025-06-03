import { useState, useRef, useEffect, KeyboardEvent } from "react";
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
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
import { Input } from "@/common/component/ui/input";
import { Button } from "@/common/component/ui/button";
import Icon from "./icon";
import dayjs from "dayjs";
import customParseFormat from "dayjs/plugin/customParseFormat";

dayjs.extend(customParseFormat);

export type ColumnConfig = {
  id: string;
  name: string;
  type: "string" | "number" | "date" | "image";
  required: boolean;
};

export type TableConfig = {
  table_name: string;
  kspace: string;
};

type RowData = {
  id: string;
  [key: string]: any;
};

const CellEditor = ({
  value,
  type,
  onSave,
  onCancel,
}: {
  value: any;
  type: string;
  onSave: (value: any) => void;
  onCancel: () => void;
}) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [inputValue, setInputValue] = useState(value);

  useEffect(() => {
    if (inputRef.current) {
      inputRef.current.focus();
    }
  }, []);

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      onSave(inputValue);
    } else if (e.key === "Escape") {
      onCancel();
    }
  };

  const renderInput = () => {
    switch (type) {
      case "number":
        return (
          <Input
            ref={inputRef}
            type="number"
            value={inputValue}
            onChange={(e) => setInputValue(Number(e.target.value))}
            onBlur={() => onSave(inputValue)}
            onKeyDown={handleKeyDown}
            className="w-full"
          />
        );
      case "date":
        return (
          <div className="relative">
            <Input
              ref={inputRef}
              type="date"
              value={
                typeof inputValue === "string"
                  ? inputValue
                  : dayjs(inputValue, "yyyy-mm-dd").toString()
              }
              onChange={(e) => setInputValue(e.target.value)}
              onBlur={() => onSave(inputValue)}
              onKeyDown={handleKeyDown}
              className="w-full"
            />
          </div>
        );
      case "image":
        return (
          <Input
            ref={inputRef}
            type="url"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onBlur={() => onSave(inputValue)}
            onKeyDown={handleKeyDown}
            placeholder="Image URL"
            className="w-full"
          />
        );
      default:
        return (
          <Input
            ref={inputRef}
            type="text"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onBlur={() => onSave(inputValue)}
            onKeyDown={handleKeyDown}
            className="w-full"
          />
        );
    }
  };

  return renderInput();
};

const CellDisplay = ({ value, type }: { value: any; type: string }) => {
  const renderValue = () => {
    switch (type) {
      case "number":
        return <span>{value}</span>;
      case "date":
        return <span>{value ? value : ""}</span>;
      case "image":
        return value ? (
          <div className="h-10 w-10 relative">
            <img
              src={value}
              alt="Image"
              className="h-full w-full object-cover rounded"
              onError={(e) => {
                (e.target as HTMLImageElement).src = "https://";
              }}
            />
          </div>
        ) : (
          <span className="text-muted-foreground">No image</span>
        );
      default:
        return <span>{value}</span>;
    }
  };

  return <div className="min-h-[40px] flex items-center">{renderValue()}</div>;
};

const fetchData = async (
  start: number,
  size: number,
  columns: ColumnConfig[]
): Promise<RowData[]> => {
  await new Promise((resolve) => setTimeout(resolve, 500));

  return Array.from({ length: size }, (_, i) => {
    const item: RowData = { id: (start + i).toString() };

    columns.forEach((column) => {
      switch (column.type) {
        case "number":
          item[column.id] = Math.floor(Math.random() * 1000);
          break;
        case "date": {
          const date = new Date();
          date.setDate(date.getDate() - Math.floor(Math.random() * 365));
          item[column.id] = date.toISOString();
          break;
        }
        case "image":
          item[column.id] = `https:
            (start + i) % 100
          }/40/40`;
          break;
        default:
          item[column.id] = `${column.name} ${start + i}`;
      }
    });

    return item;
  });
};

interface EditableTableProps {
  columnConfig: ColumnConfig[];
  tableConfig: TableConfig;
  onDataChange?: (data: RowData[]) => void;
  initialData?: RowData[];
  fetchData: (
    start: number,
    size: number,
    columns: ColumnConfig[]
  ) => Promise<RowData[]>;
}

const EditableTable = ({
  columnConfig,
  onDataChange,
  initialData = [],
}: EditableTableProps) => {
  const [data, setData] = useState<RowData[]>(initialData);
  const [loading, setLoading] = useState(false);
  const [hasMore, setHasMore] = useState(true);
  const [page, setPage] = useState(0);
  const pageSize = 20;

  const [editingCell, setEditingCell] = useState<{
    rowIndex: number;
    columnIndex: number;
  } | null>(null);

  const [focusedCell, setFocusedCell] = useState<{
    rowIndex: number;
    columnIndex: number;
  } | null>(null);

  const [newRow, setNewRow] = useState<Omit<RowData, "id">>(() => {
    const initialRow: any = {};
    columnConfig.forEach((column) => {
      initialRow[column.id] = "";
    });
    return initialRow;
  });

  const tableContainerRef = useRef<HTMLDivElement>(null);
  const tableRef = useRef<HTMLTableElement>(null);

  useEffect(() => {
    if (initialData.length === 0) {
      loadMoreData();
    }
  }, []);

  useEffect(() => {
    if (onDataChange) {
      onDataChange(data);
    }
  }, [data, onDataChange]);

  const loadMoreData = async () => {
    if (loading || !hasMore) return;

    setLoading(true);
    try {
      const newData = await fetchData(page * pageSize, pageSize, columnConfig);
      setData((prev) => [...prev, ...newData]);
      setPage((prev) => prev + 1);
      setHasMore(newData.length === pageSize);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
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
  }, [loading, hasMore]);

  const columns: ColumnDef<RowData>[] = columnConfig.map((config) => ({
    accessorKey: config.id,
    header: config.name,
    cell: ({ row, column, getValue }) => {
      const isEditing =
        editingCell?.rowIndex === row.index &&
        editingCell?.columnIndex === column.getIndex();
      const value = getValue();
      console.log(
        "remap ",
        isEditing,
        editingCell?.rowIndex,
        row.id,
        editingCell?.columnIndex,
        column.id
      );

      return isEditing ? (
        <CellEditor
          value={value}
          type={config.type}
          onSave={(newValue) => {
            const newData = [...data];
            const index = newData.findIndex(
              (item) => item.id === row.original.id
            );
            newData[index] = { ...newData[index], [config.id]: newValue };
            setData(newData);
            setEditingCell(null);
          }}
          onCancel={() => setEditingCell(null)}
        />
      ) : (
        <CellDisplay value={value} type={config.type} />
      );
    },
  }));

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  const handleAddRow = () => {
    const isValid = columnConfig.every(
      (column) =>
        !column.required ||
        (newRow[column.id] !== undefined && newRow[column.id] !== "")
    );

    if (isValid) {
      const newItem = {
        id: Math.random().toString(36).substring(2, 9),
        ...newRow,
      };

      setData((prev) => [newItem, ...prev]);

      const resetRow: any = {};
      columnConfig.forEach((column) => {
        resetRow[column.id] = "";
      });
      setNewRow(resetRow);
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
    if (editingCell) return;
    if (!focusedCell) return;

    const { rowIndex, columnIndex } = focusedCell;
    const rows = table.getRowModel().rows;
    const columns = table.getAllColumns();

    switch (e.key) {
      case "ArrowUp":
        e.preventDefault();
        if (rowIndex > 0) {
          setFocusedCell({ rowIndex: rowIndex - 1, columnIndex });
        }
        break;
      case "ArrowDown":
        e.preventDefault();
        if (rowIndex < rows.length - 1) {
          setFocusedCell({ rowIndex: rowIndex + 1, columnIndex });
        }
        break;
      case "ArrowLeft":
        e.preventDefault();
        if (columnIndex > 0) {
          setFocusedCell({ rowIndex, columnIndex: columnIndex - 1 });
        }
        break;
      case "ArrowRight":
        e.preventDefault();
        if (columnIndex < columns.length - 1) {
          setFocusedCell({ rowIndex, columnIndex: columnIndex + 1 });
        }
        break;
      case "Enter": {
        e.preventDefault();
        const row = rows[rowIndex];
        const column = columns[columnIndex];
        if (row && column) {
          setEditingCell({
            rowIndex: row.index,
            columnIndex: column.getIndex(),
          });
        }
        break;
      }
    }
  };

  useEffect(() => {
    if (!focusedCell || !tableRef.current || !tableContainerRef.current) return;

    const table = tableRef.current;
    const container = tableContainerRef.current;

    const cell = table.querySelector(
      `tr:nth-child(${focusedCell.rowIndex + 1}) td:nth-child(${
        focusedCell.columnIndex + 1
      })`
    );

    if (cell) {
      const cellRect = cell.getBoundingClientRect();
      const containerRect = container.getBoundingClientRect();

      if (cellRect.top < containerRect.top) {
        container.scrollTop -= containerRect.top - cellRect.top;
      } else if (cellRect.bottom > containerRect.bottom) {
        container.scrollTop += cellRect.bottom - containerRect.bottom;
      }
    }
  }, [focusedCell]);

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap gap-4 p-4 border rounded-md">
        {columnConfig.map((column) => (
          <div key={column.id} className="flex-1 min-w-[200px]">
            <label className="text-sm font-medium mb-1 block">
              {column.name}
              {column.required && <span className="text-red-500 ml-1">*</span>}
            </label>
            {column.type === "number" ? (
              <Input
                type="number"
                placeholder={column.name}
                value={newRow[column.id] || ""}
                onChange={(e) =>
                  setNewRow({ ...newRow, [column.id]: Number(e.target.value) })
                }
              />
            ) : column.type === "date" ? (
              <div className="relative">
                <Input
                  type="date"
                  placeholder={column.name}
                  value={newRow[column.id] || ""}
                  onChange={(e) =>
                    setNewRow({ ...newRow, [column.id]: e.target.value })
                  }
                />
              </div>
            ) : column.type === "image" ? (
              <Input
                type="url"
                placeholder="Image URL"
                value={newRow[column.id] || ""}
                onChange={(e) =>
                  setNewRow({ ...newRow, [column.id]: e.target.value })
                }
              />
            ) : (
              <Input
                placeholder={column.name}
                value={newRow[column.id] || ""}
                onChange={(e) =>
                  setNewRow({ ...newRow, [column.id]: e.target.value })
                }
              />
            )}
          </div>
        ))}
        <div className="flex items-end">
          <Button onClick={handleAddRow}>
            <Icon.Plus className="h-4 w-4 mr-2" />
            Add
          </Button>
        </div>
      </div>

      <div
        ref={tableContainerRef}
        className="rounded-md border h-[600px] overflow-auto relative"
        tabIndex={0}
        onKeyDown={handleKeyDown}
      >
        <Table ref={tableRef}>
          <TableHeader className="sticky top-0 bg-background">
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
              table.getRowModel().rows.map((row, rowIndex) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                  className={
                    focusedCell?.rowIndex === rowIndex ? "bg-muted/50" : ""
                  }
                >
                  {row.getVisibleCells().map((cell, columnIndex) => (
                    <TableCell
                      key={cell.id}
                      className={
                        focusedCell?.rowIndex === rowIndex &&
                        focusedCell?.columnIndex === columnIndex
                          ? "bg-accent"
                          : ""
                      }
                      onClick={() => {
                        if (
                          focusedCell &&
                          focusedCell.columnIndex === columnIndex &&
                          focusedCell.rowIndex === rowIndex
                        ) {
                          if (
                            !editingCell ||
                            (editingCell &&
                              editingCell.rowIndex !== rowIndex &&
                              editingCell.columnIndex !== columnIndex)
                          ) {
                            setEditingCell({
                              rowIndex: rowIndex,
                              columnIndex: columnIndex,
                            });
                          }
                        } else {
                          setFocusedCell({ rowIndex, columnIndex });
                        }
                      }}
                    >
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
        {loading && (
          <div className="absolute bottom-0 left-0 right-0 bg-background/80 p-2 flex justify-center">
            <Icon.Loader2 className="h-6 w-6 animate-spin text-primary" />
          </div>
        )}
      </div>
    </div>
  );
};

export default EditableTable;
