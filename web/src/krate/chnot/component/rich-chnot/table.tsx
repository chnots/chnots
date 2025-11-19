import { useEffect, useState } from 'react';

import Fullscreen from './fullscreen';

import type { RichPropProps } from './rich-chnot';
import { SaveState } from '@/common/types';
import { DataTable } from '@/krate/ktab/component/data-table';
import type { KTabRowData } from '@/krate/ktab/component/editable-cell';
import { ktabGetViewValue } from '@/krate/ktab/dto';
import type { KTabMeta } from '@/krate/ktab/po';
import { ktabCellList, ktabMetaCommit, ktabMetaFetch } from '@/krate/ktab/service';
import { genTID, genUID } from '@/lib/id_util';

const TableChnot = ({ otid, onPostSave, readonly, fullscreen, onSetFullscreen }: RichPropProps) => {
  console.log('render TableChnot', otid, readonly);
  const [meta, setMeta] = useState<KTabMeta>();

  useEffect(() => {
    (async () => {
      const meta = await ktabMetaFetch({
        table_id: otid,
      });
      if (meta.meta) {
        setMeta(meta.meta);
      } else {
        setMeta({
          otid: otid,
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
      <>
        {fullscreen ? (
          <Fullscreen onSetFullscreen={onSetFullscreen}>
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
                  title: Object.values(meta.columns)
                    .map((e) => e.name)
                    .join('|'),
                });
              }}
              readonly={false}
            />
          </Fullscreen>
        ) : (
          <div className="flex flex-col h-full w-full p-1 max-w-full">
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
                  title: Object.values(meta.columns)
                    .map((e) => e.name)
                    .join('|'),
                });
              }}
              readonly={readonly || false}
            />
          </div>
        )}
      </>
    )
  );
};

export default TableChnot;
