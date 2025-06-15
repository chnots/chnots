import { ktabGetViewValue, KTabRowsQueryRsp } from "./dto";

export type KTabViewRowData = {
  row_idx: number;
  [key: string]: any;
};

export function convertResponseToRowData(
  response: KTabRowsQueryRsp
): KTabViewRowData[] {
  const result: KTabViewRowData[] = [];

  for (const row of response.rows) {
    const rowData: KTabViewRowData = { row_idx: row.row_idx };

    for (const cell of row.cells) {
      if (cell.column_name === "row_idx") continue;
      rowData[cell.column_name] = ktabGetViewValue(cell.value);
    }

    result.push(rowData);
  }

  return result;
}
