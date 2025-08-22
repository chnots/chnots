import { TID } from "@/lib/id_util";
import { DbText, Varchar } from "@/lib/types";

export type KTabColumnStoreKind = "str" | "i64" | "f64" | "date" | "blob";

export type KTabColumnViewKind = "string" | "date" | "decimal";

// To use Decimal to avoid any lost.
type Decimal = string;

export const ktabViewToStoreKind = (
  kind: KTabColumnViewKind,
): KTabColumnStoreKind => {
  switch (kind.toLowerCase()) {
    case "string":
      return "str";
    case "integer":
      return "i64";
    case "float":
      return "f64";
    case "date":
      return "date";
    case "image":
      return "str";
    case "blob":
      return "blob";
    case "bool":
      return "i64";
    default:
      throw new Error(`unable to map ${kind}`);
  }
};

export type KTabColumnMeta = {
  idx: TID;
  name: string;
  comment: string;
  store_kind: KTabColumnStoreKind;
  view_kind: KTabColumnViewKind;
  required: boolean;
  order_by: number;
};

export type KTabMeta = {
  otid: TID;
  columns: Record<string, KTabColumnMeta>;
  table_name: Varchar<300>;
  table_comment: Varchar<1000>;
  update_time?: Date;
  real_table: boolean;
  tid: TID;
};

export type KTabCell = {
  table_id: number;
  col_tid: number;
  row_tid: number;
  cell_data: KTabCellData;
};

export type KTabCellData = { Date: Date } | { Text: string };

export type KTabCellText = {
  table_otid: TID;
  col_otid: TID;
  row_otid: TID;
  tid: TID;
  cell_data: DbText;
};

export type KTabCellDecimal = {
  table_otid: TID;
  col_otid: TID;
  row_otid: TID;
  tid: TID;
  cell_data: Decimal;
};

export type KTabCellDate = {
  table_otid: TID;
  col_otid: TID;
  row_otid: TID;
  tid: TID;
  cell_data: Date;
};
