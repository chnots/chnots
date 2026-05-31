import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";

export type KTabColumnStoreKind = "str" | "i64" | "f64" | "date" | "blob";

export type KTabColumnViewKind =
  | "text"
  | "string"
  | "number"
  | "integer"
  | "date"
  | "datetime"
  | "decimal"
  | "progress"
  | "checkbox"
  | "multi_select"
  | "image";

// To use Decimal to avoid any lost.
type Decimal = string;

export const ktabViewToStoreKind = (
  kind: KTabColumnViewKind,
): KTabColumnStoreKind => {
  switch (kind.toLowerCase()) {
    case "text":
    case "string":
      return "str";
    case "number":
    case "integer":
      return "i64";
    case "float":
    case "decimal":
      return "f64";
    case "date":
    case "datetime":
      return "date";
    case "image":
      return "str";
    case "blob":
      return "blob";
    case "bool":
    case "checkbox":
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

export type KTabDisplayAsGroup = {
  group: string;
  items: { viewKind: KTabColumnViewKind; label: string }[];
};

export const KTAB_DISPLAY_AS_MENU: KTabDisplayAsGroup[] = [
  {
    group: "文本",
    items: [
      { viewKind: "text", label: "Text" },
      { viewKind: "string", label: "String" },
    ],
  },
  {
    group: "数字",
    items: [
      { viewKind: "number", label: "Number" },
      { viewKind: "integer", label: "Integer" },
      { viewKind: "decimal", label: "Decimal" },
    ],
  },
  {
    group: "日期",
    items: [
      { viewKind: "date", label: "日期" },
      { viewKind: "datetime", label: "日期时间" },
    ],
  },
  {
    group: "其他",
    items: [
      { viewKind: "progress", label: "进度" },
      { viewKind: "checkbox", label: "Checkbox" },
      { viewKind: "multi_select", label: "枚举" },
      { viewKind: "image", label: "图片" },
    ],
  },
];

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
