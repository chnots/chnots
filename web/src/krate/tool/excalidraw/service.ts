import { insertInlineKFile, queryInlineKFile } from "@/krate/kfile/service";
import { genTID } from "@/lib/id_util";
import {
  ExcalidrawElement,
  FileId,
} from "@excalidraw/excalidraw/element/types";
import { serializeAsJSON } from "@excalidraw/excalidraw";
import {
  AppState,
  BinaryFileData,
  BinaryFiles,
  DataURL,
} from "@excalidraw/excalidraw/types";
import { RefObject } from "react";

export type ExcalidrawChnotState = {
  excalidrawId: string;
  elements: readonly ExcalidrawElement[];
  appState: AppState;
  files: BinaryFiles;
};

export const fetchExcalidraw = async (
  excalidraw_id: string,
): Promise<ExcalidrawChnotState | null> => {
  try {
    const rsp = await queryInlineKFile({
      meta_id: excalidraw_id,
    });
    const dataState = JSON.parse(rsp.res[0].content);
    const fileMap = new Map<ExcalidrawElement["id"], BinaryFileData>();
    const elements = dataState.elements as readonly ExcalidrawElement[] | null;

    if (elements) {
      for (const element of elements) {
        if (element.type === "image" && element.fileId) {
          try {
            const fileInlineRsp = await queryInlineKFile({
              meta_id: element.fileId,
            });

            const fileInline = fileInlineRsp.res.at(0);
            if (fileInline) {
              fileMap.set(element.fileId, {
                // @ts-ignore
                mimeType: fileInline.content_type,
                dataURL: fileInline.content as DataURL,
                created: fileInline.tid,
                lastRetrieved: fileInline.tid,
                id: element.fileId as FileId,
              });
            }
          } catch (error) {
            console.error(
              `Failed to query inline kfile for fileId ${element.fileId}`,
              error,
            );
          }
        }
      }
    }

    return {
      ...dataState,
      files: Object.fromEntries(fileMap.entries()),
      elements: dataState.elements,
    };
  } catch (e) {
    console.error("unable to fetch inline-kfile", excalidraw_id, e);
  }
  return null;
};

export type SaveExcalidrawProps = {
  state: ExcalidrawChnotState;
  contentType: string;
  onSuccess: () => void;
  onFail: () => void;
  savedFilesRef: RefObject<Map<string, string>>;
};

export const saveExcalidraw = async (props: SaveExcalidrawProps) => {
  const { state, contentType, onSuccess, onFail, savedFilesRef } = props;

  const { elements, appState, excalidrawId, files } = state;
  try {
    if (elements.length == 0) {
      return;
    }

    const content = serializeAsJSON(elements, appState, files, "database");

    for (const [fileId, file] of Object.entries(files)) {
      const ver = savedFilesRef.current.get(fileId);
      const newVar = file.created + "-" + file.version;
      if (!ver || ver != newVar) {
        await insertInlineKFile({
          res: {
            tid: genTID(),
            content: file.dataURL,
            sid: "placeholder",
          },
          archor_intervals: 3600,
          meta_id: file.id,
          content_type: file.mimeType ?? "chnots/unknown",
        });
        savedFilesRef.current.set(fileId, newVar);
      }
    }

    await insertInlineKFile({
      res: {
        tid: genTID(),
        content,
        sid: "placeholder",
      },
      archor_intervals: 3600,
      meta_id: excalidrawId,
      content_type: contentType,
    });
    onSuccess();
  } catch (err) {
    console.error("save excalidraw", err);

    onFail();
  }
};
