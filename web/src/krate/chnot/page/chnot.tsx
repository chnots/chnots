import ChnotSidebar from "@/krate/chnot/component/chnot-sidebar";
import { ChnotContainer } from "@/krate/chnot/component/chnot-container";
import { useChnotStore } from "@/krate/chnot/store/store";
import { useCallback, useEffect, useRef, useState } from "react";
import { v4 as uuid } from "uuid";
import { Chnot } from "@/krate/chnot/store/dto";
import { SidebarInset, SidebarProvider } from "@/common/component/ui/sidebar";
import {
  Dialog,
  DialogContent,
  DialogTrigger,
} from "@/common/component/ui/dialog";
import { Pencil } from "lucide-react";
import { Button } from "@/common/component/ui/button";

/**
 * This component is only to improve performance, that is to say, when
 * editor changes, the list should not be rerendered.
 *
 * @returns Chnot Editor Container
 */
const MonoChnot = () => {
  const { curMetaId, getCurrentChnot, setCurrentChnotMetaId } = useChnotStore();
  // This state is used for decoupling global currentChnotMetaId and chnotEditorId.
  // From user's opinion, I want to edit when I enter this page, there should not any other steps,
  // like to click or something.
  // Then if we create new records in the db, which could make many dirty data.
  // Consider this:
  //   1. Insert into db and cache.
  //   2. Highlight then editing chnot item in the list.
  //   3. Do not disturb user's workflow
  // So to use a mid-state to decouple them.
  const [chnotEditorId, setChnotEditorId] = useState<string>(uuid());

  // Extract from ChnotMarkdownEditor.
  const [editorChnot, setEditorChnot] = useState<Chnot | undefined>(
    getCurrentChnot()
  );

  useEffect(() => {
    if (curMetaId !== editorChnot?.meta.id) {
      setChnotEditorId(curMetaId ?? uuid());
      setEditorChnot(getCurrentChnot());
    }
  }, [curMetaId, setChnotEditorId, editorChnot, getCurrentChnot]);

  const updateEditorChnot = useCallback(
    async (chnot: Chnot) => {
      setEditorChnot(chnot);
      if (curMetaId !== chnot.meta.id) {
        setCurrentChnotMetaId(chnot.meta.id);
      }
    },
    [setCurrentChnotMetaId, setEditorChnot]
  );

  const viewModeRef = useRef(false);

  return (
    <ChnotContainer
      onClickNewButton={() => {
        setChnotEditorId(uuid());
        setCurrentChnotMetaId(undefined);
      }}
      key={chnotEditorId}
      chnot={editorChnot}
      globalViewMode={viewModeRef}
      className="w-full h-full"
      onChnotChange={updateEditorChnot}
    />
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
