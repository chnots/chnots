import { exportToBlob, exportToSvg } from "@excalidraw/excalidraw";
import { FileImage, Image } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { SaveState } from "@/common/types";
import ExcalidrawEditor from "@/krate/graph/excalidraw/component/excalidraw-editor";
import ExcalidrawPreview from "@/krate/graph/excalidraw/component/excalidraw-preview";
import {
  type ExcalidrawChnotState,
  fetchExcalidraw,
  type SaveFileCache,
  saveExcalidraw,
  unionFileSaved,
} from "@/krate/graph/excalidraw/service";
import { ChnotKind } from "../../po";
import { chnotHeadStore } from "../../store";
import Fullscreen from "./fullscreen";
import type { RichPropProps } from "./rich-mdwt-side";

const ExcalidrawChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
  showEditWhenEmpty,
}: RichPropProps & {
  showEditWhenEmpty: boolean;
}) => {
  const [state, setState] = useState<ExcalidrawChnotState>();
  const savedFilesRef = useRef(new Map<string, SaveFileCache>());

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

  useEffect(() => {
    const key = `excalidraw-${otid}`;
    const headerActions = (
      <>
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
  }, [otid, state, handleExportSvg, handleExportPng]);

  return (
    <div className="w-full flex flex-col">
      {readonly ? (
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
          <ExcalidrawEditor
            otid={otid}
            readOnly={false}
            onSave={directlySave}
          />
        </Fullscreen>
      )}
    </div>
  );
};

export default ExcalidrawChnot;
