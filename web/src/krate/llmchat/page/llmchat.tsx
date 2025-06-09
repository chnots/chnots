import SessionList from "@/krate/llmchat/component/session-list";
import SessionContainer from "@/krate/llmchat/component/session-container";
import { useKSpaceStore } from "@/krate/kspace/store/store";
import { useEffect, useState } from "react";
import { useCommonStore } from "@/common/store/common";
import { useLLMChatStore } from "@/krate/llmchat/store/store";
import { v4 as uuid } from "uuid";
import { genId } from "@/lib/id_util";

const LLMChatPage = () => {
  const { refreshAll, currentSessionId, setCurrentSessionId } =
    useLLMChatStore();
  const { currentKSpace } = useKSpaceStore();
  const { showSidebar } = useCommonStore();

  const [sessionIdOrUUID, setSessionIdOrUUID] = useState<string>(uuid());
  const [containerId, setContainerId] = useState<string | undefined>(uuid());

  useEffect(() => {
    refreshAll();
  }, [currentKSpace]);

  useEffect(() => {
    setSessionIdOrUUID(currentSessionId ?? uuid());
  }, [currentSessionId]);

  useEffect(() => {
    if (sessionIdOrUUID != currentSessionId) {
      setContainerId(uuid());
    }
  }, [currentSessionId, sessionIdOrUUID, setContainerId]);

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
          kspace={currentKSpace.name}
          key={containerId}
          chnotMetaId={sessionIdOrUUID}
          onNewButton={() => {
            setCurrentSessionId(undefined);
            setSessionIdOrUUID(genId());
          }}
          afterInit={(session) => {
            setCurrentSessionId(session.id);
            setSessionIdOrUUID(session.id);
          }}
        />
      </div>
    </div>
  );
};

export default LLMChatPage;
