import ChnotSidebar from "@/krate/chnot/component/chnot-thread-sidebar";
import ChnotThreadEditor from "@/krate/chnot/component/thread";
import { ChnotViewType, useChnotStore } from "@/krate/chnot/store";
import { useEffect, useRef, useState } from "react";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/common/component/ui/sidebar";
import { genTID, genUID, TID } from "@/lib/id_util";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";
import { ChnotThreadMeta } from "../po";
import LoadingPage from "@/common/pages/loading-page";
import { useKSpaceStore } from "@/krate/kspace/store";

/**
 * This component is only to improve performance, that is to say, when
 * editor changes, the list should not be rerendered.
 *
 * @returns ChnotSearchRspThread Editor Container
 */
const MonoChnot = ({ onNew }: { onNew: () => void }) => {
  const { curMetaId, getCurrentThread } = useChnotStore((store) => {
    return {
      curMetaId: store.curThreadOtid,
      getCurrentThread: store.getCurrentThread,
    };
  });

  const { currentKSpace, selectKSpace } = useKSpaceStore((e) => {
    return {
      currentKSpace: e.currentKSpace,
      selectKSpace: e.selectKSpace,
    };
  });

  const [editorThread, setEditorThread] = useState<ChnotThreadMeta>();
  useEffect(() => {
    if (!curMetaId && !editorThread) {
      setEditorThread({
        otid: genTID(),
        kspace: currentKSpace,
        tid: genTID(),
      });
    } else if (curMetaId !== editorThread?.otid) {
      const t = getCurrentThread();
      if (t) {
        setEditorThread(t.meta);
      }
    }
  }, [curMetaId, editorThread]);

  return editorThread ? (
    <ChnotThreadEditor
      key={editorThread.otid}
      threadMeta={editorThread}
      globalBar={<StateBar onNew={onNew} />}
    />
  ) : (
    <LoadingPage></LoadingPage>
  );
};

const StateBar = ({ onNew }: { onNew: () => void }) => {
  return (
    <div className="w-full">
      <SidebarTrigger />
      <Button onClick={onNew}>
        <Icon.BadgePlusIcon />
      </Button>
    </div>
  );
};

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns ChnotSearchRspThread Page
 */
const ChnotPage = ({ viewType }: { viewType: ChnotViewType }) => {
  const [monoComponentKey, setMonoComponentKey] = useState(genUID());
  const { setCurrentThreadOtid } = useChnotStore((store) => {
    return {
      setCurrentThreadOtid: store.setCurrentThreadOtid,
    };
  });
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
          <MonoChnot
            onNew={() => {
              setCurrentThreadOtid(undefined);
              setMonoComponentKey(genUID());
            }}
            key={monoComponentKey}
          />
        </SidebarInset>
      </SidebarProvider>
    </div>
  );
};

export default ChnotPage;
