import { useCallback, useEffect, useState } from "react";
import type { RichPropProps } from "./rich-chnot";
import Fullscreen from "./fullscreen";
import {
  fetchMindExilir,
  type MindElixirChnotState,
  saveMindExilir,
} from "@/krate/graph/mindmap/service";
import { SaveState } from "@/common/types";
import "mind-elixir/style.css";
import MindElixirReact, { type MindElixirData } from "@/krate/graph/mindmap";
import { genUID } from "@/lib/id_util";

const MindMapChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
}: RichPropProps) => {
  const [state, setState] = useState<MindElixirChnotState>();

  const defaultData: MindElixirData = {
    direction: 1,
    nodeData: {
      id: genUID(),
      topic: "Root",
    }
  };

  useEffect(() => {
    fetchMindExilir({ Otid: otid })
      .then((fetchedState) => {
        if (fetchedState) {
          setState(fetchedState);
        } else {
          setState({
            otid,
            metaId: genUID(),
            data: defaultData,
          });
        }
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
    <div className="w-full flex flex-col h-full">
      {readonly ? (
        <div className="flex h-full justify-center items-center w-full">
          <MindElixirReact
            data={state?.data}
            options={{ editable: false, toolBar: false }}
          />
        </div>
      ) : (
        <MindElixirReact
          data={state?.data}
          onChanged={(data) => {
            if (state) {
              directlySave({
                ...state,
                data
              }, "application/json")
            }

          }
          }
        />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <MindElixirReact
            data={state?.data}
            onChanged={(data) => {
              if (state) {
                directlySave({
                  ...state,
                  data
                }, "application/json")
              }
            }
            }
          />
        </Fullscreen>
      )}
    </div>
  );
};

export default MindMapChnot;
