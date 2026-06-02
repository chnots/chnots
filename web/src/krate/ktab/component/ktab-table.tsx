import { useEffect, useState, useCallback, useMemo, useRef, type FC } from "react";
import {
  PlusIcon,
  ArrowLeftRightIcon,
  DownloadIcon,
  Trash2Icon,
  PencilIcon,
  TypeIcon,
  HashIcon,
  CalendarIcon,
  CalendarClockIcon,
  BarChart3Icon,
  CheckSquareIcon,
  TagsIcon,
  ImageIcon,
  ArrowLeftIcon,
  ArrowRightIcon,
  CopyIcon,
  WrapTextIcon,
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
  ContextMenuTrigger,
} from "@/common/component/ui/context-menu";
import { genTID } from "@/lib/id_util";
import type { KTabMeta, KTabColumnMeta, KTabColumnViewKind } from "../po";
import { ktabViewToStoreKind } from "../po";
import { ktabCellCommit, ktabRowDelete } from "../service";
import { ktabToStoreValue } from "../dto";
import type { KTabViewCell } from "../dto";
import { getCellRenderer } from "./cell-renderer";
import type { NavigateDirection } from "./cell-renderer/types";
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
  hideExport?: boolean;
}

const PAGE_SIZE = 500;

const VIEW_KIND_ICONS: Record<string, FC<{ className?: string }>> = {
  text: TypeIcon,
  string: TypeIcon,
  number: HashIcon,
  integer: HashIcon,
  decimal: HashIcon,
  date: CalendarIcon,
  datetime: CalendarClockIcon,
  progress: BarChart3Icon,
  checkbox: CheckSquareIcon,
  multi_select: TagsIcon,
  image: ImageIcon,
};

const ADD_COL_TYPE_OPTIONS: { value: KTabColumnViewKind; label: string }[] = [
  { value: "text", label: "文本" },
  { value: "number", label: "数字" },
  { value: "date", label: "日期" },
  { value: "datetime", label: "日期时间" },
  { value: "checkbox", label: "复选框" },
  { value: "multi_select", label: "枚举" },
  { value: "progress", label: "进度" },
];

function makeNewRow(columns: KTabColumnMeta[]): KTabRowData {
  const row: KTabRowData = { row_tid: genTID() };
  const now = new Date();
  for (const col of columns) {
    if (col.view_kind === "date") {
      row[col.name] = now.toISOString().split("T")[0];
    } else if (col.view_kind === "datetime") {
      row[col.name] = now.toISOString();
    }
  }
  return row;
}

