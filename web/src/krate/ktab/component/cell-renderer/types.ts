import type { KTabColumnMeta } from "../../po";

export type CellRendererProps = {
  value: unknown;
  columnMeta: KTabColumnMeta;
  readonly: boolean;
  onCommit: (value: unknown) => void;
  onColumnChange?: (patch: Partial<KTabColumnMeta>) => void;
};
