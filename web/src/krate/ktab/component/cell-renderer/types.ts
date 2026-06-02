import type { KTabColumnMeta } from "../../po";

export type NavigateDirection = "next" | "prev" | "up" | "down";

export type CellRendererProps = {
  value: unknown;
  columnMeta: KTabColumnMeta;
  readonly: boolean;
  onCommit: (value: unknown) => void;
  onColumnChange?: (patch: Partial<KTabColumnMeta>) => void;
  /** Whether this cell is the currently active (focused/editing) cell */
  isActive: boolean;
  /** Called when the user clicks on this cell to activate it */
  onActivate: () => void;
  /** Called when the user presses Tab/Enter/Arrow keys to navigate to another cell */
  onNavigate: (dir: NavigateDirection) => void;
};