export function KTabTable({
  tableMeta,
  fetchData,
  readonly,
  onMetaChange,
  compact = false,
  hideExport = false,
}: KTabTableProps) {
  const [rows, setRows] = useState<KTabRowData[]>([]);
  const [transposed, setTransposed] = useState(false);
  const [renaming, setRenaming] = useState<string | null>(null);
  const [renameDraft, setRenameDraft] = useState("");
  const [activeCell, setActiveCell] = useState<{
    rowIdx: number;
    colName: string;
  } | null>(null);
  const [wrapEnabled, setWrapEnabled] = useState(
    () => !!tableMeta.wrap_enabled,
  );
  const pendingRowTidsRef = useRef<Set<number>>(new Set());
  const [addColumnOpen, setAddColumnOpen] = useState(false);
  const [newColName, setNewColName] = useState("");
  const [newColKind, setNewColKind] = useState<KTabColumnViewKind>("text");

  // Insert column dialog state
  const [insertColTarget, setInsertColTarget] = useState<{
    col: KTabColumnMeta;
    direction: "left" | "right";
  } | null>(null);
  const [insertColName, setInsertColName] = useState("");
  const [insertColKind, setInsertColKind] =
    useState<KTabColumnViewKind>("text");

  // Delete row confirmation state
  const [deleteRowTarget, setDeleteRowTarget] = useState<KTabRowData | null>(
    null,
  );

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
      const cells: KTabViewCell[] = [];

      if (pendingRowTidsRef.current.has(row.row_tid)) {
        // Batch commit all default values + the edited cell
        for (const c of columns) {
          let cellValue: unknown;
          if (c.name === col.name) {
            cellValue = value;
          } else if (c.view_kind === "date" && row[c.name] != null) {
            cellValue = row[c.name];
          } else if (c.view_kind === "datetime" && row[c.name] != null) {
            cellValue = row[c.name];
          } else {
            continue; // skip empty/non-default columns
          }
          const storeValue = ktabToStoreValue(c.view_kind, cellValue);
          if (storeValue) {
            cells.push({
              row_tid: row.row_tid,
              column_name: c.name,
              value: storeValue,
            });
          }
        }
        pendingRowTidsRef.current.delete(row.row_tid);
      } else {
        const storeValue = ktabToStoreValue(col.view_kind, value);
        if (storeValue) {
          cells.push({
            row_tid: row.row_tid,
            column_name: col.name,
            value: storeValue,
          });
        }
      }

      if (cells.length > 0) {
        await ktabCellCommit({ table_id: tableMeta.otid, cells });
      }

      const stored = value instanceof Date ? value.toISOString() : value;
      setRows((prev) =>
        prev.map((r) =>
          r.row_tid === row.row_tid ? { ...r, [col.name]: stored } : r,
        ),
      );
    },
    [tableMeta.otid, columns],
  );

  const updateColumn = useCallback(
    async (colName: string, patch: Partial<KTabColumnMeta>) => {
      const col = tableMeta.columns[colName];
      if (!col) return;
      const newMeta = {
        ...tableMeta,
        columns: {
          ...tableMeta.columns,
          [colName]: { ...col, ...patch },
        },
      };
      await onMetaChange(newMeta);
    },
    [tableMeta, onMetaChange],
  );

  // --- Column operations ---

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

  const handleInsertColumn = useCallback(async () => {
    if (!insertColTarget || !insertColName.trim()) return;
    const { col, direction } = insertColTarget;
    const sortedCols = [...columns];
    const targetIndex = sortedCols.findIndex((c) => c.idx === col.idx);

    let newOrder: number;
    if (direction === "left") {
      const leftOrder =
        targetIndex > 0 ? sortedCols[targetIndex - 1].order_by : undefined;
      newOrder =
        leftOrder !== undefined
          ? (leftOrder + col.order_by) / 2
          : col.order_by - 1;
    } else {
      const rightOrder =
        targetIndex < sortedCols.length - 1
          ? sortedCols[targetIndex + 1].order_by
          : undefined;
      newOrder =
        rightOrder !== undefined
          ? (col.order_by + rightOrder) / 2
          : col.order_by + 1;
    }

    const idx = genTID();
    const name = insertColName.trim();
    const newCol: KTabColumnMeta = {
      idx,
      name,
      comment: "",
      store_kind: ktabViewToStoreKind(insertColKind),
      view_kind: insertColKind,
      required: false,
      order_by: newOrder,
    };
    const newMeta = {
      ...tableMeta,
      columns: { ...tableMeta.columns, [name]: newCol },
    };
    await onMetaChange(newMeta);
    setInsertColTarget(null);
    setInsertColName("");
    setInsertColKind("text");
  }, [insertColTarget, insertColName, insertColKind, columns, tableMeta, onMetaChange]);

  const handleMoveColumn = useCallback(
    async (col: KTabColumnMeta, direction: "left" | "right") => {
      const sortedCols = [...columns];
      const targetIndex = sortedCols.findIndex((c) => c.idx === col.idx);
      if (direction === "left" && targetIndex === 0) return;
      if (direction === "right" && targetIndex === sortedCols.length - 1)
        return;

      const swapCol =
        direction === "left"
          ? sortedCols[targetIndex - 1]
          : sortedCols[targetIndex + 1];

      const colKey = Object.keys(tableMeta.columns).find(
        (k) => tableMeta.columns[k].name === col.name,
      );
      const swapKey = Object.keys(tableMeta.columns).find(
        (k) => tableMeta.columns[k].name === swapCol.name,
      );
      if (!colKey || !swapKey) return;

      const newMeta = {
        ...tableMeta,
        columns: {
          ...tableMeta.columns,
          [colKey]: { ...tableMeta.columns[colKey], order_by: swapCol.order_by },
          [swapKey]: {
            ...tableMeta.columns[swapKey],
            order_by: col.order_by,
          },
        },
      };
      await onMetaChange(newMeta);
    },
    [columns, tableMeta, onMetaChange],
  );

  // --- Row operations ---

  const handleAddRow = useCallback(() => {
    const newRow = makeNewRow(columns);
    pendingRowTidsRef.current.add(newRow.row_tid);
    setRows((prev) => [...prev, newRow]);
  }, [columns]);

  const handleInsertRow = useCallback(
    (targetRow: KTabRowData, position: "above" | "below") => {
      const newRow = makeNewRow(columns);
      pendingRowTidsRef.current.add(newRow.row_tid);
      setRows((prev) => {
        const idx = prev.findIndex((r) => r.row_tid === targetRow.row_tid);
        const insertAt = position === "above" ? idx : idx + 1;
        return [...prev.slice(0, insertAt), newRow, ...prev.slice(insertAt)];
      });
    },
    [columns],
  );

  // --- Navigation ---

  const getColumnNames = useCallback(
    () => columns.map((c) => c.name),
    [columns],
  );

  const navigateCell = useCallback(
    (fromRowIdx: number, fromColName: string, dir: NavigateDirection) => {
      const colNames = getColumnNames();
      const colIdx = colNames.indexOf(fromColName);
      if (colIdx === -1) return;

      let nextRowIdx = fromRowIdx;
      let nextColIdx = colIdx;

      switch (dir) {
        case "next": // Tab
          nextColIdx = colIdx + 1;
          if (nextColIdx >= colNames.length) {
            nextColIdx = 0;
            nextRowIdx = fromRowIdx + 1;
          }
          break;
        case "prev": // Shift+Tab
          nextColIdx = colIdx - 1;
          if (nextColIdx < 0) {
            nextColIdx = colNames.length - 1;
            nextRowIdx = fromRowIdx - 1;
          }
          break;
        case "down": // Enter
          nextColIdx = colIdx; // Stay in same column
          nextRowIdx = fromRowIdx + 1;
          break;
        case "up": // Shift+Enter
          nextColIdx = colIdx; // Stay in same column
          nextRowIdx = fromRowIdx - 1;
          break;
      }

      if (nextRowIdx < 0) {
        setActiveCell(null);
        return;
      }

      // If past the last row, create a new one
      if (nextRowIdx >= rows.length) {
        const newRow = makeNewRow(columns);
        pendingRowTidsRef.current.add(newRow.row_tid);
        setRows((prev) => [...prev, newRow]);
      }

      setActiveCell({
        rowIdx: nextRowIdx,
        colName: colNames[nextColIdx],
      });
    },
    [columns, getColumnNames, rows.length],
  );

  // --- Wrap toggle ---

  const toggleWrap = useCallback(async () => {
    const next = !wrapEnabled;
    setWrapEnabled(next);
    const newMeta = { ...tableMeta, wrap_enabled: next };
    await onMetaChange(newMeta);
  }, [wrapEnabled, tableMeta, onMetaChange]);

  const handleCopyCell = useCallback(async (value: unknown) => {
    await navigator.clipboard.writeText(String(value ?? ""));
  }, []);

  const handleDeleteRow = useCallback(
    async (row: KTabRowData) => {
      await ktabRowDelete({
        table_id: tableMeta.otid,
        row_tid: row.row_tid,
      });
      setRows((prev) => prev.filter((r) => r.row_tid !== row.row_tid));
      setDeleteRowTarget(null);
      // Clear active cell if it was on the deleted row
      setActiveCell((prev) => {
        if (prev && rows.findIndex((r) => r.row_tid === row.row_tid) === prev.rowIdx) {
          return null;
        }
        return prev;
      });
    },
    [tableMeta.otid, rows],
  );

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

  // --- Context menus ---

  const columnContextMenu = (col: KTabColumnMeta) => {
    const sortedCols = [...columns];
    const colIndex = sortedCols.findIndex((c) => c.idx === col.idx);
    const canMoveLeft = colIndex > 0;
    const canMoveRight = colIndex < sortedCols.length - 1;

    return (
      <ContextMenuContent>
        <ContextMenuItem
          onClick={() =>
            setInsertColTarget({ col, direction: "left" })
          }
        >
          <PlusIcon className="h-3.5 w-3.5 mr-2" />
          向左插入
        </ContextMenuItem>
        <ContextMenuItem
          onClick={() =>
            setInsertColTarget({ col, direction: "right" })
          }
        >
          <PlusIcon className="h-3.5 w-3.5 mr-2" />
          向后插入
        </ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem
          disabled={!canMoveLeft}
          onClick={() => handleMoveColumn(col, "left")}
        >
          <ArrowLeftIcon className="h-3.5 w-3.5 mr-2" />
          左移
        </ContextMenuItem>
        <ContextMenuItem
          disabled={!canMoveRight}
          onClick={() => handleMoveColumn(col, "right")}
        >
          <ArrowRightIcon className="h-3.5 w-3.5 mr-2" />
          右移
        </ContextMenuItem>
        <ContextMenuSeparator />
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
      </ContextMenuContent>
    );
  };

  const cellContextMenu = (row: KTabRowData) => (
    <ContextMenuContent>
      <ContextMenuItem onClick={() => handleInsertRow(row, "above")}>
        <PlusIcon className="h-3.5 w-3.5 mr-2" />
        上面插入一行
      </ContextMenuItem>
      <ContextMenuItem onClick={() => handleInsertRow(row, "below")}>
        <PlusIcon className="h-3.5 w-3.5 mr-2" />
        下面插入一行
      </ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem onClick={() => handleCopyCell(row)}>
        <CopyIcon className="h-3.5 w-3.5 mr-2" />
        复制单元格
      </ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem
        variant="destructive"
        onClick={() => setDeleteRowTarget(row)}
      >
        <Trash2Icon className="h-3.5 w-3.5 mr-2" />
        删除行
      </ContextMenuItem>
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
      <Button variant="ghost" size="sm" onClick={toggleWrap}>
        <WrapTextIcon className="h-3.5 w-3.5 mr-1" />
        {wrapEnabled ? "No Wrap" : "Wrap"}
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
        onClick={toggleWrap}
      >
        <WrapTextIcon className="h-3.5 w-3.5" />
      </Button>
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
        {compact && !readonly && !hideExport && compactExport}
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
      {compact && !readonly && !hideExport && compactExport}

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
              {columns.map((col) => {
                const Icon = VIEW_KIND_ICONS[col.view_kind] || TypeIcon;
                return (
                  <th
                    key={col.idx}
                    className="border px-2 py-1 text-left font-medium select-none overflow-hidden text-ellipsis whitespace-nowrap"
                  >
                    <ContextMenu>
                      <ContextMenuTrigger asChild>
                        <div className="flex items-center gap-1 cursor-default">
                          <Icon className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
                          <span>{col.name}</span>
                        </div>
                      </ContextMenuTrigger>
                      {columnContextMenu(col)}
                    </ContextMenu>
                  </th>
                );
              })}
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
                            setNewColKind(
                              e.target.value as KTabColumnViewKind,
                            )
                          }
                        >
                          {ADD_COL_TYPE_OPTIONS.map((opt) => (
                            <option key={opt.value} value={opt.value}>
                              {opt.label}
                            </option>
                          ))}
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
                  const isActive =
                    activeCell?.rowIdx === rowIdx &&
                    activeCell?.colName === col.name;
                  const inner = (
                    <Renderer
                      value={row[col.name]}
                      columnMeta={col}
                      readonly={readonly}
                      isActive={isActive}
                      onActivate={() =>
                        setActiveCell({
                          rowIdx,
                          colName: col.name,
                        })
                      }
                      onNavigate={(dir) =>
                        navigateCell(rowIdx, col.name, dir)
                      }
                      onCommit={(val) => commitCell(row, col, val)}
                      onColumnChange={(patch) =>
                        updateColumn(col.name, patch)
                      }
                    />
                  );
                  const tdClasses = [
                    "border",
                    "px-0",
                    "py-0",
                    "overflow-hidden",
                    wrapEnabled
                      ? "whitespace-pre-wrap break-all"
                      : "text-ellipsis whitespace-nowrap",
                    isActive
                      ? "ring-2 ring-primary ring-inset bg-primary/5"
                      : "",
                  ]
                    .filter(Boolean)
                    .join(" ");
                  return (
                    <td key={col.idx} className={tdClasses}>
                      {readonly ? (
                        inner
                      ) : (
                        <ContextMenu>
                          <ContextMenuTrigger asChild>
                            <div className="w-full h-full">{inner}</div>
                          </ContextMenuTrigger>
                          {cellContextMenu(row)}
                        </ContextMenu>
                      )}
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

      {/* Insert column dialog */}
      <Dialog
        open={insertColTarget !== null}
        onOpenChange={(open) => {
          if (!open) setInsertColTarget(null);
        }}
      >
        <DialogContent className="sm:max-w-sm">
          <DialogHeader>
            <DialogTitle>
              向{insertColTarget?.direction === "left" ? "左" : "后"}插入列
            </DialogTitle>
          </DialogHeader>
          <Input
            placeholder="列名"
            value={insertColName}
            onChange={(e) => setInsertColName(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") handleInsertColumn();
            }}
            autoFocus
          />
          <select
            className="w-full border rounded px-2 py-1 text-sm bg-background"
            value={insertColKind}
            onChange={(e) =>
              setInsertColKind(e.target.value as KTabColumnViewKind)
            }
          >
            {ADD_COL_TYPE_OPTIONS.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setInsertColTarget(null)}
            >
              取消
            </Button>
            <Button onClick={handleInsertColumn}>插入</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete row confirmation dialog */}
      <Dialog
        open={deleteRowTarget !== null}
        onOpenChange={(open) => {
          if (!open) setDeleteRowTarget(null);
        }}
      >
        <DialogContent className="sm:max-w-sm">
          <DialogHeader>
            <DialogTitle>确认删除</DialogTitle>
          </DialogHeader>
          <p className="text-sm text-muted-foreground">
            确定要删除此行吗？此操作无法撤销。
          </p>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteRowTarget(null)}>
              取消
            </Button>
            <Button
              variant="destructive"
              onClick={() =>
                deleteRowTarget && handleDeleteRow(deleteRowTarget)
              }
            >
              删除
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
