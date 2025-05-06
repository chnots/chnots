import React, { useState, useRef, useCallback, useEffect } from "react";
import { v4 as uuid } from "uuid";

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
  MIME_TYPES,
  serializeAsJSON,
  useHandleLibrary,
} from "@excalidraw/excalidraw";
import useDebounce from "@/hooks/use-debounce";
import {
  insertInlineResource,
  queryInlineResource,
} from "@/store/resource/service";
import { useNamespaceStore } from "@/store/namespace";
import { useSearchParams } from "react-router-dom";
import { resolvablePromise, ResolvablePromise } from "@/utils/resolve-promise";
import md5 from "crypto-js/md5";
import { useCallbackRefState } from "@/hooks/use-callback-ref-state";

export interface ExcalidrawProps {
  useCustom?: (api: ExcalidrawImperativeAPI | null, customArgs?: any[]) => void;
  customArgs?: any[];
  instanceId?: string;
}

const CONTENT_TYPE = "excalidraw-v1";

export default function ExcalidrawContainer({
  useCustom,
  customArgs,
  instanceId,
}: ExcalidrawProps) {
  const { currentNamespace } = useNamespaceStore();

  const [viewModeEnabled, setViewModeEnabled] = useState(false);
  const [zenModeEnabled, setZenModeEnabled] = useState(false);
  const [gridModeEnabled, setGridModeEnabled] = useState(false);
  const [theme, setTheme] = useState<Theme>("light");

  const [searchParams] = useSearchParams();
  const [excalidrawId, setExcalidrawId] = useState<string>();
  useEffect(() => {
    let id = instanceId ? instanceId : searchParams.get("exdId");
    id = id ?? uuid();
    setExcalidrawId(id);
  }, [setExcalidrawId, searchParams]);

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
    if (!excalidrawAPI) {
      return;
    }
    const fetchData = async () => {
      const rsp = await queryInlineResource({
        rid: excalidrawId,
      });
      try {
        const dataState = JSON.parse(rsp.res[0].content);
        const fileMap = new Map<ExcalidrawElement["id"], BinaryFileData>();
        const eles = dataState.elements as readonly ExcalidrawElement[] | null;

        if (eles) {
          for (const element of eles) {
            if (element.type === "image" && element.fileId) {
              try {
                const fileInlineRsp = await queryInlineResource({
                  rid: element.fileId,
                });

                const fileInline = fileInlineRsp.res.at(0);
                if (fileInline) {
                  fileMap.set(element.fileId, {
                    // @ts-ignore
                    mimeType: fileInline.content_type,
                    id: fileInline.rid as FileId,
                    dataURL: fileInline.content as DataURL,
                    created: fileInline.insert_time?.getTime(),
                    lastRetrieved: fileInline.insert_time?.getTime(),
                  });
                }
              } catch (error) {
                console.error(
                  `Failed to query inline resource for fileId ${element.fileId}`,
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
        console.log("unable to fetch inline-resource", excalidrawId, e);
        initialStatePromiseRef.current.promise.resolve({});
      }
    };
    fetchData();
  }, [excalidrawAPI, MIME_TYPES]);

  const savedFilesRef = useRef(new Map<string, string>());
  const onChange = useDebounce(
    (
      elements: NonDeletedExcalidrawElement[],
      state: AppState,
      files: BinaryFiles
    ) => {
      const content = serializeAsJSON(elements, state, files, "database");
      (async () => {
        for (const [fileId, file] of Object.entries(files)) {
          const ver = savedFilesRef.current.get(fileId);
          const newVar = file.created + "-" + file.version;
          if (!ver || ver != newVar) {
            await insertInlineResource({
              res: {
                id: md5(newVar).toString(),
                rid: fileId,
                namespace: currentNamespace.name,
                archor: true,
                name: fileId,
                content: file.dataURL,
                content_type: file.mimeType,
                insert_time: new Date(file.created),
              },
              archor_intervals: 3600,
              ignore_conflict: true,
            });
            savedFilesRef.current.set(fileId, newVar);
          }
        }

        await insertInlineResource({
          res: {
            id: uuid(),
            rid: instanceId ?? uuid(),
            namespace: currentNamespace.name,
            archor: false,
            name: instanceId ?? uuid(),
            content,
            content_type: CONTENT_TYPE,
            insert_time: new Date(),
          },
          archor_intervals: 3600,
        });
      })();
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
      onChange={onChange}
      viewModeEnabled={viewModeEnabled}
      zenModeEnabled={zenModeEnabled}
      gridModeEnabled={gridModeEnabled}
      theme={theme}
      validateEmbeddable={true}
      onLinkOpen={onLinkOpen}
    />
  );
}
