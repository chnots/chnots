import ChnotSidebar from "@/krate/chnot/component/chnot-sidebar";
import {
  ChnotEditor,
  ChnotEditorProvider,
} from "@/krate/chnot/component/chnot-editor";
import { useChnotStore } from "@/krate/chnot/store";
import { useCallback, useEffect, useRef, useState } from "react";
import { Chnot } from "@/krate/chnot/dto";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/common/component/ui/sidebar";
import { genUID, TID } from "@/lib/id_util";
import { ChnotKind } from "../po";
import { useShallow } from "zustand/react/shallow";
import Settings from "@/common/pages/settings-page";
import { useCommonStore } from "@/common/store";

/**
 * This component is only to improve performance, that is to say, when
 * editor changes, the list should not be rerendered.
 *
 * @returns Chnot Editor Container
 */
const MonoChnot = () => {
  const {
    curMetaId,
    getCurrentChnot,
    setCurrentChnotMetaId,
    appendChnot,
    kinds,
  } = useChnotStore(
    useShallow((store) => {
      return {
        curMetaId: store.curMetaId,
        getCurrentChnot: store.getCurrentChnot,
        setCurrentChnotMetaId: store.setCurrentChnotMetaId,
        appendChnot: store.overwriteChnotCache,
        kinds: store.kinds,
      };
    }),
  );

  const [comKey, setComKey] = useState<string>(genUID());
  const metaTidRef = useRef<TID>(null);
  const [editorChnot, setEditorChnot] = useState<Chnot | undefined>();

  useEffect(() => {
    if (curMetaId !== metaTidRef.current) {
      setComKey(genUID());
      const cc = getCurrentChnot();
      setEditorChnot(cc);
      metaTidRef.current = cc?.meta.otid ?? null;
    }
  }, [curMetaId, editorChnot]);

  const updateEditorChnot = useCallback(
    async (chnot: Chnot) => {
      if (curMetaId !== chnot.meta.otid) {
        setCurrentChnotMetaId(chnot.meta.otid);
      }
      appendChnot(chnot);
    },
    [curMetaId],
  );

  const viewModeRef = useRef(false);

  return (
    <ChnotEditorProvider
      key={comKey}
      props={{
        kind:
          editorChnot?.meta.kind ??
          (kinds && kinds.length == 1
            ? kinds.at(0)!
            : ChnotKind.MarkdownWithToent),
        metaTid: editorChnot?.meta.otid,
        readonly: viewModeRef.current,
        topleft: <SidebarTrigger />,
        onClickNewButton: () => {
          setComKey(genUID());
          metaTidRef.current = null;
          setCurrentChnotMetaId(undefined);
        },
        onSetReadonly: (readonly: boolean) => {
          viewModeRef.current = readonly;
        },
        onChnotChange: (chnot) => {
          updateEditorChnot(chnot);
        },
        onSetMetaTid: (tid) => {
          console.log("onSetMetaTid", tid);
          metaTidRef.current = tid;
        },
      }}
    >
      <ChnotEditor className="w-full h-full" />
    </ChnotEditorProvider>
  );
};

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns Chnot Page
 */
const ChnotPage = () => {
  return (
    <div className="bg-panel flex h-full max-h-full rounded-md overflow-hidden">
      <SidebarProvider
        style={
          {
            "--sidebar-width": "calc(var(--spacing) * 96)",
            "--header-height": "calc(var(--spacing) * 12)",
          } as React.CSSProperties
        }
      >
        <ChnotSidebar />
        <SidebarInset className="min-w-0">
          <MonoChnot />
        </SidebarInset>
      </SidebarProvider>
    </div>
  );
};

export default ChnotPage;
