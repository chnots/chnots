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
  type MindElixirReactRef,
} from "@/krate/graph/mind-elixir";
import { BASE_URL } from "@/lib/request";
import { kfileInlineUpload, kfileUpload } from "@/krate/kfile/service";
import { genTID, genUID } from "@/lib/id_util";

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
  imageBlob: Blob,
): Promise<string> {
  if (!imageBlob.type.startsWith("image/")) {
    throw new TypeError(`the input is no a image: ${imageBlob.type}`);
  }

  return await blobToBase64DataUrl(imageBlob);
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
  const mindELixirRef = useRef<MindElixirReactRef>(null);

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
    [otid, onPostSave],
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
      onPaste: async (clipboardEvent) => {
        const items = clipboardEvent.clipboardData?.items;
        if (!items) return false;

        for (let i = 0; i < items.length; i++) {
          const item = items[i];
          if (item.type.indexOf("image") !== -1) {
            const blob = item.getAsFile();
            if (blob) {
              try {
                const uploadId = genUID();
                const rsp = await kfileUpload({
                  upload_id: uploadId,
                  chunk: blob,
                  filename: blob.name,
                  chunk_no: 0,
                  total_chunks: 1,
                  meta_id: uploadId,
                  content_type: blob.type,
                  filesize: blob.size,
                  last_modified: genTID() ,
                  otid: genTID(),
                  db_store: true,
                  binaryp: true
                });
                  const instance = mindELixirRef.current.instance;
                if (instance && instance?.currentNode && rsp.kfile) {
                  const image = {
                    url: rsp.kfile?.id + "/" + blob.name,
                    width: 200,
                    height: 200,
                    fit: "contain" as const,
                  };
                  instance.reshapeNode(instance.currentNode, {image})
                }
              } catch (error) {
                console.error("Failed to convert image to base64:", error);
                throw error;
              }
            }
          }
        }
      },
      imageProxy: (url: string) => {
        return `${BASE_URL}/api/v1/kfile-asset-download/${url}`;
      },
      ref: mindELixirRef,
    };
  }, [data]);

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
