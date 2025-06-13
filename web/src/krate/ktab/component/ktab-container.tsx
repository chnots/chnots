import {
  ktabCellsRead,
  ktabMetaOverwrite,
  ktabMetaRead,
} from "../store/service";
import { KTabMeta } from "../store/po";
import { queryKKV } from "@/krate/kfile/store/service";
import { DataTable } from "./data-table";
import { ktabGetViewValue } from "../store/dto";
import { KTabRowData } from "./editable-cell";
import { genTID, TID } from "@/lib/id_util";
import { useEffect, useState } from "react";
import { TableForm } from "./table-meta";
import { TypeOf, ZodObject, ZodString, ZodOptional, ZodTypeAny } from "zod";

const KTabChnot = ({
  chnotMetaId,
  kspace,
  onInitialSave,
  isEditing,
}: {
  chnotMetaId?: TID;
  onInitialSave: (meta: KTabMeta) => Promise<void>;
  kspace: string;
  isEditing: boolean;
}) => {
  const [meta, setMeta] = useState<KTabMeta>();
  const loadMeta = async () => {
    if (chnotMetaId) {
      const value = await queryKKV({
        key: chnotMetaId.toString(),
        kind: "chnot_sub_type",
      });
      if (value.value) {
        const meta = await ktabMetaRead({
          table_id: parseInt(value.value, 10),
        });
        if (meta.meta) {
          return meta.meta;
        }
      }
      throw new Error(`find no ktab with chnotMetaId: ${chnotMetaId}`);
    }
  };
  useEffect(() => {
    loadMeta().then((rsp) => {
      if (rsp) {
        setMeta(rsp);
      }
    });
  }, []);

  return meta ? (
    <div className="w-full p-2">
      <DataTable
        tableMeta={meta}
        fetchData={async (
          table_id: number,
          start: number,
          size: number,
          column_name?: string
        ): Promise<KTabRowData[]> => {
          const data = await ktabCellsRead({
            table_id,
            fields: [],
            filter: {
              RowsByIdx: {
                row_idx_included: start,
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
              { row_idx: row.row_idx }
            );
          });
        }}
        onMetaChange={async (tableMeta: KTabMeta) => {
          await ktabMetaOverwrite({
            meta: tableMeta,
          });
          setMeta(tableMeta);
        }}
        isEditing={isEditing}
      />
    </div>
  ) : (
    <TableForm
      onSubmit={async (
        values: TypeOf<
          ZodObject<
            { name: ZodString; description: ZodOptional<ZodString> },
            "strip",
            ZodTypeAny,
            { name: string; description?: string | undefined },
            { name: string; description?: string | undefined }
          >
        >
      ) => {
        const meta = {
          tid: genTID(),
          columns: {},
          table_name: values.name,
          table_comment: values.description,
          create_time: new Date(),
          kspace: kspace,
          real_table: false,
        };
        await ktabMetaOverwrite({
          meta: meta,
        });
        await onInitialSave(meta);
        setMeta(meta);
      }}
    />
  );
};

export default KTabChnot;
