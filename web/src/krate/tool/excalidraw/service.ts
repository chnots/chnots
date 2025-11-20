import { serializeAsJSON } from '@excalidraw/excalidraw';

import type { ExcalidrawElement, FileId } from '@excalidraw/excalidraw/element/types';
import type { AppState, BinaryFileData, BinaryFiles, DataURL } from '@excalidraw/excalidraw/types';
import type { RefObject } from 'react';
import type { KfileMetaFetchReqId } from '@/krate/kfile/dto';
import { kfileInlineDownload, kfileInlineUpload } from '@/krate/kfile/service';
import { genTID, type TID } from '@/lib/id_util';

export type ExcalidrawChnotState = {
  otid: TID;
  metaId: string;
  elements: readonly ExcalidrawElement[];
  appState: AppState;
  files: BinaryFiles;
};

export const fetchExcalidraw = async (
  id: KfileMetaFetchReqId,
): Promise<ExcalidrawChnotState | null> => {
  try {
    const rsp = await kfileInlineDownload({
      req_id: id,
    });
    if (!rsp.file) {
      return null;
    }
    const dataState = JSON.parse(rsp.file?.content);
    const fileMap = new Map<ExcalidrawElement['id'], BinaryFileData>();
    const elements = dataState.elements as readonly ExcalidrawElement[] | null;

    if (elements) {
      for (const element of elements) {
        if (element.type === 'image' && element.fileId) {
          try {
            const fileInlineRsp = await kfileInlineDownload({
              req_id: { ID: element.fileId },
            });

            const fileInline = fileInlineRsp.file;
            if (fileInline) {
              fileMap.set(element.fileId, {
                // @ts-expect-error
                mimeType: fileInline.content_type,
                dataURL: fileInline.content as DataURL,
                created: fileInline.tid,
                lastRetrieved: fileInline.tid,
                id: element.fileId as FileId,
              });
            }
          } catch (_error) {}
        }
      }
    }

    return {
      ...dataState,
      metaId: rsp.meta?.id,
      files: Object.fromEntries(fileMap.entries()),
      elements: dataState.elements,
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
  contentType: string;
  onSuccess: () => void;
  onFail: () => void;
  savedFilesRef: RefObject<Map<string, SaveFileCache>>;
};

export const saveExcalidraw = async (props: SaveExcalidrawProps) => {
  const { state, contentType, onSuccess, onFail, savedFilesRef } = props;

  const { elements, appState, metaId, files } = state;
  try {
    if (elements.length === 0) {
      return;
    }

    const content = serializeAsJSON(elements, appState, files, 'database');

    for (const [fileId, file] of Object.entries(files)) {
      const cache = savedFilesRef.current.get(fileId);
      const newVer = `${file.created}-${file.version}`;
      if (!cache || cache.ver !== newVer) {
        const otid = cache ? cache.otid : genTID();
        await kfileInlineUpload({
          res: {
            tid: genTID(),
            content: file.dataURL,
            sid: 'placeholder',
          },
          archor_intervals: 3600,
          meta_id: file.id,
          content_type: file.mimeType ?? 'chnot/unknown',
          otid: otid,
        });
        savedFilesRef.current.set(fileId, { ver: newVer, otid: otid });
      }
    }

    await kfileInlineUpload({
      res: {
        tid: genTID(),
        content,
        sid: 'placeholder',
      },
      archor_intervals: 3600,
      meta_id: metaId,
      content_type: contentType,
      otid: props.otid,
    });
    onSuccess();
  } catch (_err) {
    onFail();
  }
};
