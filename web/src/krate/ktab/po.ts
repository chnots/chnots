export type KTabColumnStoreKind = "str" | "i64" | "f64" | "date" | "blob";

export type KTabColumnViewKind =
  | "string"
  | "date"
  | "decimal";

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
  idx: number;
  name: string;
  comment?: string;
  store_kind: KTabColumnStoreKind;
  view_kind: KTabColumnViewKind;
  required: boolean;
  order_by: number;
};

export type KTabMeta = {
  tid: number;
  columns: Record<string, KTabColumnMeta>;
  table_name: string;
  table_comment?: string;
  create_time: Date;
  update_time?: Date;
  omit_tid?: Date;
  kspace: string;
  real_table: boolean;
};

export type KTabCell = {
  table_id: number;
  col_idx: number;
  row_idx: number;
  omit_tid?: Date;
  cell_data: KTabCellData;
}

export type KTabCellData = { Date: Date } | { Text: string };
