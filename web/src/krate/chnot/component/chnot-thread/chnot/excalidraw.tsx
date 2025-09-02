import { useCallback, useEffect, useRef, useState } from "react";

import { SaveState } from "@/common/types";
import { genUID } from "@/lib/id_util";
import ExcalidrawEditor from "@/krate/tool/excalidraw/component/excalidraw-editor";
import { ChnotKind } from "@/krate/chnot/po";
import { Button } from "@/common/component/ui/button";
import ExcalidrawPreview from "@/krate/tool/excalidraw/component/excalidraw-preview";
import {
  ExcalidrawChnotState,
  fetchExcalidraw,
  saveExcalidraw,
} from "@/krate/tool/excalidraw/service";
import { ChnotChromeProps } from "./chrome";

const ExcalidrawBlock = ({
  otid,
  kindId: initialKindId,
  onPostSave,
}: ChnotChromeProps) => {
  const [kindId] = useState(initialKindId ?? genUID());
  const [open, setOpen] = useState(false);
  const [state, setState] = useState<ExcalidrawChnotState>();
  const savedFilesRef = useRef(new Map<string, string>());

  useEffect(() => {
    fetchExcalidraw(kindId)
      .then((state) => {
        if (state) {
          setState({
            excalidrawId: kindId,
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
            data: {
              otid: otid,
              kind: ChnotKind.ExcalidrawV1,
              kind_id: kindId,
            },
          });
        },
        onFail: () => {
          onPostSave({ saveState: SaveState.Error });
        },
      });
    },
    [kindId, otid],
  );

  return (
    <div className="w-full flex flex-col">
      <div className="flex h-auto justify-center">
        <ExcalidrawPreview state={state} className="w-8/12" />
      </div>
      <Button onClick={() => setOpen((prev) => !prev)}>Edit</Button>
      {open && (
        <div className="w-screen h-screen z-50 flex flex-col fixed bottom-0 left-0">
          <Button onClick={() => setOpen(false)}>Close</Button>
          <ExcalidrawEditor
            excalidrawId={kindId}
            state={state}
            readOnly={false}
            onSave={(state, contentType) => {
              directlySave(state, contentType);
            }}
          />
        </div>
      )}
    </div>
  );
};

export default ExcalidrawBlock;
