import ChnotSidebar from "@/krate/chnot/component/chnot-sidebar";
import ChnotThreadEditor from "@/krate/chnot/component/thread";
import { useChnotStore } from "@/krate/chnot/store";
import { useEffect, useRef, useState } from "react";
import { ChnotThreadListRspData } from "@/krate/chnot/dto";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/common/component/ui/sidebar";
import { genUID } from "@/lib/id_util";
import { useShallow } from "zustand/react/shallow";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";
import { ChnotThreadMeta } from "../po";

/**
 * This component is only to improve performance, that is to say, when
 * editor changes, the list should not be rerendered.
 *
 * @returns ChnotThreadListRspData Editor Container
 */
const MonoChnot = ({ onNew }: { onNew: () => void }) => {
  const { curMetaId, getCurrentThread } = useChnotStore(
    useShallow((store) => {
      return {
        curMetaId: store.curThreadOtid,
        getCurrentThread: store.getCurrentThread,
      };
    }),
  );

  const [componentKey, setComponentKey] = useState<string>(genUID());
  const threadOtidRef = useRef<ChnotThreadMeta>(null);
  const [editorThread, setEditorThread] = useState<
    ChnotThreadListRspData | undefined
  >();

  useEffect(() => {
    if (curMetaId !== threadOtidRef.current?.otid) {
      setComponentKey(genUID());
      const thread = getCurrentThread();
      setEditorThread(thread);
      threadOtidRef.current = thread?.meta ?? null;
    }
  }, [curMetaId, editorThread]);

  return (
    <ChnotThreadEditor
      key={componentKey}
      threadMeta={editorThread?.meta}
      cachedThreadMetaRef={threadOtidRef}
      globalBar={<StateBar onNew={onNew} />}
    />
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
 * @returns ChnotThreadListRspData Page
 */
const ChnotPage = () => {
  const [monoComponentKey, setMonoComponentKey] = useState(genUID());
  const { setCurrentThreadOtid } = useChnotStore(
    useShallow((store) => {
      return {
        setCurrentThreadOtid: store.setCurrentThreadOtid,
      };
    }),
  );
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
