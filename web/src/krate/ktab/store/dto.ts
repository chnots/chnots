import { KTabColumnStoreKind, KTabMeta } from "./po";

export type KTabOverwriteCellsRsp = object;

export type KTabOverwriteMetaReq = {
  meta: KTabMeta;
};

export type KTabMetaOverwriteRsp = object;

export type KTabMetaQueryReq = {
  table_id: number;
};

export type KTabMetaQueryRsp = {
  meta?: KTabMeta;
};

export type KTabViewTypeName = "string" | "date" | "demical";

export type KTabStoreValue =
  | { Text: string }
  | { I64: number }
  | { F64: number }
  | { Date: Date }
  | { Blob: Blob };

export const ktabToStoreValue = (
  kind: string,
  value: unknown
): KTabStoreValue => {
  switch (kind.toLowerCase()) {
    case "string":
      return { Text: value as string };
    case "demical":
      return { I64: value as number };
    case "date":
      return { Date: value as Date };
    default:
      throw new Error();
  }
};

export function ktabGetViewValue(
  cellValue: KTabStoreValue
): string | number | Date {
  if ("Text" in cellValue) {
    return cellValue.Text;
  } else if ("I64" in cellValue) {
    return cellValue.I64;
  } else if ("F64" in cellValue) {
    return cellValue.F64;
  } else if ("Date" in cellValue) {
    return cellValue.Date;
  } else {
    throw new Error("Invalid cell value type");
  }
}

export type KTabStoreCell = {
  row_idx: number;
  column_name: string;
  value: KTabStoreValue;
};

export type KTabCellsOverwriteReq = {
  table_id: number;
  cells: KTabStoreCell[];
};

export type KtabRowsQueryReqFilter =
  | {
      OneRowByIdx: {
        row_idx: number;
      };
    }
  | {
      RowsByIdx: {
        row_idx_included: number;
        page_size: number;
      };
    }
  | {
      FieldSortPage: {
        field_name: string;
        field_kind: KTabColumnStoreKind;
        start_included: number;
        page_size: number;
      };
    };

export type KTabRowsQueryReq = {
  table_id: number;
  fields: string[];
  filter: KtabRowsQueryReqFilter;
};

export type KTabRowsQueryRsp = {
  rows: { row_idx: number; cells: KTabStoreCell[] }[];
};
