import { OmitTID, TID } from "@/lib/id_util";
import { KTabColumnStoreKind, KTabMeta } from "./po";

export type KTabOverwriteCellsRsp = object;

export type KTabOverwriteMetaReq = {
  meta: KTabMeta;
};

export type KTabMetaOverwriteRsp = object;                      

export type KTabMetaQueryReq = {
  table_id: TID;
};                       

export type KTabMetaQueryRsp = {
  meta?: KTabMeta;
};                       

export type KTabViewTypeName = "string" | "date" | "demical";

export type KTabStoreValue =
  | { Text: string }
  | { Decimal: string }
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
      return { Decimal: value as string };
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
  } else if ("Decimal" in cellValue) {
    return cellValue.Decimal;
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
  table_id: TID;
  cells: KTabViewCell[];
};                       

export type KTabRowsQueryReqFilter =
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
  table_id: TID;
  filter: KTabRowsQueryReqFilter;
};                       

export type KTabRowsQueryRsp = {
  rows: KTabRowsQueryRspRow[];
};                       

export type KTabMetaOverwriteReq = {
  meta: KTabMeta;
};                       

export type KTabCellsOverwriteRsp = object;                      

export type KTabRowsQueryRspRow = {
  row_idx: TID;
  cells: KTabViewCell[];
};                       

export type KTabViewCell = {
  row_idx: TID;
  column_name: string;
  value: KTabStoreValue;
};                       

export type KTabCell = {
  tid: TID;
  table_id: TID;
  col_idx: TID;
  row_idx: TID;
  omit_tid?: OmitTID;
  cell_data: KTabStoreValue;
};                      