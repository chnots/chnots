import { SaveState } from "@/common/types";
import { ChnotChromeProps } from "./rich-chnot";
import { KTabMeta } from "@/krate/ktab/po";
import { KTabRowData } from "@/krate/ktab/component/editable-cell";
import { DataTable } from "@/krate/ktab/component/data-table";
import {
  ktabCellList,
  ktabMetaCommit,
  ktabMetaFetch,
} from "@/krate/ktab/service";
import { ktabGetViewValue } from "@/krate/ktab/dto";
import { genTID, genUID } from "@/lib/id_util";
import { useEffect, useState } from "react";

const TableChnot = ({ otid, onPostSave, readonly }: ChnotChromeProps) => {
  const [meta, setMeta] = useState<KTabMeta>();

  useEffect(() => {
    (async () => {
      if (otid) {
        const meta = await ktabMetaFetch({
          table_id: otid,
        });
        if (meta.meta) {
          setMeta(meta.meta);
        }
        throw new Error(`find no ktab with kindId: ${otid}`);
      } else {
        setMeta({
          otid: genTID(),
          columns: {},
          table_name: genUID(),
          table_comment: genUID(),
          real_table: false,
          tid: genTID(),
        });
      }
    })();
  }, []);

  return (
    meta && (
      <DataTable
        tableMeta={meta}
        fetchData={async (
          table_id: number,
          start: number,
          size: number,
        ): Promise<KTabRowData[]> => {
          const data = await ktabCellList({
            table_id,
            filter: {
              RowsByIdx: {
                row_tid_included: start,
                page_size: size,
              },
            },
          });
          return data.rows.map((row) => {
            return row.cells.reduce<KTabRowData>(
              (acc, cell) => {
                acc[cell.column_name] = ktabGetViewValue(cell.value);
                return acc;
              },
              { row_tid: row.row_tid },
            );
          });
        }}
        onMetaChange={async (meta: KTabMeta) => {
          await ktabMetaCommit({
            meta: meta,
          });
          setMeta(meta);
          onPostSave({
            saveState: SaveState.Saved,
          });
        }}
        isEditing={readonly || true}
      />
    )
  );
};

export default TableChnot;
