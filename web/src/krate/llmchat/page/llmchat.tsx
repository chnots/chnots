import SessionList from "@/krate/llmchat/component/session-list";
import SessionContainer from "@/krate/llmchat/component/session-container";
import { useKSpaceStore } from "@/krate/kspace/store";
import { useEffect, useState } from "react";
import { useLLMChatStore } from "@/krate/llmchat/store";
import { v4 as uuid } from "uuid";
import { genUID, genTID, TID } from "@/lib/id_util";
import { useCommonStore } from "@/common/store";

const LLMChatPage = () => {
  const { refreshAll, currentSessionId, setCurrentSessionId } =
    useLLMChatStore();
  const { currentKSpace } = useKSpaceStore((store) => {
    return {
      currentKSpace: store.currentKSpace,
    };
  });
  const { showSidebar } = useCommonStore();

  const [sessionIdOrTID, setSessionIdOrTID] = useState<TID>(genTID());
  const [containerId, setContainerId] = useState<string | undefined>(uuid());

  useEffect(() => {
    refreshAll();
  }, [currentKSpace]);

  useEffect(() => {
    setSessionIdOrTID(currentSessionId ?? genTID());
  }, [currentSessionId]);

  useEffect(() => {
    if (sessionIdOrTID != currentSessionId) {
      setContainerId(uuid());
    }
  }, [currentSessionId, sessionIdOrTID, setContainerId]);

  return (
    <div className="flex flex-row w-full h-full max-h-full overflow-hidden">
      <title>LLM Chat</title>

      {showSidebar && (
        <div className="flex flex-col items-between border-r kc-basic-with-bdr w-3/12 h-full">
          <div className="overflow-auto h-full">
            <SessionList />
          </div>
        </div>
      )}
      <div className="flex-1 h-full">
        <SessionContainer
          key={containerId}
          kindId={sessionIdOrTID.toString()}
          onNewButton={() => {
            setCurrentSessionId(undefined);
            setSessionIdOrTID(genTID());
          }}
          onAfterSave={(session) => {
            setCurrentSessionId(session.otid);
            setSessionIdOrTID(session.otid);
          }}
        />
      </div>
    </div>
  );
};

export default LLMChatPage;
