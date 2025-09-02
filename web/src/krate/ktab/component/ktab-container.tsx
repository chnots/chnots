import { ktabCellsRead, ktabMetaOverwrite, ktabMetaRead } from "../service";
import { KTabMeta } from "../po";
import { DataTable } from "./data-table";
import { ktabGetViewValue } from "../dto";
import { KTabRowData } from "./editable-cell";
import { genTID } from "@/lib/id_util";
import { useEffect, useState } from "react";
import { TableForm } from "./table-meta";
import { TypeOf, ZodObject, ZodString, ZodOptional, ZodTypeAny } from "zod";

const KTabChnot = ({
  kindId,
  onPostSave: onAfterSave,
  isEditing,
}: {
  kindId?: string;
  onPostSave: (meta: KTabMeta) => void;
  isEditing: boolean;
}) => {
  const [meta, setMeta] = useState<KTabMeta>();
  const loadMeta = async () => {
    if (kindId) {
      const meta = await ktabMetaRead({
        table_id: parseInt(kindId, 10),
      });
      if (meta.meta) {
        return meta.meta;
      }
      throw new Error(`find no ktab with kindId: ${kindId}`);
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
        ): Promise<KTabRowData[]> => {
          const data = await ktabCellsRead({
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
        >,
      ) => {
        const meta = {
          otid: genTID(),
          columns: {},
          table_name: values.name,
          table_comment: values.description ?? "",
          create_time: new Date(),
          real_table: false,
          tid: genTID(),
        };
        await ktabMetaOverwrite({
          meta: meta,
        });
        onAfterSave(meta);
        setMeta(meta);
      }}
    />
  );
};

export default KTabChnot;
