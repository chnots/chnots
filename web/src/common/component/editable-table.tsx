import { useState, useRef, useEffect } from "react";
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
import { Plus, Loader2 } from "lucide-react";

type Person = {
  id: string;
  name: string;
  email: string;
  role: string;
};

const CellTextInput = ({
  value,
  setNewCell,
}: {
  value: string;
  setNewCell: (input: string) => void;
}) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const handleBlur = () => {
    console.log("handle bluring ", inputRef.current?.value);
    if (inputRef.current) {
      console.log("handle input callback");
      setNewCell(inputRef.current?.value);
    }
  };

  return (
    <Input
      ref={inputRef}
      autoFocus
      defaultValue={value}
      onBlur={() => handleBlur()}
      onKeyDown={(e) => {
        if (e.key === "Enter") {
          if (inputRef.current) setNewCell(inputRef.current?.value);
        }
      }}
    />
  );
};

const fetchData = async (start: number, size: number): Promise<Person[]> => {
  await new Promise((resolve) => setTimeout(resolve, 500));
  return Array.from({ length: size }, (_, i) => ({
    id: (start + i).toString(),
    name: `User ${start + i}`,
    email: `user${start + i}@example.com`,
    role: i % 3 === 0 ? "Admin" : i % 2 === 0 ? "Editor" : "Viewer",
  }));
};

export default function EditableTable() {
  console.log("refresh EditableTable");
  const [data, setData] = useState<Person[]>([]);
  const [loading, setLoading] = useState(false);
  const [hasMore, setHasMore] = useState(true);
  const [page, setPage] = useState(0);
  const pageSize = 20;

  const [editingCell, setEditingCell] = useState<{
    rowId: string;
    columnId: string;
  } | null>(null);
  const [newRow, setNewRow] = useState<Omit<Person, "id"> & { id?: string }>({
    name: "",
    email: "",
    role: "",
  });

  const tableContainerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadMoreData();
  }, []);

  const loadMoreData = async () => {
    if (loading || !hasMore) return;

    setLoading(true);
    try {
      const newData = await fetchData(page * pageSize, pageSize);
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

  const columns: ColumnDef<Person>[] = [
    {
      accessorKey: "name",
      header: "Name",
      cell: ({ row, column, getValue }) => {
        const isEditing =
          editingCell?.rowId === row.id && editingCell?.columnId === column.id;
        const value = getValue() as string;

        return isEditing ? (
          <CellTextInput
            value={value}
            setNewCell={(input: string) => {
              const newData = [...data];
              const index = newData.findIndex(
                (item) => item.id === row.original.id
              );
              newData[index] = { ...newData[index], email: input };
              setData(newData);
              setEditingCell(null);
            }}
          />
        ) : (
          <div
            className="min-h-[40px] flex items-center"
            onDoubleClick={() =>
              setEditingCell({ rowId: row.id, columnId: column.id })
            }
          >
            {value}
          </div>
        );
      },
    },
    {
      accessorKey: "email",
      header: "Email",
      cell: ({ row, column, getValue }) => {
        const isEditing =
          editingCell?.rowId === row.id && editingCell?.columnId === column.id;
        const value = getValue() as string;

        return isEditing ? (
          <Input
            autoFocus
            value={value}
            onChange={(e) => {
              const newData = [...data];
              const index = newData.findIndex(
                (item) => item.id === row.original.id
              );
              newData[index] = { ...newData[index], email: e.target.value };
              setData(newData);
            }}
            onBlur={() => setEditingCell(null)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                setEditingCell(null);
              }
            }}
          />
        ) : (
          <div
            className="min-h-[40px] flex items-center"
            onDoubleClick={() =>
              setEditingCell({ rowId: row.id, columnId: column.id })
            }
          >
            {value}
          </div>
        );
      },
    },
    {
      accessorKey: "role",
      header: "Role",
      cell: ({ row, column, getValue }) => {
        const isEditing =
          editingCell?.rowId === row.id && editingCell?.columnId === column.id;
        const value = getValue() as string;

        return isEditing ? (
          <Input
            autoFocus
            value={value}
            onChange={(e) => {
              const newData = [...data];
              const index = newData.findIndex(
                (item) => item.id === row.original.id
              );
              newData[index] = { ...newData[index], role: e.target.value };
              setData(newData);
            }}
            onBlur={() => setEditingCell(null)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                setEditingCell(null);
              }
            }}
          />
        ) : (
          <div
            className="min-h-[40px] flex items-center"
            onDoubleClick={() =>
              setEditingCell({ rowId: row.id, columnId: column.id })
            }
          >
            {value}
          </div>
        );
      },
    },
  ];

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  const handleAddRow = () => {
    if (newRow.name && newRow.email && newRow.role) {
      setData((prev) => [
        {
          id: Math.random().toString(36).substring(2, 9),
          ...newRow,
        },
        ...prev,
      ]);
      setNewRow({ name: "", email: "", role: "" });
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex gap-4 p-4 border rounded-md">
        <Input
          placeholder="Name"
          value={newRow.name}
          onChange={(e) => setNewRow({ ...newRow, name: e.target.value })}
          className="flex-1"
        />
        <Input
          placeholder="Email"
          value={newRow.email}
          onChange={(e) => setNewRow({ ...newRow, email: e.target.value })}
          className="flex-1"
        />
        <div className="flex gap-2 flex-1">
          <Input
            placeholder="Role"
            value={newRow.role}
            onChange={(e) => setNewRow({ ...newRow, role: e.target.value })}
          />
          <Button
            onClick={handleAddRow}
            disabled={!newRow.name || !newRow.email || !newRow.role}
          >
            <Plus className="h-4 w-4 mr-2" />
            Add
          </Button>
        </div>
      </div>

      <div
        ref={tableContainerRef}
        className="rounded-md border h-[600px] overflow-auto relative"
      >
        <Table>
          <TableHeader className="sticky top-0 bg-background">
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                    </TableHead>
                  );
                })}
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

        {loading && (
          <div className="flex justify-center items-center p-4">
            <Loader2 className="h-6 w-6 animate-spin" />
          </div>
        )}

        {!hasMore && !loading && (
          <div className="flex justify-center items-center p-4 text-sm text-muted-foreground">
            No more data to load
          </div>
        )}
      </div>
    </div>
  );
}
