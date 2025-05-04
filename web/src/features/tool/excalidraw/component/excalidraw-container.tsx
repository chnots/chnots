import React, {
  useState,
  useRef,
  useCallback,
  Children,
  cloneElement,
  useEffect,
} from "react";
import { v4 as uuid } from "uuid";

import * as TExcalidraw from "@excalidraw/excalidraw";
import "./excalidraw.scss";
import {
  AppState,
  ExcalidrawImperativeAPI,
  ExcalidrawInitialDataState,
} from "@excalidraw/excalidraw/types";
import {
  NonDeletedExcalidrawElement,
  Theme,
} from "@excalidraw/excalidraw/element/types";
import {
  convertToExcalidrawElements,
  MIME_TYPES,
} from "@excalidraw/excalidraw";
import useDebounce from "@/hooks/use-debounce";
import {
  insertInlineResource,
  queryInlineResource,
} from "@/store/resource/service";
import { useNamespaceStore } from "@/store/namespace";
import { useSearchParams } from "react-router-dom";
import { resolvablePromise, ResolvablePromise } from "@/utils/resolve-promise";

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

  const { useHandleLibrary, Excalidraw } = TExcalidraw;
  const appRef = useRef<any>(null);
  const [viewModeEnabled, setViewModeEnabled] = useState(false);
  const [zenModeEnabled, setZenModeEnabled] = useState(false);
  const [gridModeEnabled, setGridModeEnabled] = useState(false);
  const [renderScrollbars, setRenderScrollbars] = useState(false);
  const [theme, setTheme] = useState<Theme>("light");
  const [disableImageTool, setDisableImageTool] = useState(false);

  const [searchParams] = useSearchParams();
  const [excalidrawId, setExcalidrawId] = useState<string>();
  useEffect(() => {
    let id = instanceId ? instanceId : searchParams.get("exdId");
    id = id ?? uuid();
    setExcalidrawId(id);
  }, [setExcalidrawId, searchParams]);

  const [excalidrawAPI, setExcalidrawAPI] =
    useState<ExcalidrawImperativeAPI | null>(null);

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
        const state = JSON.parse(rsp.res[0].content);
        //@ts-ignore
        initialStatePromiseRef.current.promise.resolve({
          ...state,
          elements: convertToExcalidrawElements(state.elements),
        });
      } catch (e) {
        console.log("unable to fetch inline-resource", excalidrawId, e);
        initialStatePromiseRef.current.promise.resolve({});
      }
    };
    fetchData();
  }, [excalidrawAPI, convertToExcalidrawElements, MIME_TYPES]);

  const save = useDebounce(
    (content: object, rid: string, name: string) => {
      const json = JSON.stringify(content);
      insertInlineResource({
        res: {
          id: uuid(),
          rid: rid,
          namespace: currentNamespace.name,
          archor: false,
          name: name,
          content: json,
          content_type: CONTENT_TYPE,
          insert_time: new Date(),
        },
        archor_intervals: 3600,
      });
    },
    3000,
    true
  );

  const renderExcalidraw = (children: React.ReactNode) => {
    const Excalidraw: any = Children.toArray(children).find(
      (child) =>
        React.isValidElement(child) &&
        typeof child.type !== "string" &&
        //@ts-ignore
        child.type.displayName === "Excalidraw"
    );
    if (!Excalidraw) {
      return;
    }

    const newElement = cloneElement(Excalidraw, {
      excalidrawAPI: (api: ExcalidrawImperativeAPI) => setExcalidrawAPI(api),
      initialData: initialStatePromiseRef.current.promise,
      onChange: (elements: NonDeletedExcalidrawElement[], state: AppState) => {
        save({ elements, state }, excalidrawId, "excalidraw example");
      },
      viewModeEnabled,
      zenModeEnabled,
      renderScrollbars,
      gridModeEnabled,
      theme,
      name: "Custom name of drawing",
      UIOptions: {
        canvasActions: {
          loadScene: false,
        },
        tools: { image: !disableImageTool },
      },
      renderTopRightUI,
      onLinkOpen,
      validateEmbeddable: true,
    });
    return newElement;
  };
  const renderTopRightUI = () => {
    return <></>;
  };

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
    <div ref={appRef} className="h-full w-full min-w-full">
      {renderExcalidraw(<Excalidraw />)}
    </div>
  );
}
