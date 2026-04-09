import dayjs from "dayjs";
import { DownloadIcon } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { SaveState } from "@/common/types";
import { chnotHeadStore } from "@/krate/chnot/store";
import { KFileViewer } from "@/krate/kfile/components";
import { handleDownloadKfile } from "@/krate/kfile/components/download";
import type {
  KfileHistoryFetchRsp,
  KfileHistoryVersion,
} from "@/krate/kfile/dto";
import type { KFileMeta } from "@/krate/kfile/po";
import {
  kfileHistoryApply,
  kfileHistoryFetch,
  kfileHistoryList,
} from "@/krate/kfile/service";
import { tidToDate } from "@/lib/date-utils";
import { ChnotKind } from "../../po";
import HistoryHeaderActions from "../header/chnot-history-header-actions";
import Fullscreen from "./fullscreen";
import type { RichPropProps } from "./rich-mdwt-side";

const KFileChnot = ({
  otid,
  fullscreen,
  readonly,
  onPostSave,
  onSetFullscreen,
  disableHeaderActions,
}: RichPropProps) => {
  const [reloadSeed, setReloadSeed] = useState(0);
  const [historyVersions, setHistoryVersions] = useState<KfileHistoryVersion[]>(
    [],
  );
  const [previewTid, setPreviewTid] = useState<number | undefined>(undefined);
  const [previewData, setPreviewData] = useState<
    KfileHistoryFetchRsp | undefined
  >(undefined);
  const formatTid = useCallback((tid: number) => {
    const date = tidToDate(tid);
    return date ? dayjs(date).format("YYMM-DD HH:mm:ss") : String(tid);
  }, []);

  const loadHistoryList = useCallback(async () => {
    const rsp = await kfileHistoryList({ otid });
    setHistoryVersions(rsp.versions);
  }, [otid]);

  const viewHistory = useCallback(
    async (tid: number) => {
      const rsp = await kfileHistoryFetch({ otid, tid });
      setPreviewTid(tid);
      setPreviewData(rsp);
    },
    [otid],
  );

  const leavePreview = useCallback(() => {
    setPreviewTid(undefined);
    setPreviewData(undefined);
  }, []);

  const applyHistory = useCallback(async () => {
    if (!previewTid) {
      return;
    }
    await kfileHistoryApply({ otid, tid: previewTid });
    leavePreview();
    setReloadSeed((v) => v + 1);
  }, [otid, previewTid, leavePreview]);

  useEffect(() => {
    if (disableHeaderActions) {
      return;
    }
    const key = `kfile-history-${otid}`;
    const actions = (
      <HistoryHeaderActions
        versions={historyVersions}
        previewing={previewData !== undefined}
        onOpenHistory={() => {
          void loadHistoryList();
        }}
        formatVersion={(version) => formatTid(version.tid)}
        getVersionKey={(version) => version.tid}
        onViewVersion={(version) => viewHistory(version.tid)}
        onApply={applyHistory}
        onLatest={leavePreview}
      />
    );
    chnotHeadStore.getState().registerHeaderActions(key, actions);
    return () => {
      chnotHeadStore.getState().unregisterHeaderActions(key);
    };
  }, [
    disableHeaderActions,
    otid,
    previewData,
    historyVersions,
    formatTid,
    loadHistoryList,
    viewHistory,
    applyHistory,
    leavePreview,
  ]);

  const renderPreview = (meta: KFileMeta | undefined, content?: string) => {
    if (!meta) {
      return (
        <div className="p-3 text-sm text-muted-foreground">No history data</div>
      );
    }
    return (
      <div className="w-full h-full overflow-auto p-3 space-y-3">
        <div className="text-sm text-muted-foreground">{meta.filename}</div>
        {meta.content_type.startsWith("image/") ? (
          <img
            src={`/api/v1/kfile-asset-download/${meta.id}/${encodeURIComponent(meta.filename)}`}
            alt={meta.filename}
            className="max-w-full h-auto rounded-md border"
          />
        ) : content ? (
          <pre className="whitespace-pre-wrap break-all text-sm">{content}</pre>
        ) : (
          <Button variant="outline" onClick={() => handleDownloadKfile(meta)}>
            <DownloadIcon className="w-4 h-4" />
            Download History File
          </Button>
        )}
      </div>
    );
  };

  return (
    <div className="w-full h-full">
      {previewData ? (
        renderPreview(previewData.meta, previewData.file?.content)
      ) : (
        <KFileViewer
          key={`${otid}-${reloadSeed}`}
          otid={otid}
          onPostSave={(f) => {
            onPostSave({
              otid,
              saveState: SaveState.Saved,
              title: f.filename,
              kind: ChnotKind.KFileV1,
            });
          }}
          readonly={readonly}
          disableHeaderActions={disableHeaderActions}
        />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          {previewData ? (
            renderPreview(previewData.meta, previewData.file?.content)
          ) : (
            <KFileViewer
              key={`${otid}-${reloadSeed}-fullscreen`}
              otid={otid}
              onPostSave={(f) => {
                onPostSave({
                  otid,
                  saveState: SaveState.Saved,
                  title: f.filename,
                  kind: ChnotKind.KFileV1,
                });
              }}
              disableHeaderActions={disableHeaderActions}
            />
          )}
        </Fullscreen>
      )}
    </div>
  );
};

export default KFileChnot;
