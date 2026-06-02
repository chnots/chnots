import { DownloadIcon, Maximize2Icon, TableIcon } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { ChnotKind } from "@/krate/chnot/po";
import { KTabTable } from "@/krate/ktab/component/ktab-table";
import type { KTabRowData } from "@/krate/ktab/component/ktab-table";
import { exportCsv } from "@/krate/ktab/component/csv-export";
import { ktabGetViewValue } from "@/krate/ktab/dto";
import type { KTabMeta } from "@/krate/ktab/po";
import {
  ktabCellList,
  ktabMetaCommit,
} from "@/krate/ktab/service";
import type { TID } from "@/lib/id_util";

import { useMdwtThreadStore } from "./mdwt-thread-store";

interface KTabInlineWidgetProps {
  otid: TID;
  kindData: KTabMeta;
  onItemClick: (otid: TID, kind: ChnotKind) => void;
}

const KTabInlineWidget = ({
  otid,
  kindData,
  onItemClick,
}: KTabInlineWidgetProps) => {
  const selectedItem = useMdwtThreadStore((s) => s.selectedItem);
  const isSelected = selectedItem?.otid === otid;
  const [meta, setMeta] = useState<KTabMeta>(kindData);

  // Sync kindData prop when it changes (e.g., after fullscreen edit triggers refresh)
  useEffect(() => {
    setMeta(kindData);
  }, [kindData]);

  const handleExpand = useCallback(() => {
    onItemClick(otid, ChnotKind.KTab);
  }, [otid, onItemClick]);

  const fetchData = useCallback(
    async (tableId: number, start: number, size: number) => {
      const data = await ktabCellList({
        table_id: tableId,
        filter: {
          RowsByIdx: { row_tid_included: start, page_size: size },
        },
      });
      return data.rows.map((row) =>
        row.cells.reduce<KTabRowData>(
          (acc, cell) => {
            acc[cell.column_name] = ktabGetViewValue(cell.value);
            return acc;
          },
          { row_tid: row.row_tid },
        ),
      );
    },
    [],
  );

  const handleExport = useCallback(async () => {
    await exportCsv(meta, fetchData);
  }, [meta, fetchData]);

  const handleMetaChange = useCallback(
    async (newMeta: KTabMeta) => {
      await ktabMetaCommit({ meta: newMeta });
      setMeta(newMeta);
    },
    [],
  );

  return (
    <div className="my-2 mx-1 rounded border bg-muted/30 overflow-hidden">
      <div className="flex items-center h-8 px-2 gap-2 bg-muted/50 border-b">
        <TableIcon className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
        <div className="flex-1" />
        {!isSelected && (
          <Button
            variant="ghost"
            size="icon"
            className="h-6 w-6"
            onClick={handleExport}
          >
            <DownloadIcon className="h-3.5 w-3.5" />
          </Button>
        )}
        <Button
          variant={isSelected ? "secondary" : "ghost"}
          size="icon"
          className="h-6 w-6"
          onClick={handleExpand}
        >
          <Maximize2Icon className="h-3.5 w-3.5" />
        </Button>
      </div>
      {!isSelected && (
        <div className="max-h-[300px] overflow-hidden">
          <KTabTable
            tableMeta={meta}
            fetchData={fetchData}
            readonly={false}
            onMetaChange={handleMetaChange}
            compact
            hideExport
          />
        </div>
      )}
    </div>
  );
};

export default KTabInlineWidget;
