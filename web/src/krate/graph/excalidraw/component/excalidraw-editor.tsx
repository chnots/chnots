import type React from "react";
import { memo, useCallback, useEffect, useRef, useState } from "react";

import "./excalidraw.scss";
import "@excalidraw/excalidraw/index.css";

import { Excalidraw, useHandleLibrary } from "@excalidraw/excalidraw";
import type {
  NonDeletedExcalidrawElement,
  Theme,
} from "@excalidraw/excalidraw/element/types";
import type { ExcalidrawImperativeAPI } from "@excalidraw/excalidraw/types";
import { useCallbackRefState } from "@/hooks/use-callback-ref-state";
import { genUID, type TID } from "@/lib/id_util";
import { type ExcalidrawChnotState, fetchExcalidraw } from "../service";
import useDebounce from "@/hooks/use-debounce";

const ExcalidrawEditor = ({
  otid,
  readOnly: viewMode,
  onSave,
}: {
  otid: TID;
  readOnly?: boolean;
  state?: ExcalidrawChnotState;
  onSave: (state: ExcalidrawChnotState) => Promise<void>;
}) => {
  console.log("render ExcalidrawEditor");
  const [viewModeEnabled, setViewModeEnabled] = useState(viewMode);
  const [zenModeEnabled, _setZenModeEnabled] = useState(false);
  const [gridModeEnabled, _setGridModeEnabled] = useState(false);
  const [theme, _setTheme] = useState<Theme>("light");
  const toSaveExcalidrawStateRef = useRef<ExcalidrawChnotState>(null);

  useEffect(() => {
    setViewModeEnabled(viewMode ?? false);
  }, [viewMode]);

  const [excalidrawAPI, excalidrawRefCallback] =
    useCallbackRefState<ExcalidrawImperativeAPI>();

  useEffect(() => {
    return () => {};
  }, [onSave]);

  const debounceSave = useDebounce(
    async () => {
      if (toSaveExcalidrawStateRef.current) {
        await onSave(toSaveExcalidrawStateRef.current);
      }
    },
    {
      duration: 2000,
      executeOnUnmount: true,
    },
  );

  useHandleLibrary({ excalidrawAPI });

  const onLinkOpen = useCallback(
    (
      element: NonDeletedExcalidrawElement,
      event: CustomEvent<{
        nativeEvent: MouseEvent | React.PointerEvent<HTMLCanvasElement>;
      }>,
    ) => {
      // biome-ignore lint/style/noNonNullAssertion: safe
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
      initialData={async () => {
        const rsp = await fetchExcalidraw(otid);
        if (rsp) {
          return rsp;
        } else {
          return null;
        }
      }}
      onChange={(elements, appState, files) => {
        toSaveExcalidrawStateRef.current = {
          otid,
          elements: [...elements],
          appState: { ...appState },
          files: { ...files },
        };
        debounceSave();
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

const ExcalidrawEditorMemo = memo(ExcalidrawEditor);
export default ExcalidrawEditorMemo;
