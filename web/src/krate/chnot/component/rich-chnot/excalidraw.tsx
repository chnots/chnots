import { useCallback, useEffect, useRef, useState } from "react";
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
import Fullscreen from "./fullscreen";
import type { RichPropProps } from "./rich-mdwt-side";
import { ChnotKind } from "../../po";
import { Button } from "@/common/component/ui/button";

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
    fetchExcalidraw(otid)
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
