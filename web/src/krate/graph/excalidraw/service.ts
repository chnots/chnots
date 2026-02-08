import { serializeAsJSON } from "@excalidraw/excalidraw";

import type {
  ExcalidrawElement,
  FileId,
} from "@excalidraw/excalidraw/element/types";
import type {
  AppState,
  BinaryFileData,
  BinaryFiles,
  DataURL,
} from "@excalidraw/excalidraw/types";
import type { RefObject } from "react";
import { inlineKFileDownload, inlineKFileUpload } from "@/krate/kfile/service";
import { genTID, type TID } from "@/lib/id_util";
import request from "@/lib/request";
import type {
  ExcalidrawCommitReq,
  ExcalidrawCommitRsp,
  ExcalidrawFetchReq,
  ExcalidrawFetchRsp,
} from "../dto";
import type { ImportedDataState } from "@excalidraw/excalidraw/data/types";

export type ExcalidrawChnotState = {
  otid: TID;
  elements?: ExcalidrawElement[] | null;
  appState?: Partial<AppState>;
  files?: BinaryFiles;
  fileOtids?: Map<string, TID>;
};

export const excalidrawFetchInner = async (
  req: ExcalidrawFetchReq,
): Promise<ExcalidrawFetchRsp> => {
  return await request.postJson(`api/v1/excalidraw-fetch`, req);
};

export const excalidrawCommitInner = async (
  req: ExcalidrawCommitReq,
): Promise<ExcalidrawCommitRsp> => {
  return await request.postJson(`api/v1/excalidraw-commit`, req);
};

export const fetchExcalidraw = async (
  otid: TID,
  savedFileMaps?: Map<string, SaveFileCache>,
): Promise<ExcalidrawChnotState | null> => {
  try {
    const rsp = await excalidrawFetchInner({
      otid: otid,
    });
    if (!rsp.data) {
      return null;
    }
    const dataState: ImportedDataState = rsp.data;
    const fileMap = new Map<ExcalidrawElement["id"], BinaryFileData>();
    const elements = dataState.elements as readonly ExcalidrawElement[] | null;

    if (elements) {
      for (const element of elements) {
        if (element.type === "image" && element.fileId) {
          try {
            const fileInlineRsp = await inlineKFileDownload({
              req_id: { Id: element.fileId },
            });

            const fileInline = fileInlineRsp.file;
            if (fileInline && fileInlineRsp.meta) {
              const data: BinaryFileData = {
                // @ts-expect-error
                mimeType: fileInline.content_type,
                dataURL: fileInline.content as DataURL,
                created: fileInline.tid,
                lastRetrieved: fileInline.tid,
                id: element.fileId as FileId,
              };
              fileMap.set(element.fileId, data);

              const newVer = `${data.created}-${data.version}`;

              savedFileMaps?.set(element.fileId, {
                ver: newVer,
                otid: fileInlineRsp.meta.otid,
              });
            }
          } catch (_error) {}
        }
      }
    }

    return {
      ...dataState,
      appState: dataState.appState ? { ...dataState.appState } : undefined,
      otid: otid,
      files: Object.fromEntries(fileMap.entries()),
      elements: dataState.elements ? [...dataState.elements] : [],
    };
  } catch (_e) {}
  return null;
};

export type SaveFileCache = {
  ver: string;
  otid: TID;
};

export type SaveExcalidrawProps = {
  otid: TID;
  state: ExcalidrawChnotState;
  onSuccess: () => void;
  onFail: () => void;
  savedFilesRef: RefObject<Map<string, SaveFileCache>>;
};

export const unionFileSaved = async (
  files: BinaryFiles,
  savedFiles: Map<string, SaveFileCache>,
) => {
  for (const [fileId, file] of Object.entries(files)) {
    const cache = savedFiles.get(fileId);
    const newVer = `${file.created}-${file.version}`;
    if (!cache || cache.ver !== newVer) {
      const otid = cache ? cache.otid : genTID();
      await inlineKFileUpload({
        res: {
          tid: genTID(),
          content: file.dataURL,
          sid: "placeholder",
        },
        archor_intervals: 3600,
        meta_id: file.id,
        content_type: file.mimeType ?? "chnot/unknown",
        otid: otid,
        binaryp: true,
      });
      savedFiles.set(fileId, { ver: newVer, otid: otid });
    }
  }
};
export const saveExcalidraw = async (props: SaveExcalidrawProps) => {
  console.log("begin to save excalidraw");
  const { state, onSuccess, onFail, savedFilesRef } = props;

  const { elements, appState, files } = state;
  try {
    if (!elements || elements.length === 0) {
      return;
    }

    const content = serializeAsJSON(
      elements,
      appState ?? {},
      files ?? {},
      "database",
    );

    unionFileSaved(files ?? {}, savedFilesRef.current);

    await excalidrawCommitInner({
      otid: props.otid,
      data: content as Object,
    });
    onSuccess();
  } catch (_err) {
    onFail();
  }
};
