import { useCallback, useEffect, useRef, useState } from "react";
import type { RichPropProps } from "./rich-chnot";
import Fullscreen from "./fullscreen";
import { fetchMindExilir, type MindElixirChnotState, saveMindExilir } from "@/krate/graph/mindmap/service";
import MindElixirReact from "mind-elixir-react";
import { SaveState } from "@/common/types";

const MindMapChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
}: RichPropProps) => {
  const [state, setState] = useState<MindElixirChnotState>();

  useEffect(() => {
    fetchMindExilir({ Otid: otid })
      .then((state) => {
    })
      .catch((_err) => { });
  }, [otid]);

  const directlySave = useCallback(
    (state: MindElixirChnotState, contentType: string) => {
      const title = state.data.nodeData.topic;
      saveMindExilir({
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
      });
    },
    [otid, onPostSave],
  );

  return (
    <div className="w-full flex flex-col">
      {readonly ? (
        <div className="flex h-auto justify-center items-center w-full">
        </div>
      ) : (
        <MindElixirReact />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <MindElixirReact />
        </Fullscreen>
      )}
    </div>
  );
};

export default MindMapChnot;
