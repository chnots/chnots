import React, { useState, useRef, useCallback, useEffect } from "react";

import "./excalidraw.scss";
import {
  AppState,
  BinaryFileData,
  BinaryFiles,
  DataURL,
  ExcalidrawImperativeAPI,
  ExcalidrawInitialDataState,
} from "@excalidraw/excalidraw/types";
import {
  ExcalidrawElement,
  FileId,
  NonDeletedExcalidrawElement,
  Theme,
} from "@excalidraw/excalidraw/element/types";
import {
  Excalidraw,
  serializeAsJSON,
  useHandleLibrary,
} from "@excalidraw/excalidraw";
import useDebounce from "@/hooks/use-debounce";
import { insertInlineKFile, queryInlineKFile } from "@/krate/kfile/service";
import { useSearchParams } from "react-router-dom";
import { resolvablePromise, ResolvablePromise } from "@/lib/resolve-promise";
import { useCallbackRefState } from "@/hooks/use-callback-ref-state";
import { genUID as genUID, genTID, TID } from "../../../../lib/id_util";

export type ExcalidrawProps = {
  useCustom?: (api: ExcalidrawImperativeAPI | null, customArgs?: any[]) => void;
  customArgs?: any[];
  kindId?: string;
  readOnly?: boolean;
  onAfterSave?: (tid: TID) => void;
};

const CONTENT_TYPE = "chnots/excalidraw-v1";

export default function ExcalidrawContainer({
  kindId: initialKindId,
  onAfterSave,
  useCustom,
  customArgs,
  readOnly: viewMode,
}: ExcalidrawProps) {
  const [viewModeEnabled, setViewModeEnabled] = useState(viewMode);
  const [zenModeEnabled, setZenModeEnabled] = useState(false);
  const [gridModeEnabled, setGridModeEnabled] = useState(true);
  const [theme, setTheme] = useState<Theme>("light");
  const [kindId, setKindId] = useState(initialKindId);
  console.log(`ExcalidrawContainer: kindId: ${kindId}`);

  const [searchParams] = useSearchParams();
  const [idInfo, setDrawId] = useState<{
    drawId: string;
    persisted: boolean;
  }>();

  useEffect(() => {
    setViewModeEnabled(viewMode ?? false);
  }, [viewMode]);

  useEffect(() => {
    (async () => {
      const kfileMetaId = searchParams.get("exdId") ?? kindId;
      if (kfileMetaId) {
        setDrawId({ drawId: kfileMetaId, persisted: true });
      } else {
        setDrawId({ drawId: genUID(), persisted: false });
      }
    })();
  }, [setDrawId, searchParams, kindId]);

  const [excalidrawAPI, excalidrawRefCallback] =
    useCallbackRefState<ExcalidrawImperativeAPI>();

  if (useCustom) {
    useCustom(excalidrawAPI, customArgs);
  }

  useHandleLibrary({ excalidrawAPI });

  const initialStatePromiseRef = useRef<{
    promise: ResolvablePromise<ExcalidrawInitialDataState | null>;
  }>({ promise: null! });
  if (!initialStatePromiseRef.current.promise) {
    initialStatePromiseRef.current.promise =
      resolvablePromise<ExcalidrawInitialDataState | null>();
  }
  useEffect(() => {
    if (!excalidrawAPI || !idInfo) {
      return;
    }
    if (!idInfo.persisted) {
      initialStatePromiseRef.current.promise.resolve({});
      return;
    }

    (async () => {
      console.log("load from kfile table");
      try {
        const rsp = await queryInlineKFile({
          meta_id: idInfo.drawId,
        });
        const dataState = JSON.parse(rsp.res[0].content);
        const fileMap = new Map<ExcalidrawElement["id"], BinaryFileData>();
        const elements = dataState.elements as
          | readonly ExcalidrawElement[]
          | null;

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
                  error
                );
              }
            }
          }
        }

        initialStatePromiseRef.current.promise.resolve({
          ...dataState,
          files: Object.fromEntries(fileMap.entries()),
          elements: dataState.elements,
        });
      } catch (e) {
        console.log("unable to fetch inline-kfile", idInfo, e);
        initialStatePromiseRef.current.promise.resolve({});
      }
    })();
  }, [excalidrawAPI, idInfo]);

  const savedFilesRef = useRef(new Map<string, string>());
  const onChange = useDebounce(
    (
      excalidrawId: string,
      elements: NonDeletedExcalidrawElement[],
      state: AppState,
      files: BinaryFiles,
      afterSave: (kid: string) => void
    ) => {
      if (elements.length > 0) {
        const content = serializeAsJSON(elements, state, files, "database");
        (async () => {
          for (const [fileId, file] of Object.entries(files)) {
            const ver = savedFilesRef.current.get(fileId);
            const newVar = file.created + "-" + file.version;
            console.log("save images");
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
            content_type: CONTENT_TYPE,
          });
          afterSave(excalidrawId);
        })();
      }
    },
    1200,
    true
  );

  const onLinkOpen = useCallback(
    (
      element: NonDeletedExcalidrawElement,
      event: CustomEvent<{
        nativeEvent: MouseEvent | React.PointerEvent<HTMLCanvasElement>;
      }>
    ) => {
      const link = element.link!;
      const { nativeEvent } = event.detail;
      const isNewTab = nativeEvent.ctrlKey || nativeEvent.metaKey;
      const isNewWindow = nativeEvent.shiftKey;
      const isInternalLink =
        link.startsWith("/") || link.includes(window.location.origin);
      if (isInternalLink && !isNewTab && !isNewWindow) {
        // signal that we're handling the redirect ourselves
        event.preventDefault();
        // do a custom redirect, such as passing to react-router
        // ...
      }
    },
    []
  );

  return (
    <Excalidraw
      excalidrawAPI={excalidrawRefCallback}
      initialData={initialStatePromiseRef.current.promise}
      onChange={(e1, e2, e3) => {
        onChange(idInfo?.drawId, e1, e2, e3, onAfterSave);
      }}
      viewModeEnabled={viewModeEnabled}
      zenModeEnabled={zenModeEnabled}
      gridModeEnabled={gridModeEnabled}
      theme={theme}
      validateEmbeddable={true}
      onLinkOpen={onLinkOpen}
    />
  );
}
