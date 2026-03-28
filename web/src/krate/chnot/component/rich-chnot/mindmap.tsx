import dayjs from "dayjs";
import { FileImage, Image } from "lucide-react";
import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { SaveState } from "@/common/types";
import {
  fetchMindExilir,
  type MindElixirChnotData,
  mindElixirHistoryApplyInner,
  mindElixirHistoryFetchInner,
  mindElixirHistoryListInner,
  saveMindExilir,
} from "@/krate/graph/mind-elixir/service";
import Fullscreen from "./fullscreen";
import "mind-elixir/style.css";
import MindElixirReact, {
  type MindElixirData,
  type MindElixirReactProps,
  type MindElixirReactRef,
} from "@/krate/graph/mind-elixir";
import MindElixirPreview from "@/krate/graph/mind-elixir/preview";
import { kfileUpload } from "@/krate/kfile/service";
import { tidToDate } from "@/lib/date-utils";
import { genTID, genUID } from "@/lib/id_util";
import { BASE_URL } from "@/lib/request";
import { ChnotKind } from "../../po";
import { chnotHeadStore } from "../../store";
import HistoryHeaderActions from "../header/chnot-history-header-actions";
import type { RichPropProps } from "./rich-mdwt-side";

const MindMapChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
  showEditWhenEmpty,
  disableHeaderActions,
}: RichPropProps & { showEditWhenEmpty?: boolean }) => {
  const [data, setData] = useState<MindElixirData>();
  const [historyVersions, setHistoryVersions] = useState<number[]>([]);
  const [previewTid, setPreviewTid] = useState<number | undefined>(undefined);
  const [previewData, setPreviewData] = useState<MindElixirData | undefined>(
    undefined,
  );
  const savingFlag = useRef<boolean>(false);
  const mindELixirRef = useRef<MindElixirReactRef>(null);
  const dataCacheRef = useRef<MindElixirData>(undefined);
  const [loading, setLoading] = useState<boolean>(false);
  const formatTid = useCallback((tid: number) => {
    const date = tidToDate(tid);
    return date ? dayjs(date).format("YYMM-DD HH:mm:ss") : String(tid);
  }, []);

  useEffect(() => {
    setLoading(true);
    fetchMindExilir(otid)
      .then((fetchedState) => {
        if (fetchedState) {
          setData(fetchedState);
        }
      })
      .catch((_err) => {
        setData(undefined);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [otid]);

  const directlySave = useCallback(
    (toSaveState: MindElixirChnotData) => {
      if (savingFlag.current === true) {
        return;
      }
      savingFlag.current = true;
      const title = toSaveState.data?.nodeData.topic ?? "Empty Mindmap";
      saveMindExilir({
        data: toSaveState.data,
        otid: otid,
        onSuccess: () => {
          onPostSave({
            otid,
            saveState: SaveState.Saved,
            title,
            kind: ChnotKind.MindMapV1,
          });
          dataCacheRef.current = toSaveState.data;
          savingFlag.current = false;
        },
        onFail: () => {
          onPostSave({
            otid,
            saveState: SaveState.Error,
            title,
            kind: ChnotKind.MindMapV1,
          });
          savingFlag.current = false;
        },
      });
    },
    [otid, onPostSave],
  );

  const handleSetFullscreen = useCallback(
    (flag: boolean) => {
      if (!fullscreen) {
        setData(dataCacheRef.current);
      }
      if (onSetFullscreen) {
        onSetFullscreen(flag);
      }
    },
    [onSetFullscreen],
  );

  const handleExportPng = useCallback(() => {
    const instance = mindELixirRef.current?.instance;
    if (!instance) return;
    instance.exportPng().then((blob) => {
      if (!blob) return;
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = "mindmap.png";
      a.click();
      URL.revokeObjectURL(url);
    });
  }, []);

  const handleExportSvg = useCallback(() => {
    const instance = mindELixirRef.current?.instance;
    if (!instance) return;
    const blob = instance.exportSvg();
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "mindmap.svg";
    a.click();
    URL.revokeObjectURL(url);
  }, []);

  const loadHistoryList = useCallback(async () => {
    const rsp = await mindElixirHistoryListInner({ otid });
    setHistoryVersions(rsp.versions.map((v) => v.tid));
  }, [otid]);

  const viewHistory = useCallback(
    async (tid: number) => {
      const rsp = await mindElixirHistoryFetchInner({ otid, tid });
      if (!rsp.data) {
        return;
      }
      setPreviewTid(tid);
      setPreviewData(rsp.data);
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
    const rsp = await mindElixirHistoryApplyInner({
      otid,
      tid: previewTid,
    });
    if (rsp.data) {
      setData(rsp.data);
      dataCacheRef.current = rsp.data;
    }
    leavePreview();
  }, [otid, previewTid, leavePreview]);

  useEffect(() => {
    if (disableHeaderActions) {
      return;
    }
    const key = `mindmap-${otid}`;
    const historyActions = (
      <HistoryHeaderActions
        versions={historyVersions}
        previewing={previewData !== undefined}
        onOpenHistory={() => {
          void loadHistoryList();
        }}
        formatVersion={(tid) => formatTid(tid)}
        getVersionKey={(tid) => tid}
        onViewVersion={(tid) => viewHistory(tid)}
        onApply={applyHistory}
        onLatest={leavePreview}
      />
    );

    const headerActions = (
      <>
        {historyActions}
        <Button
          variant="ghost"
          size="icon"
          onClick={handleExportSvg}
          disabled={!data}
          title="Export SVG"
        >
          <FileImage />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          onClick={handleExportPng}
          disabled={!data}
          title="Export PNG"
        >
          <Image />
        </Button>
      </>
    );

    chnotHeadStore.getState().registerHeaderActions(key, headerActions);

    return () => {
      chnotHeadStore.getState().unregisterHeaderActions(key);
    };
  }, [
    otid,
    data,
    handleExportSvg,
    handleExportPng,
    disableHeaderActions,
    previewData,
    historyVersions,
    formatTid,
    loadHistoryList,
    viewHistory,
    applyHistory,
    leavePreview,
  ]);

  const options = useMemo<MindElixirReactProps>(() => {
    return {
      data,
      onChanged: (data) => {
        directlySave({
          otid,
          data,
        });
      },
      plugins: [],
      onPaste: async (clipboardEvent) => {
        const items = clipboardEvent.clipboardData?.items;
        if (!items) return false;

        for (let i = 0; i < items.length; i++) {
          const item = items[i];
          if (item.type.indexOf("image") !== -1) {
            const blob = item.getAsFile();
            if (blob) {
              try {
                const uploadId = genUID();
                const rsp = await kfileUpload({
                  upload_id: uploadId,
                  chunk: blob,
                  filename: blob.name,
                  chunk_no: 0,
                  total_chunks: 1,
                  meta_id: uploadId,
                  content_type: blob.type,
                  filesize: blob.size,
                  last_modified: genTID(),
                  otid: genTID(),
                  db_store: true,
                  binaryp: true,
                });
                const instance = mindELixirRef.current?.instance;
                if (instance && instance?.currentNode && rsp.kfile) {
                  const image = {
                    url: rsp.kfile?.id + "/" + blob.name,
                    width: 200,
                    height: 200,
                    fit: "contain" as const,
                  };
                  instance.reshapeNode(instance.currentNode, { image });
                }
              } catch (error) {
                console.error("Failed to convert image to base64:", error);
                throw error;
              }
            }
          }
        }
      },
      imageProxy: (url: string) => {
        return `${BASE_URL}/api/v1/kfile-asset-download/${url}`;
      },
      ref: mindELixirRef,
    };
  }, [data]);

  return (
    <div className="w-full flex flex-col h-full">
      {readonly ? (
        previewData ? (
          <div className="flex h-full justify-center items-center w-full">
            <MindElixirPreview data={previewData} />
          </div>
        ) : data ? (
          <div className="flex h-full justify-center items-center w-full">
            <MindElixirPreview data={data} />
          </div>
        ) : loading ? (
          <div>Loading</div>
        ) : (
          showEditWhenEmpty && (
            <div className="p-3">
              <Button
                onClick={() => {
                  if (onSetFullscreen) {
                    onSetFullscreen(true);
                  }
                }}
                variant={"outline"}
              >
                Click to Edit
              </Button>
            </div>
          )
        )
      ) : previewData ? (
        <MindElixirPreview data={previewData} />
      ) : (
        <MindElixirReact {...options} />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={handleSetFullscreen}>
          {previewData ? (
            <MindElixirPreview data={previewData} />
          ) : (
            <MindElixirReact {...options} />
          )}
        </Fullscreen>
      )}
    </div>
  );
};

const MindMapChnotMemo = memo(MindMapChnot);

export default MindMapChnotMemo;
