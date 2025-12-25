import { useCallback, useEffect, useRef, useState } from "react";
import type { RichPropProps } from "./rich-chnot";
import Fullscreen from "./fullscreen";
import {
  fetchMindExilir,
  type MindElixirChnotData,
  saveMindExilir,
} from "@/krate/graph/mind-elixir/service";
import { SaveState } from "@/common/types";
import "mind-elixir/style.css";
import MindElixirReact, { type MindElixirData } from "@/krate/graph/mind-elixir";

const MindMapChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
}: RichPropProps) => {
  const [data, setData] = useState<MindElixirData>();
  const savingFlag = useRef<boolean>(false);

  useEffect(() => {
    fetchMindExilir({ Otid: otid })
      .then((fetchedState) => {
        if (fetchedState) {
          setData(fetchedState);
        } else {
          setData(undefined);
        }
      })
      .catch((_err) => {
        setData(undefined);
      });
  }, [otid]);

  const directlySave = useCallback(
    (toSaveState: MindElixirChnotData) => {
      if (savingFlag.current === true) {
        return
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
          });
          savingFlag.current = false;
        },
        onFail: () => {
          onPostSave({ otid, saveState: SaveState.Error, title });
          savingFlag.current = false;
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
            data={data}
            options={{ editable: false, toolBar: false }}
          />
        </div>
      ) : (
        <MindElixirReact
          data={data}
          onChanged={(data) => {
            directlySave({
              otid,
              data
            })
          }}
        />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <MindElixirReact
            data={data}
            onChanged={(data) => {
              directlySave({
                otid,
                data
              })
            }}
          />
        </Fullscreen>
      )}
    </div>
  );
};

export default MindMapChnot;
