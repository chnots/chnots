import ChnotSidebar from "@/krate/chnot/component/chnot-sidebar";
import ChnotEditor from "@/krate/chnot/component/chnot-thread";
import { useChnotStore } from "@/krate/chnot/store";
import { useCallback, useEffect, useRef, useState } from "react";
import { ChnotThread } from "@/krate/chnot/dto";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
  useSidebar,
} from "@/common/component/ui/sidebar";
import { genUID, TID } from "@/lib/id_util";
import { ChnotKind } from "../po";
import { useShallow } from "zustand/react/shallow";
import Settings from "@/common/pages/settings-page";
import { useCommonStore } from "@/common/store";
import ChnotBlocks from "../component/chnot-thread/thread";
import UnderConstructionPage from "@/common/pages/under-construction-page";

/**
 * This component is only to improve performance, that is to say, when
 * editor changes, the list should not be rerendered.
 *
 * @returns ChnotThread Editor Container
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

  const { isMobile } = useSidebar();
  const [comKey, setComKey] = useState<string>(genUID());
  const metaOtidRef = useRef<TID>(null);
  const [editorChnot, setEditorChnot] = useState<ChnotThread | undefined>();

  useEffect(() => {
    if (curMetaId !== metaOtidRef.current) {
      setComKey(genUID());
      const cc = getCurrentChnot();
      setEditorChnot(cc);
      metaOtidRef.current = cc?.meta.otid ?? null;
    }
  }, [curMetaId, editorChnot]);

  const updateEditorChnot = useCallback(
    async (chnot: ChnotThread) => {
      if (curMetaId !== chnot.meta.otid) {
        setCurrentChnotMetaId(chnot.meta.otid);
      }
      appendChnot(chnot);
    },
    [curMetaId],
  );

  const viewModeRef = useRef(false);

  return editorChnot?.meta ? (
    <ChnotEditor key={comKey} meta={editorChnot.meta} />
  ) : (
    <UnderConstructionPage />
  );
};

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns ChnotThread Page
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
