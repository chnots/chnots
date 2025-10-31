import { useCallback, useEffect, useRef, useState } from "react";

import { SaveState } from "@/common/types";
import ExcalidrawEditor from "@/krate/tool/excalidraw/component/excalidraw-editor";
import { Button } from "@/common/component/ui/button";
import ExcalidrawPreview from "@/krate/tool/excalidraw/component/excalidraw-preview";
import {
  ExcalidrawChnotState,
  fetchExcalidraw,
  saveExcalidraw,
  SaveFileCache,
} from "@/krate/tool/excalidraw/service";
import { RichPropProps } from "./rich-chnot";
import Fullscreen from "./fullscreen";

const ExcalidrawBlock = ({
  otid,
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
      .catch((err) => {
        console.error("unable to load excalidraw", err);
      });
  }, []);

  const directlySave = useCallback(
    (state: ExcalidrawChnotState, contentType: string) => {
      saveExcalidraw({
        savedFilesRef,
        state,
        contentType: contentType,
        onSuccess: () => {
          setState(state);
          onPostSave({
            saveState: SaveState.Saved,
          });
        },
        onFail: () => {
          onPostSave({ saveState: SaveState.Error });
        },
        otid: otid,
      });
    },
    [otid],
  );

  return (
    <div className="w-full flex flex-col">
      <div className="flex h-auto justify-center">
        <ExcalidrawPreview state={state} className="w-8/12" />
      </div>
      {fullscreen && (
        <Fullscreen onFullscreen={onSetFullscreen}>
          <ExcalidrawEditor
            otid={otid}
            state={state}
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

export default ExcalidrawBlock;
