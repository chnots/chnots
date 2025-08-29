import React, { useState, useCallback, useEffect, useRef } from "react";

import "./excalidraw.scss";
import "@excalidraw/excalidraw/index.css";

import { ExcalidrawImperativeAPI } from "@excalidraw/excalidraw/types";
import {
  NonDeletedExcalidrawElement,
  Theme,
} from "@excalidraw/excalidraw/element/types";
import { Excalidraw, useHandleLibrary } from "@excalidraw/excalidraw";
import { useCallbackRefState } from "@/hooks/use-callback-ref-state";
import { ExcalidrawChnotState } from "../service";

const CONTENT_TYPE = "chnots/excalidraw-v1";

const ExcalidrawEditor = ({
  excalidrawId,
  useCustom,
  customArgs,
  readOnly: viewMode,
  onSave,
  state,
}: {
  excalidrawId: string;
  useCustom?: (api: ExcalidrawImperativeAPI | null, customArgs?: any[]) => void;
  customArgs?: any[];
  readOnly?: boolean;
  state?: ExcalidrawChnotState;
  onSave: (state: ExcalidrawChnotState, contentType: string) => void;
}) => {
  const [viewModeEnabled, setViewModeEnabled] = useState(viewMode);
  const [zenModeEnabled, setZenModeEnabled] = useState(false);
  const [gridModeEnabled, setGridModeEnabled] = useState(false);
  const [theme, setTheme] = useState<Theme>("light");
  const excalidrawSaves = useRef<ExcalidrawChnotState>(null);

  useEffect(() => {
    setViewModeEnabled(viewMode ?? false);
  }, [viewMode]);

  const [excalidrawAPI, excalidrawRefCallback] =
    useCallbackRefState<ExcalidrawImperativeAPI>();

  useEffect(() => {
    return () => {
      if (excalidrawSaves.current) {
        onSave(excalidrawSaves.current, CONTENT_TYPE);
      }
    };
  }, []);

  if (useCustom) {
    useCustom(excalidrawAPI, customArgs);
  }

  useHandleLibrary({ excalidrawAPI });

  const onLinkOpen = useCallback(
    (
      element: NonDeletedExcalidrawElement,
      event: CustomEvent<{
        nativeEvent: MouseEvent | React.PointerEvent<HTMLCanvasElement>;
      }>,
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
    [],
  );

  return (
    <Excalidraw
      excalidrawAPI={excalidrawRefCallback}
      initialData={state}
      onChange={(elements, appState, files) => {
        excalidrawSaves.current = { elements, appState, files, excalidrawId };
      }}
      viewModeEnabled={viewModeEnabled}
      zenModeEnabled={zenModeEnabled}
      gridModeEnabled={gridModeEnabled}
      theme={theme}
      validateEmbeddable={true}
      onLinkOpen={onLinkOpen}
    />
  );
};

export default ExcalidrawEditor;
