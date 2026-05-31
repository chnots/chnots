import type { FC } from "react";
import { CheckboxCell } from "./checkbox-cell";
import { TextCell } from "./text-cell";
import { NumberCell } from "./number-cell";
import { DateCell } from "./date-cell";
import { MultiSelectCell } from "./multi-select-cell";
import type { CellRendererProps } from "./types";
import type { KTabColumnViewKind } from "../../po";

export const getCellRenderer = (
  viewKind: KTabColumnViewKind,
): FC<CellRendererProps> => {
  switch (viewKind) {
    case "checkbox":
      return CheckboxCell;
    case "number":
    case "integer":
    case "decimal":
      return NumberCell;
    case "date":
    case "datetime":
      return DateCell;
    case "multi_select":
      return MultiSelectCell;
    default:
      return TextCell;
  }
};
