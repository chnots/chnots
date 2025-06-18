import { OmitTID, TID } from "@/lib/id_util";

export type KTabColumnStoreKind = "str" | "i64" | "f64" | "date" | "blob";

export type KTabColumnViewKind = "string" | "date" | "decimal";

// To use Decimal to avoid any lost.
type Decimal = string;

export const ktabViewToStoreKind = (
  kind: KTabColumnViewKind
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
  tid: TID;
  omit_tid?: OmitTID;
  columns: Record<string, KTabColumnMeta>;
  table_name: string;
  table_comment: string;
  update_time?: Date;
  real_table: boolean;
};                   

export type KTabCell = {
  table_id: number;
  col_idx: number;
  row_idx: number;
  omit_tid?: Date;
  cell_data: KTabCellData;
};

export type KTabCellData = { Date: Date } | { Text: string };

export type KTabCellText = {
  tid: TID;
  omit_tid?: OmitTID;
  table_id: TID;
  col_idx: TID;
  row_idx: TID;
  cell_data: string;
};                   

export type KTabCellDecimal = {
  tid: TID;
  omit_tid?: OmitTID;
  table_id: TID;
  col_idx: TID;
  row_idx: TID;
  cell_data: Decimal;
};                   

export type KTabCellDate = {
  tid: TID;
  omit_tid?: OmitTID;
  table_id: TID;
  col_idx: TID;
  row_idx: TID;
  cell_data: Date;
};                   
