import ChnotList from "@/features/chnot/component/chnot-list";
import { ChnotContainer } from "@/features/chnot/component/chnot-container";
import { useChnotStore } from "@/store/chnot/store";
import { useCommonStore } from "@/store/common";
import { useNamespaceStore } from "@/store/namespace";
import { useCallback, useEffect, useRef, useState } from "react";
import { v4 as uuid } from "uuid";
import { Chnot } from "@/store/chnot/dto";

/**
 * This component is only to improve performance, that is to say, when
 * editor changes, the list should not be rerendered.
 *
 * @returns Chnot Editor Container
 */
const MonoChnotContainer = () => {
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
    <div className="flex flex-grow justify-center items-center p-4">
      <ChnotContainer
        newButtonAction={() => {
          setChnotEditorId(uuid());
          setCurrentChnotMetaId(undefined);
        }}
        key={chnotEditorId}
        chnot={editorChnot}
        globalViewMode={viewModeRef}
        className="w-full max-w-3xl h-full"
        chnotChange={updateEditorChnot}
      />
    </div>
  );
};

/**
 * Page for chnots, which is left and right layouted.
 *
 * Current there is only one chnot editor in the page, use multi webpages.
 * @returns Chnot Page
 */
const ChnotPage = () => {
  const { refreshChnots } = useChnotStore();
  const { currentNamespace } = useNamespaceStore();
  const { showSidebar } = useCommonStore();
  useEffect(() => {
    refreshChnots();
  }, [currentNamespace]);

  return (
    <div className="bg-panel flex h-full max-h-full rounded-md">
      <title>{`Chnots`}</title>
      {showSidebar && (
        <div className="shrink-0 border-r kborder flex flex-col w-3/12 p-2">
          <ChnotList />
        </div>
      )}

      <MonoChnotContainer />
    </div>
  );
};

export default ChnotPage;
