import { type KTabCellListRsp, ktabGetViewValue } from './dto';

export type KTabViewRowData = {
  row_tid: number;
  [key: string]: any;
};

export function convertResponseToRowData(response: KTabCellListRsp): KTabViewRowData[] {
  const result: KTabViewRowData[] = [];

  for (const row of response.rows) {
    const rowData: KTabViewRowData = { row_tid: row.row_tid };

    for (const cell of row.cells) {
      if (cell.column_name === 'row_tid') continue;
      rowData[cell.column_name] = ktabGetViewValue(cell.value);
    }

    result.push(rowData);
  }

  return result;
}
