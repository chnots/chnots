import { SaveState } from "@/common/types";
import { ChnotKind } from "@/krate/chnot/po";
import { ChnotChromeProps } from "./chrome";
import KTabChnot from "@/krate/ktab/component/ktab-container";
import { KTabMeta } from "@/krate/ktab/po";
import { KTabRowData } from "@/krate/ktab/component/editable-cell";
import { DataTable } from "@/krate/ktab/component/data-table";
import {
  ktabCellList,
  ktabMetaCommit,
  ktabMetaFetch,
} from "@/krate/ktab/service";
import { ktabGetViewValue } from "@/krate/ktab/dto";
import { genTID, genUID, TID } from "@/lib/id_util";
import { useEffect, useRef, useState } from "react";

const TableChnot = ({
  chnotOtid,
  kindId,
  onPostSave,
  isFocused,
}: ChnotChromeProps) => {
  const [meta, setMeta] = useState<KTabMeta>();

  useEffect(() => {
    (async () => {
      if (kindId) {
        const meta = await ktabMetaFetch({
          table_id: parseInt(kindId, 10),
        });
        if (meta.meta) {
          setMeta(meta.meta);
        }
        throw new Error(`find no ktab with kindId: ${kindId}`);
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
            data: {
              chnotOtid,
              kind: ChnotKind.KTab,
              kindId: meta.otid.toString(),
            },
          });
        }}
        isEditing={isFocused || true}
      />
    )
  );
};

export default TableChnot;
