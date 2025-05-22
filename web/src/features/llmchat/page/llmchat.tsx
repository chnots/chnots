import SessionList from "@/features/llmchat/component/session-list";
import SessionContainer from "@/features/llmchat/component/session-container";
import { useWorkspaceStore } from "@/store/workspace";
import { useEffect, useState } from "react";
import { useCommonStore } from "@/store/common";
import { useLLMChatStore } from "@/store/llmchat/store";
import { v4 as uuid } from "uuid";
import { genId } from "@/utils/id_util";

const LLMChatPage = () => {
  const { refreshAll, currentSessionId, setCurrentSessionId } =
    useLLMChatStore();
  const { currentWorkspace } = useWorkspaceStore();
  const { showSidebar } = useCommonStore();

  const [sessionIdOrUUID, setSessionIdOrUUID] = useState<string>(uuid());
  const [containerId, setContainerId] = useState<string | undefined>(uuid());

  useEffect(() => {
    refreshAll();
  }, [currentWorkspace]);

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
          key={containerId}
          sessionIdOrUUID={sessionIdOrUUID}
          onNewButton={() => {
            setCurrentSessionId(undefined);
            setSessionIdOrUUID(genId());
          }}
          afterInit={(id) => {
            setCurrentSessionId(id);
            setSessionIdOrUUID(id);
          }}
        />
      </div>
    </div>
  );
};

export default LLMChatPage;
