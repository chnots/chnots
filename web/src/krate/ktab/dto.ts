import { TID } from "@/lib/id_util";
import { KTabColumnStoreKind, KTabMeta } from "./po";

export type KTabOverwriteCellsRsp = object;

export type KTabOverwriteMetaReq = {
  meta: KTabMeta;
};

export type KTabMetaCommitRsp = object;

export type KTabMetaFetchReq = {
  table_id: TID;
};

export type KTabMetaFetchRsp = {
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
  value: unknown,
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
  cellValue?: KTabStoreValue,
): string | number | Date | null {
  if (!cellValue) {
    return null;
  } else if ("Text" in cellValue) {
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
  row_tid: number;
  column_name: string;
  value: KTabStoreValue;
};

export type KTabCellCommitReq = {
  table_id: TID;
  cells: KTabViewCell[];
};

export type KTabCellListReqFilter =
  | {
      OneRowByIdx: {
        row_tid: number;
      };
    }
  | {
      RowsByIdx: {
        row_tid_included: number;
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

export type KTabCellListReq = {
  table_id: TID;
  filter: KTabCellListReqFilter;
  must_existed?: boolean;
};

export type KTabCellListRsp = {
  rows: KTabCellListRspRow[];
};

export type KTabMetaCommitReq = {
  meta: KTabMeta;
};

export type KTabCellCommitRsp = object;

export type KTabCellListRspRow = {
  row_tid: TID;
  cells: KTabViewCell[];
};

export type KTabViewCell = {
  row_tid: TID;
  column_name: string;
  value: KTabStoreValue;
};

export type KTabCell = {
  tid: TID;
  table_otid: TID;
  col_otid: TID;
  row_otid: TID;
  cell_data: KTabStoreValue;
};
