import { exportToBlob, exportToSvg } from "@excalidraw/excalidraw";
import dayjs from "dayjs";
import { FileImage, Image } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { SaveState } from "@/common/types";
import ExcalidrawEditor from "@/krate/graph/excalidraw/component/excalidraw-editor";
import ExcalidrawPreview from "@/krate/graph/excalidraw/component/excalidraw-preview";
import {
  type ExcalidrawChnotState,
  excalidrawHistoryApplyInner,
  excalidrawHistoryFetchInner,
  excalidrawHistoryListInner,
  fetchExcalidraw,
  type SaveFileCache,
  saveExcalidraw,
  unionFileSaved,
} from "@/krate/graph/excalidraw/service";
import { tidToDate } from "@/lib/date-utils";
import { ChnotKind } from "../../po";
import { chnotHeadStore } from "../../store";
import HistoryHeaderActions from "../header/chnot-history-header-actions";
import Fullscreen from "./fullscreen";
import type { RichPropProps } from "./types";

const ExcalidrawChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
  showEditWhenEmpty,
  disableHeaderActions,
}: RichPropProps & {
  showEditWhenEmpty: boolean;
}) => {
  const [state, setState] = useState<ExcalidrawChnotState>();
  const [historyVersions, setHistoryVersions] = useState<number[]>([]);
  const [previewTid, setPreviewTid] = useState<number | undefined>(undefined);
  const [previewState, setPreviewState] = useState<
    ExcalidrawChnotState | undefined
  >(undefined);
  const savedFilesRef = useRef(new Map<string, SaveFileCache>());
  const formatTid = useCallback((tid: number) => {
    const date = tidToDate(tid);
    return date ? dayjs(date).format("YYMM-DD HH:mm:ss") : String(tid);
  }, []);

  useEffect(() => {
    fetchExcalidraw(otid, savedFilesRef.current)
      .then((state) => {
        if (state) {
          unionFileSaved(state.files ?? {}, savedFilesRef.current);
          setState({
            otid: otid,
            elements: state.elements,
            appState: state.appState,
            files: state.files,
          });
        }
      })
      .catch((_err) => {});
  }, [otid]);

  const directlySave = useCallback(
    async (state: ExcalidrawChnotState) => {
      const title =
        state.elements?.find((e) => e.type === "text")?.text || "Some Shapes";
      saveExcalidraw({
        savedFilesRef,
        state,
        onSuccess: () => {
          setState(state);
          onPostSave({
            otid,
            saveState: SaveState.Saved,
            title,
            kind: ChnotKind.ExcalidrawV1,
          });
        },
        onFail: () => {
          onPostSave({
            otid,
            saveState: SaveState.Error,
            title,
            kind: ChnotKind.ExcalidrawV1,
          });
        },
        otid: otid,
      });
    },
    [otid, onPostSave],
  );

  const handleExportSvg = useCallback(async () => {
    if (!state?.elements) return;
    const svg = await exportToSvg({
      elements: state.elements,
      appState: state.appState,
      files: state.files,
    });
    const svgData = new XMLSerializer().serializeToString(svg);
    const blob = new Blob([svgData], { type: "image/svg+xml" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "excalidraw.svg";
    a.click();
    URL.revokeObjectURL(url);
  }, [state]);

  const handleExportPng = useCallback(async () => {
    if (!state?.elements) return;
    const blob = await exportToBlob({
      elements: state.elements,
      appState: { ...state.appState, exportBackground: true },
      files: state.files,
      mimeType: "image/png",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "excalidraw.png";
    a.click();
    URL.revokeObjectURL(url);
  }, [state]);

  const loadHistoryList = useCallback(async () => {
    const rsp = await excalidrawHistoryListInner({ otid });
    setHistoryVersions(rsp.versions.map((v) => v.tid));
  }, [otid]);

  const viewHistory = useCallback(
    async (tid: number) => {
      const rsp = await excalidrawHistoryFetchInner({ otid, tid });
      if (!rsp.data) {
        return;
      }
      setPreviewTid(tid);
      setPreviewState({
        otid,
        elements: (rsp.data as any).elements,
        appState: rsp.data as any,
        files: undefined,
      });
    },
    [otid],
  );

  const leavePreview = useCallback(() => {
    setPreviewTid(undefined);
    setPreviewState(undefined);
  }, []);

  const applyHistory = useCallback(async () => {
    if (!previewTid) {
      return;
    }
    await excalidrawHistoryApplyInner({
      otid,
      tid: previewTid,
    });
    const latest = await fetchExcalidraw(otid, savedFilesRef.current);
    if (latest) {
      setState(latest);
    }
    leavePreview();
  }, [otid, previewTid, leavePreview]);

  useEffect(() => {
    if (disableHeaderActions) {
      return;
    }
    const key = `excalidraw-${otid}`;
    const historyActions = (
      <HistoryHeaderActions
        versions={historyVersions}
        previewing={previewState !== undefined}
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
          disabled={!state?.elements}
          title="Export SVG"
        >
          <FileImage />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          onClick={handleExportPng}
          disabled={!state?.elements}
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
    state,
    handleExportSvg,
    handleExportPng,
    disableHeaderActions,
    previewState,
    historyVersions,
    formatTid,
    loadHistoryList,
    viewHistory,
    applyHistory,
    leavePreview,
  ]);

  return (
    <div className="w-full flex flex-col">
      {previewState ? (
        <div className="flex h-auto justify-center items-center w-full">
          <ExcalidrawPreview state={previewState} className="w-8/12" />
        </div>
      ) : readonly ? (
        state ? (
          <div className="flex h-auto justify-center items-center w-full">
            <ExcalidrawPreview state={state} className="w-8/12" />
          </div>
        ) : (
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
      ) : (
        <ExcalidrawEditor otid={otid} readOnly={false} onSave={directlySave} />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          {previewState ? (
            <div className="flex h-auto justify-center items-center w-full">
              <ExcalidrawPreview state={previewState} className="w-8/12" />
            </div>
          ) : (
            <ExcalidrawEditor
              otid={otid}
              readOnly={false}
              onSave={directlySave}
            />
          )}
        </Fullscreen>
      )}
    </div>
  );
};

export default ExcalidrawChnot;
