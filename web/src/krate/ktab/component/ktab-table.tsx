import { useEffect, useState, useCallback, useMemo } from "react";
import {
  PlusIcon,
  ArrowLeftRightIcon,
  DownloadIcon,
  Trash2Icon,
  PencilIcon,
} from "lucide-react";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/common/component/ui/dialog";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
} from "@/common/component/ui/context-menu";
import { genTID } from "@/lib/id_util";
import type { KTabMeta, KTabColumnMeta, KTabColumnViewKind } from "../po";
import { ktabViewToStoreKind, KTAB_DISPLAY_AS_MENU } from "../po";
import { ktabCellCommit } from "../service";
import { ktabToStoreValue } from "../dto";
import type { KTabViewCell } from "../dto";
import { getCellRenderer } from "./cell-renderer";
import { exportCsv } from "./csv-export";

export type KTabRowData = {
  row_tid: number;
  [key: string]: unknown;
};

export interface KTabTableProps {
  tableMeta: KTabMeta;
  fetchData: (
    tableId: number,
    start: number,
    size: number,
  ) => Promise<KTabRowData[]>;
  readonly: boolean;
  onMetaChange: (meta: KTabMeta) => void | Promise<void>;
  compact?: boolean;
}

const PAGE_SIZE = 500;

const VIEW_KIND_LABELS: Record<string, string> = {
  text: "Text",
  string: "Text",
  number: "数字",
  integer: "数字",
  decimal: "数字",
  progress: "进度",
  checkbox: "Checkbox",
  multi_select: "枚举",
  date: "日期",
  datetime: "日期时间",
  image: "图片",
};

