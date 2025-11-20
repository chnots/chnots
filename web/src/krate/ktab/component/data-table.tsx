'use client';

import * as React from 'react';
import { useCallback } from 'react';
import {
  type AccessorKeyColumnDef,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  type SortingState,
  useReactTable,
} from '@tanstack/react-table';

import { ktabToStoreValue } from '../dto';
import { ktabCellCommit } from '../service';
import EditableCell, { type KTabRowData } from './editable-cell';

import type { KTabMeta } from '../po';
import { Button } from '@/common/component/ui/button';
import { Input } from '@/common/component/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/common/component/ui/select';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/common/component/ui/table';
import { genTID } from '@/lib/id_util';

export function DataTable({
  tableMeta,
  fetchData,
  onMetaChange,
  readonly,
}: {
  tableMeta: KTabMeta;
  onMetaChange: (tableMeta: KTabMeta) => Promise<void>;
  fetchData: (
    table_id: number,
    start: number,
    size: number,
    column_name?: string,
  ) => Promise<KTabRowData[]>;
  readonly: boolean;
}) {
  const [loading, setLoading] = React.useState(false);
  const [hasMore, setHasMore] = React.useState(true);
  const [page, setPage] = React.useState(0);
  const pageSize = 20;

  const [data, setData] = React.useState<KTabRowData[]>([]);

  const [columns, setColumns] = React.useState<AccessorKeyColumnDef<KTabRowData>[]>([]);

  const updateData = React.useCallback(
    async (rowIndex: number, columnId: string, value: any) => {
      if (tableMeta) {
        await ktabCellCommit({
          table_id: tableMeta.otid,
          cells: [
            {
              row_tid: rowIndex,
              column_name: columnId,
              value: ktabToStoreValue(tableMeta.columns[columnId].view_kind, value),
            },
          ],
        });
        setData((old) =>
          old.map((row, index) => {
            if (index === rowIndex && old[rowIndex]) {
              return {
                ...old[rowIndex],
                [columnId]: value,
              };
            }
            return row;
          }),
        );
      }
    },
    [tableMeta],
  );

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
        }, {}),
      );
    }
  }, [tableMeta, updateData]);

  const loadMoreData = useCallback(async () => {
    if (loading || !hasMore) return;

    setLoading(true);
    try {
      if (tableMeta) {
        const newData = await fetchData(tableMeta.otid, page * pageSize, pageSize);
        setData((prev) => [...prev, ...newData]);
        setPage((prev) => prev + 1);
        setHasMore(newData.length === pageSize);
      }
    } finally {
      setLoading(false);
    }
  }, [fetchData, tableMeta, hasMore, loading, page]);

  React.useEffect(() => {
    loadMoreData();
  }, [loadMoreData]);

  const [sorting, setSorting] = React.useState<SortingState>([]);

  const [newColumnName, setNewColumnName] = React.useState('');
  const [newColumnType, setNewColumnType] = React.useState<'string' | 'date' | 'decimal'>('string');

  const handleAddNewColumn = async () => {
    if (!newColumnName) {
      alert('Column name cannot be empty.');
      return;
    }
    const newColumnId = newColumnName.toLowerCase().replace(/\s+/g, '_');
    setNewColumnName('');

    await onMetaChange({
      ...tableMeta,
      columns: {
        ...tableMeta.columns,
        [newColumnId]: {
          idx: genTID(),
          name: newColumnId,
          comment: '',
          store_kind: 'str',
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
      { row_tid: genTID() },
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
      isEditing: !readonly,
      updateData,
    },
  });

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key !== 'Tab' || readonly) return;

    const activeElement = document.activeElement;
    const allInputs = Array.from(
      e.currentTarget.querySelectorAll('input, button[role="combobox"]'),
    );
    if (!activeElement) {
      return;
    }
    const currentIndex = allInputs.indexOf(activeElement);

    if (currentIndex === allInputs.length - 1 && !e.shiftKey) {
      e.preventDefault();
      addNewRow();

      // TODO: unable to auto focus now line.
      setTimeout(() => {
        const newInputs = Array.from(
          e.currentTarget.querySelectorAll('input, button[role="combobox"]'),
        );
        (newInputs[currentIndex + 1] as HTMLElement)?.focus();
      }, 100);
    }
  };

  return (
    <div role="none" onKeyDown={handleKeyDown} className="flex flex-col w-full h-full">
      {!readonly && (
        <div className="flex items-center justify-between py-4">
          <div className="flex items-center space-x-2 flex-wrap">
            <Input
              placeholder="New Column Name"
              value={newColumnName}
              onChange={(e) => setNewColumnName(e.target.value)}
            />
            <Select value={newColumnType} onValueChange={(value: any) => setNewColumnType(value)}>
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
          <Button variant="outline" size="sm" onClick={addNewRow}>
            Add Row
          </Button>
        </div>
      )}
      <div className={`flex-grow rounded-md border`}>
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(header.column.columnDef.header, header.getContext())}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow key={row.id} data-state={row.getIsSelected() && 'selected'}>
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell colSpan={columns.length} className="h-24 text-center">
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
