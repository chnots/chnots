import type { ParsedTable } from "../../codemirror";
import {
  generateMarkdownTable,
  parseMarkdownTable,
} from "../../codemirror";
import { Plus, Trash2 } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogTitle,
} from "@/common/component/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/common/component/ui/tooltip";

interface TableEditorDialogProps {
  open: boolean;
  rawText: string;
  onOpenChange: (open: boolean) => void;
  onSave: (newText: string) => void;
}

export function TableEditorDialog({
  open,
  rawText,
  onOpenChange,
  onSave,
}: TableEditorDialogProps) {
  const [table, setTable] = useState<ParsedTable | null>(null);
  const [hoveredRow, setHoveredRow] = useState<number | null>(null);
  const [hoveredCol, setHoveredCol] = useState<number | null>(null);
  const tableRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (open && rawText) {
      const parsed = parseMarkdownTable(rawText);
      setTable(parsed);
    }
  }, [open, rawText]);

  const updateCell = useCallback(
    (rowIdx: number, colIdx: number, value: string) => {
      setTable((prev) => {
        if (!prev) return prev;
        const next = structuredClone(prev);
        if (rowIdx === -1) {
          next.headers[colIdx] = value;
        } else {
          next.rows[rowIdx][colIdx] = value;
        }
        return next;
      });
    },
    [],
  );

  const addRow = useCallback((afterIdx: number) => {
    setTable((prev) => {
      if (!prev) return prev;
      const next = structuredClone(prev);
      const newRow = Array.from({ length: next.headers.length }, () => "");
      next.rows.splice(afterIdx + 1, 0, newRow);
      return next;
    });
  }, []);

  const addColumn = useCallback((afterIdx: number) => {
    setTable((prev) => {
      if (!prev) return prev;
      const next = structuredClone(prev);
      next.headers.splice(afterIdx + 1, 0, "");
      next.alignments.splice(afterIdx + 1, 0, "left");
      for (const row of next.rows) {
        row.splice(afterIdx + 1, 0, "");
      }
      return next;
    });
  }, []);

  const deleteRow = useCallback((idx: number) => {
    setTable((prev) => {
      if (!prev) return prev;
      const next = structuredClone(prev);
      next.rows.splice(idx, 1);
      return next;
    });
  }, []);

  const deleteColumn = useCallback((idx: number) => {
    setTable((prev) => {
      if (!prev) return prev;
      if (prev.headers.length <= 1) return prev;
      const next = structuredClone(prev);
      next.headers.splice(idx, 1);
      next.alignments.splice(idx, 1);
      for (const row of next.rows) {
        row.splice(idx, 1);
      }
      return next;
    });
  }, []);

  const handleSave = useCallback(() => {
    if (!table) return;
    const md = generateMarkdownTable(table);
    onSave(md);
    onOpenChange(false);
  }, [table, onSave, onOpenChange]);

  if (!table) return null;

  const colCount = table.headers.length;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-4xl max-h-[80vh] flex flex-col gap-3 p-4">
        <DialogTitle className="text-base font-semibold">
          Edit Table
        </DialogTitle>

        <div className="flex-1 overflow-auto" ref={tableRef}>
          <table className="te-table w-full border-collapse">
            <thead>
              <tr>
                <th className="te-corner-cell" />
                {table.headers.map((header, colIdx) => (
                  <th
                    key={colIdx}
                    className="te-th relative group"
                    onMouseEnter={() => setHoveredCol(colIdx)}
                    onMouseLeave={() => setHoveredCol(null)}
                  >
                    <input
                      className="te-header-input"
                      value={header}
                      onChange={(e) => updateCell(-1, colIdx, e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter") {
                          (e.target as HTMLInputElement).blur();
                        }
                      }}
                    />
                    {hoveredCol === colIdx && colCount > 1 && (
                      <button
                        type="button"
                        className="te-col-action te-col-delete"
                        onClick={() => deleteColumn(colIdx)}
                      >
                        <Tooltip>
                          <TooltipTrigger asChild>
                            <Trash2 className="size-3" />
                          </TooltipTrigger>
                          <TooltipContent side="top">
                            Delete column
                          </TooltipContent>
                        </Tooltip>
                      </button>
                    )}
                    {hoveredCol === colIdx && (
                      <button
                        type="button"
                        className="te-col-action te-col-add"
                        onClick={() => addColumn(colIdx)}
                      >
                        <Tooltip>
                          <TooltipTrigger asChild>
                            <Plus className="size-3" />
                          </TooltipTrigger>
                          <TooltipContent side="top">
                            Add column right
                          </TooltipContent>
                        </Tooltip>
                      </button>
                    )}
                  </th>
                ))}
                <th className="te-corner-cell" />
              </tr>
            </thead>
            <tbody>
              {table.rows.map((row, rowIdx) => (
                <tr
                  key={rowIdx}
                  onMouseEnter={() => setHoveredRow(rowIdx)}
                  onMouseLeave={() => setHoveredRow(null)}
                >
                  <td className="te-row-action-cell">
                    {hoveredRow === rowIdx && (
                      <div className="te-row-actions">
                        <button
                          type="button"
                          className="te-row-action-btn"
                          onClick={() => addRow(rowIdx)}
                        >
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Plus className="size-3" />
                            </TooltipTrigger>
                            <TooltipContent side="left">
                              Add row below
                            </TooltipContent>
                          </Tooltip>
                        </button>
                        {table.rows.length > 1 && (
                          <button
                            type="button"
                            className="te-row-action-btn te-row-delete-btn"
                            onClick={() => deleteRow(rowIdx)}
                          >
                            <Tooltip>
                              <TooltipTrigger asChild>
                                <Trash2 className="size-3" />
                              </TooltipTrigger>
                              <TooltipContent side="left">
                                Delete row
                              </TooltipContent>
                            </Tooltip>
                          </button>
                        )}
                      </div>
                    )}
                  </td>
                  {row.map((cell, colIdx) => (
                    <td key={colIdx} className="te-td">
                      <input
                        className="te-cell-input"
                        value={cell}
                        onChange={(e) =>
                          updateCell(rowIdx, colIdx, e.target.value)
                        }
                        onKeyDown={(e) => {
                          if (e.key === "Enter") {
                            (e.target as HTMLInputElement).blur();
                          }
                          if (e.key === "Tab" && !e.shiftKey) {
                            const isLastCol = colIdx === row.length - 1;
                            const isLastRow = rowIdx === table.rows.length - 1;
                            if (isLastCol && isLastRow) {
                              e.preventDefault();
                              addRow(rowIdx);
                            }
                          }
                        }}
                      />
                    </td>
                  ))}
                  <td className="te-row-action-cell te-row-action-right" />
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <div className="flex items-center justify-between pt-2 border-t">
          <span className="text-xs text-muted-foreground">
            {table.rows.length} rows × {table.headers.length} columns
          </span>
          <div className="flex gap-2">
            <button
              type="button"
              className="te-btn te-btn-secondary"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </button>
            <button
              type="button"
              className="te-btn te-btn-primary"
              onClick={handleSave}
            >
              Save
            </button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
