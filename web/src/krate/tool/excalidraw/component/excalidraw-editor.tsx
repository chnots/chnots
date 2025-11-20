import { useCallback, useEffect, useRef, useState } from 'react';

import type React from 'react';

import './excalidraw.scss';
import '@excalidraw/excalidraw/index.css';

import { Excalidraw, useHandleLibrary } from '@excalidraw/excalidraw';

import { type ExcalidrawChnotState, fetchExcalidraw } from '../service';

import { useCallbackRefState } from '@/hooks/use-callback-ref-state';

import type { NonDeletedExcalidrawElement, Theme } from '@excalidraw/excalidraw/element/types';
import type { ExcalidrawImperativeAPI } from '@excalidraw/excalidraw/types';
import { genUID, type TID } from '@/lib/id_util';

const CONTENT_TYPE = 'chnots/excalidraw-v1';

const ExcalidrawEditor = ({
  otid,
  readOnly: viewMode,
  onSave,
}: {
  otid: TID;
  readOnly?: boolean;
  state?: ExcalidrawChnotState;
  onSave: (state: ExcalidrawChnotState, contentType: string) => void;
}) => {
  const [viewModeEnabled, setViewModeEnabled] = useState(viewMode);
  const [zenModeEnabled, _setZenModeEnabled] = useState(false);
  const [gridModeEnabled, _setGridModeEnabled] = useState(false);
  const [theme, _setTheme] = useState<Theme>('light');
  const toSaveExcalidrawStateRef = useRef<ExcalidrawChnotState>(null);
  const toSaveExcalidrawMetaIdRef = useRef<string>(null);

  useEffect(() => {
    setViewModeEnabled(viewMode ?? false);
  }, [viewMode]);

  const [excalidrawAPI, excalidrawRefCallback] = useCallbackRefState<ExcalidrawImperativeAPI>();

  useEffect(() => {
    return () => {
      if (toSaveExcalidrawStateRef.current) {
        onSave(toSaveExcalidrawStateRef.current, CONTENT_TYPE);
      }
    };
  }, [onSave]);

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
      const isInternalLink = link.startsWith('/') || link.includes(window.location.origin);
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
        const rsp = await fetchExcalidraw({ Otid: otid });
        if (rsp) {
          toSaveExcalidrawMetaIdRef.current = rsp.metaId;
          return rsp;
        } else {
          toSaveExcalidrawMetaIdRef.current = genUID();
          return null;
        }
      }}
      onChange={(elements, appState, files) => {
        if (toSaveExcalidrawMetaIdRef.current) {
          toSaveExcalidrawStateRef.current = {
            otid,
            elements,
            appState,
            files,
            metaId: toSaveExcalidrawMetaIdRef.current,
          };
        }
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