export function KTabTable({
  tableMeta,
  fetchData,
  readonly,
  onMetaChange,
  compact = false,
}: KTabTableProps) {
  const [rows, setRows] = useState<KTabRowData[]>([]);
  const [transposed, setTransposed] = useState(false);
  const [renaming, setRenaming] = useState<string | null>(null);
  const [renameDraft, setRenameDraft] = useState("");
  const [addColumnOpen, setAddColumnOpen] = useState(false);
  const [newColName, setNewColName] = useState("");
  const [newColKind, setNewColKind] = useState<KTabColumnViewKind>("text");

  const columns = useMemo(() => {
    return Object.values(tableMeta.columns).sort(
      (a, b) => a.order_by - b.order_by,
    );
  }, [tableMeta.columns]);

  const loadRows = useCallback(async () => {
    const data = await fetchData(tableMeta.otid, 0, PAGE_SIZE);
    setRows(data);
  }, [fetchData, tableMeta.otid]);

  useEffect(() => {
    loadRows();
  }, [loadRows]);

  const commitCell = useCallback(
    async (row: KTabRowData, col: KTabColumnMeta, value: unknown) => {
      const storeValue = ktabToStoreValue(col.view_kind, value);
      const cell: KTabViewCell = {
        row_tid: row.row_tid,
        column_name: col.name,
        value: storeValue,
      };
      await ktabCellCommit({ table_id: tableMeta.otid, cells: [cell] });
      const stored = value instanceof Date ? value.toISOString() : value;
      setRows((prev) =>
        prev.map((r) =>
          r.row_tid === row.row_tid ? { ...r, [col.name]: stored } : r,
        ),
      );
    },
    [tableMeta.otid],
  );

  // --- Column operations ---

  const updateColumn = useCallback(
    async (colIdx: string, patch: Partial<KTabColumnMeta>) => {
      const col = tableMeta.columns[colIdx];
      if (!col) return;
      const newMeta = {
        ...tableMeta,
        columns: {
          ...tableMeta.columns,
          [colIdx]: { ...col, ...patch },
        },
      };
      await onMetaChange(newMeta);
    },
    [tableMeta, onMetaChange],
  );

  const handleRename = useCallback(async () => {
    if (!renaming || !renameDraft.trim()) return;
    const colKey = Object.keys(tableMeta.columns).find(
      (k) => tableMeta.columns[k].name === renaming,
    );
    if (!colKey) return;
    const newName = renameDraft.trim();
    const { [colKey]: col, ...restCols } = tableMeta.columns;
    const newMeta = {
      ...tableMeta,
      columns: { ...restCols, [newName]: { ...col, name: newName } },
    };
    await onMetaChange(newMeta);
    setRows((prev) =>
      prev.map((r) => {
        if (!(renaming in r)) return r;
        const updated = { ...r, [newName]: r[renaming] };
        delete updated[renaming];
        return updated;
      }),
    );
    setRenaming(null);
    setRenameDraft("");
  }, [renaming, renameDraft, tableMeta, onMetaChange]);

  const handleDeleteColumn = useCallback(
    async (colName: string) => {
      if (!tableMeta.columns[colName]) return;
      const { [colName]: _, ...rest } = tableMeta.columns;
      const newMeta = { ...tableMeta, columns: rest };
      await onMetaChange(newMeta);
      setRows((prev) =>
        prev.map((r) => {
          const { [colName]: __, ...rest } = r;
          return rest as KTabRowData;
        }),
      );
    },
    [tableMeta, onMetaChange],
  );

  const handleDisplayAs = useCallback(
    async (colName: string, viewKind: KTabColumnViewKind) => {
      if (!tableMeta.columns[colName]) return;
      await updateColumn(colName, {
        view_kind: viewKind,
        store_kind: ktabViewToStoreKind(viewKind),
      });
    },
    [tableMeta.columns, updateColumn],
  );

  const handleAddColumn = useCallback(async () => {
    if (!newColName.trim()) return;
    const idx = genTID();
    const maxOrder = columns.reduce((m, c) => Math.max(m, c.order_by), 0);
    const newCol: KTabColumnMeta = {
      idx,
      name: newColName.trim(),
      comment: "",
      store_kind: ktabViewToStoreKind(newColKind),
      view_kind: newColKind,
      required: false,
      order_by: maxOrder + 1,
    };
    const newMeta = {
      ...tableMeta,
      columns: { ...tableMeta.columns, [newColName.trim()]: newCol },
    };
    await onMetaChange(newMeta);
    setAddColumnOpen(false);
    setNewColName("");
    setNewColKind("text");
  }, [newColName, newColKind, columns, tableMeta, onMetaChange]);

  const handleAddRow = useCallback(() => {
    const newRowTid = genTID();
    setRows((prev) => [...prev, { row_tid: newRowTid }]);
  }, []);

  const handleExport = useCallback(async () => {
    await exportCsv(tableMeta, fetchData);
  }, [tableMeta, fetchData]);

  // --- Transpose ---

  const transposedData = useMemo(() => {
    if (!transposed) return null;
    const tRows: KTabRowData[] = columns.map((col, i) => {
      const row: KTabRowData = { row_tid: i, _col_name: col.name };
      rows.forEach((r) => {
        row[`row_${r.row_tid}`] = r[col.name] ?? "";
      });
      return row;
    });
    return {
      rows: tRows,
      headers: ["Field", ...rows.map((r) => `#${r.row_tid}`)],
    };
  }, [transposed, columns, rows]);

  // --- Context menu for column header ---
  const columnContextMenu = (col: KTabColumnMeta) => (
    <ContextMenuContent>
      <ContextMenuItem
        onClick={() => {
          setRenameDraft(col.name);
          setRenaming(col.name);
        }}
      >
        <PencilIcon className="h-3.5 w-3.5 mr-2" />
        重命名
      </ContextMenuItem>
      <ContextMenuItem
        variant="destructive"
        onClick={() => handleDeleteColumn(col.name)}
      >
        <Trash2Icon className="h-3.5 w-3.5 mr-2" />
        删除
      </ContextMenuItem>
      <ContextMenuSeparator />
      {/* 显示为 submenu */}
      {KTAB_DISPLAY_AS_MENU.map(
        (group: {
          group: string;
          items: { viewKind: KTabColumnViewKind; label: string }[];
        }) => (
          <ContextMenuSub key={group.group}>
            <ContextMenuSubTrigger>{group.group}</ContextMenuSubTrigger>
            <ContextMenuSubContent>
              {group.items.map(
                (item: { viewKind: KTabColumnViewKind; label: string }) => (
                  <ContextMenuItem
                    key={item.viewKind + item.label}
                    onClick={() => handleDisplayAs(col.name, item.viewKind)}
                  >
                    {col.view_kind === item.viewKind && "✓ "}
                    {item.label}
                  </ContextMenuItem>
                ),
              )}
            </ContextMenuSubContent>
          </ContextMenuSub>
        ),
      )}
    </ContextMenuContent>
  );

  // --- Toolbar ---
  const toolbar = (untranspose = false) => (
    <div className="flex items-center gap-2 px-2 py-1 border-b">
      <Button
        variant="ghost"
        size="sm"
        onClick={() => setTransposed(!untranspose)}
      >
        <ArrowLeftRightIcon className="h-3.5 w-3.5 mr-1" />
        {untranspose ? "Untranspose" : "Transpose"}
      </Button>
      <Button variant="ghost" size="sm" onClick={handleExport}>
        <DownloadIcon className="h-3.5 w-3.5 mr-1" />
        CSV
      </Button>
    </div>
  );

  const compactExport = (
    <div className="flex justify-end px-2 py-1">
      <Button
        variant="ghost"
        size="icon"
        className="h-6 w-6"
        onClick={handleExport}
      >
        <DownloadIcon className="h-3.5 w-3.5" />
      </Button>
    </div>
  );

  // --- Transposed view ---
  if (transposed && transposedData) {
    return (
      <div className="flex flex-col h-full">
        {!compact && toolbar(true)}
        {compact && !readonly && compactExport}
        <div className="overflow-auto flex-1">
          <table className="w-full border-collapse table-fixed text-sm">
            <colgroup>
              <col className="w-24" />
            </colgroup>
            <thead>
              <tr className="bg-muted/50">
                <th className="border px-2 py-1 text-left font-medium text-muted-foreground">
                  #
                </th>
                {transposedData.headers.slice(1).map((h) => (
                  <th
                    key={h}
                    className="border px-2 py-1 text-left font-medium overflow-hidden text-ellipsis whitespace-nowrap"
                  >
                    {h}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {transposedData.rows.map((row) => (
                <tr key={String(row._col_name)} className="hover:bg-accent/30">
                  <td className="border px-2 py-1 text-muted-foreground bg-muted/30 font-medium">
                    {String(row._col_name)}
                  </td>
                  {rows.map((originalRow) => (
                    <td
                      key={originalRow.row_tid}
                      className="border px-2 py-1 overflow-hidden break-words"
                    >
                      {String(row[`row_${originalRow.row_tid}`] ?? "—")}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  // --- Normal view ---
  return (
    <div className="flex flex-col h-full">
      {!compact && toolbar()}
      {compact && !readonly && compactExport}

      <div className="overflow-auto flex-1">
        <table className="w-full border-collapse table-fixed text-sm">
          <colgroup>
            <col className="w-12" />
            {columns.map((col) => (
              <col key={col.idx} />
            ))}
          </colgroup>
          <thead>
            <tr className="bg-muted/50">
              <th className="border px-2 py-1 text-center font-medium text-muted-foreground">
                #
              </th>
              {columns.map((col) => (
                <th
                  key={col.idx}
                  className="border px-2 py-1 text-left font-medium select-none overflow-hidden text-ellipsis whitespace-nowrap"
                >
                  <ContextMenu>
                    <ContextMenuTrigger asChild>
                      <div className="flex items-center gap-1 cursor-default">
                        <span className="text-xs text-muted-foreground">
                          {VIEW_KIND_LABELS[col.view_kind]}
                        </span>
                        <span>{col.name}</span>
                      </div>
                    </ContextMenuTrigger>
                    {columnContextMenu(col)}
                  </ContextMenu>
                </th>
              ))}
              {!readonly && (
                <th className="border w-10 min-w-10">
                  <Popover open={addColumnOpen} onOpenChange={setAddColumnOpen}>
                    <PopoverTrigger asChild>
                      <button className="w-full h-full flex items-center justify-center hover:bg-accent/50">
                        <PlusIcon className="h-4 w-4 text-muted-foreground" />
                      </button>
                    </PopoverTrigger>
                    <PopoverContent align="start" className="w-64 p-3">
                      <div className="space-y-2">
                        <Input
                          placeholder="列名"
                          value={newColName}
                          onChange={(e) => setNewColName(e.target.value)}
                          onKeyDown={(e) => {
                            if (e.key === "Enter") handleAddColumn();
                          }}
                        />
                        <select
                          className="w-full border rounded px-2 py-1 text-sm bg-background"
                          value={newColKind}
                          onChange={(e) =>
                            setNewColKind(e.target.value as KTabColumnViewKind)
                          }
                        >
                          {Object.entries(VIEW_KIND_LABELS).map(
                            ([kind, label]) => (
                              <option key={kind} value={kind}>
                                {label}
                              </option>
                            ),
                          )}
                        </select>
                        <Button
                          size="sm"
                          className="w-full"
                          onClick={handleAddColumn}
                        >
                          添加列
                        </Button>
                      </div>
                    </PopoverContent>
                  </Popover>
                </th>
              )}
            </tr>
          </thead>
          <tbody>
            {rows.map((row, rowIdx) => (
              <tr key={row.row_tid} className="hover:bg-accent/30">
                <td className="border px-2 py-0 text-center text-muted-foreground bg-muted/20 text-xs tabular-nums">
                  {rowIdx + 1}
                </td>
                {columns.map((col) => {
                  const Renderer = getCellRenderer(col.view_kind);
                  return (
                    <td
                      key={col.idx}
                      className="border px-0 py-0 overflow-hidden break-words"
                    >
                      <Renderer
                        value={row[col.name]}
                        columnMeta={col}
                        readonly={readonly}
                        onCommit={(val) => commitCell(row, col, val)}
                      />
                    </td>
                  );
                })}
                {!readonly && <td className="border" />}
              </tr>
            ))}
            {/* Add row button */}
            {!readonly && (
              <tr>
                <td
                  colSpan={columns.length + 2}
                  className="border border-dashed"
                >
                  <button
                    className="w-full py-1 flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-accent/30 transition-colors"
                    onClick={handleAddRow}
                  >
                    <PlusIcon className="h-3.5 w-3.5 mr-1" />
                    添加行
                  </button>
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      {/* Rename dialog */}
      <Dialog
        open={renaming !== null}
        onOpenChange={(open) => {
          if (!open) setRenaming(null);
        }}
      >
        <DialogContent className="sm:max-w-sm">
          <DialogHeader>
            <DialogTitle>重命名</DialogTitle>
          </DialogHeader>
          <Input
            value={renameDraft}
            onChange={(e) => setRenameDraft(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") handleRename();
            }}
            autoFocus
          />
          <DialogFooter>
            <Button variant="outline" onClick={() => setRenaming(null)}>
              取消
            </Button>
            <Button onClick={handleRename}>保存</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
