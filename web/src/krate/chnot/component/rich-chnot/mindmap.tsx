import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
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
import { kfileUpload } from "@/krate/kfile/service";
import { genTID, genUID } from "@/lib/id_util";
import MindElixirPreview from "@/krate/graph/mind-elixir/preview";
import type { RichPropProps } from "./rich-mdwt-side";
import { ChnotKind } from "../../po";
import { Button } from "@/common/component/ui/button";

const MindMapChnot = ({
  otid,
  readonly,
  fullscreen,
  onPostSave,
  onSetFullscreen,
  showEditWhenEmpty,
}: RichPropProps & { showEditWhenEmpty?: boolean }) => {
  const [data, setData] = useState<MindElixirData>();
  const savingFlag = useRef<boolean>(false);
  const mindELixirRef = useRef<MindElixirReactRef>(null);
  const dataCacheRef = useRef<MindElixirData>(undefined);
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    setLoading(true);
    fetchMindExilir(otid)
      .then((fetchedState) => {
        if (fetchedState) {
          setData(fetchedState);
        }
      })
      .catch((_err) => {
        setData(undefined);
      })
      .finally(() => {
        setLoading(false);
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
            kind: ChnotKind.MindMapV1,
          });
          dataCacheRef.current = toSaveState.data;
          savingFlag.current = false;
        },
        onFail: () => {
          onPostSave({
            otid,
            saveState: SaveState.Error,
            title,
            kind: ChnotKind.MindMapV1,
          });
          savingFlag.current = false;
        },
      });
    },
    [otid, onPostSave],
  );

  const handleSetFullscreen = useCallback(
    (flag: boolean) => {
      if (!fullscreen) {
        setData(dataCacheRef.current);
      }
      if (onSetFullscreen) {
        onSetFullscreen(flag);
      }
    },
    [onSetFullscreen],
  );

  const options = useMemo<MindElixirReactProps>(() => {
    return {
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
                  last_modified: genTID(),
                  otid: genTID(),
                  db_store: true,
                  binaryp: true,
                });
                const instance = mindELixirRef.current?.instance;
                if (instance && instance?.currentNode && rsp.kfile) {
                  const image = {
                    url: rsp.kfile?.id + "/" + blob.name,
                    width: 200,
                    height: 200,
                    fit: "contain" as const,
                  };
                  instance.reshapeNode(instance.currentNode, { image });
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
        data ? (
          <div className="flex h-full justify-center items-center w-full">
            <MindElixirPreview data={data} />
          </div>
        ) : loading ? (
          <div>Loading</div>
        ) : (
          showEditWhenEmpty && (
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
        )
      ) : (
        <MindElixirReact {...options} />
      )}
      {fullscreen && (
        <Fullscreen onSetFullscreen={handleSetFullscreen}>
          <MindElixirReact {...options} />
        </Fullscreen>
      )}
    </div>
  );
};

const MindMapChnotMemo = memo(MindMapChnot);

export default MindMapChnotMemo;
