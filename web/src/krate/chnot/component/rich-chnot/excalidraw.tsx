import { useCallback, useEffect, useRef, useState } from "react";
import { SaveState } from "@/common/types";
import ExcalidrawEditor from "@/krate/tool/excalidraw/component/excalidraw-editor";
import ExcalidrawPreview from "@/krate/tool/excalidraw/component/excalidraw-preview";
import {
  type ExcalidrawChnotState,
  fetchExcalidraw,
  type SaveFileCache,
  saveExcalidraw,
} from "@/krate/tool/excalidraw/service";
import Fullscreen from "./fullscreen";
import type { RichPropProps } from "./rich-chnot";

const ExcalidrawChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
}: RichPropProps) => {
  const [state, setState] = useState<ExcalidrawChnotState>();
  const savedFilesRef = useRef(new Map<string, SaveFileCache>());

  useEffect(() => {
    fetchExcalidraw({ Otid: otid })
      .then((state) => {
        if (state) {
          setState({
            otid: otid,
            metaId: state.metaId,
            elements: state.elements,
            appState: state.appState,
            files: state.files,
          });
        }
      })
      .catch((_err) => {});
  }, [otid]);

  const directlySave = useCallback(
    (state: ExcalidrawChnotState, contentType: string) => {
      const title =
        state.elements.find((e) => e.type === "text")?.text || "Some Shapes";
      saveExcalidraw({
        savedFilesRef,
        state,
        contentType: contentType,
        onSuccess: () => {
          setState(state);
          onPostSave({
            otid,
            saveState: SaveState.Saved,
            title,
          });
        },
        onFail: () => {
          onPostSave({ otid, saveState: SaveState.Error, title });
        },
        otid: otid,
      });
    },
    [otid, onPostSave],
  );

  return (
    <div className="w-full flex flex-col">
      {readonly ? (
        <div className="flex h-auto justify-center items-center w-full">
          <ExcalidrawPreview state={state} className="w-8/12" />
        </div>
      ) : (
        <ExcalidrawEditor
          otid={otid}
          readOnly={false}
          onSave={(state, contentType) => {
            directlySave(state, contentType);
          }}
        />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <ExcalidrawEditor
            otid={otid}
            readOnly={false}
            onSave={(state, contentType) => {
              directlySave(state, contentType);
            }}
          />
        </Fullscreen>
      )}
    </div>
  );
};

export default ExcalidrawChnot;
