import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { RichPropProps } from "./rich-chnot";
import Fullscreen from "./fullscreen";
import {
  fetchMindExilir,
  type MindElixirChnotData,
  saveMindExilir,
} from "@/krate/graph/mind-elixir/service";
import { SaveState } from "@/common/types";
import "mind-elixir/style.css";
import MindElixirReact, {
  type MindElixirReactProps,
  type MindElixirData,
} from "@/krate/graph/mind-elixir";
import { BASE_URL } from "@/lib/request";

export async function blobToBase64DataUrl(blob: Blob): Promise<string> {
  return new Promise<string>((resolve, reject) => {
    const reader = new FileReader();

    reader.onloadend = () => {
      if (typeof reader.result === "string") {
        resolve(reader.result);
      } else {
        reject(new Error("not valid base64 string"));
      }
    };

    reader.onerror = () => {
      reject(new Error("error occured when reading the base64"));
    };

    reader.onabort = () => {
      reject(new Error("the read action is aborted"));
    };

    reader.readAsDataURL(blob);
  });
}

export async function imageBlobToBase64DataUrl(
  imageBlob: Blob
): Promise<string> {
  if (!imageBlob.type.startsWith("image/")) {
    throw new TypeError(`the input is no a image: ${imageBlob.type}`);
  }

  return blobToBase64DataUrl(imageBlob);
}

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
          });
          savingFlag.current = false;
        },
        onFail: () => {
          onPostSave({ otid, saveState: SaveState.Error, title });
          savingFlag.current = false;
        },
      });
    },
    [otid, onPostSave]
  );

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
      onClipboard: async (
        clipboardEvent: ClipboardEvent
      ): Promise<{ url: string }> => {
        return "";
      },
      imageProxy: (url: string) => {
        return `${BASE_URL}/api/v1/kfile-inline-asset-download/${url}/d.txt`;
      },
    };
  }, []);

  return (
    <div className="w-full flex flex-col h-full">
      {readonly ? (
        <div className="flex h-full justify-center items-center w-full">
          <MindElixirReact data={data} editable={false} toolBar={false} />
        </div>
      ) : (
        <MindElixirReact {...options} />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <MindElixirReact {...options} />
        </Fullscreen>
      )}
    </div>
  );
};

export default MindMapChnot;
