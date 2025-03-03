import SessionList from "@/features/llmchat/component/session-list";
import SessionBody from "@/features/llmchat/component/session-body";
import { useLLMChatStore } from "@/store/llmchat";
import { useNamespaceStore } from "@/store/namespace";
import { useEffect, useState } from "react";
import AddButton from "../component/session-add-button";
import { useCommonStore } from "@/store/common";

const LLMChatPage = () => {
  const { refreshAll, setCurrentSession } = useLLMChatStore();
  const { currentNamespace } = useNamespaceStore();
  const { showSidebar } = useCommonStore();
  const [newSessionCount, setNewSessionCount] = useState(0);

  useEffect(() => {
    refreshAll();
  }, [currentNamespace]);

  return (
    <div className="flex flex-row w-full h-full max-h-full overflow-hidden">
      <title>{`LLM Chat`}</title>

      <div className="flex flex-col items-between bg-secondary border-r kborder w-3/12 h-full">
        {showSidebar && (
          <div className="overflow-auto h-full">
            <SessionList />
          </div>
        )}
        <div className="w-full flex flex-row mb-2 justify-center">
          <AddButton
            className="px-2 py-1 max-w-40"
            onAdd={() => {
              setNewSessionCount((prev) => {
                return prev + 1;
              });
              setCurrentSession(undefined);
            }}
          />
        </div>
      </div>
      <div className="flex-1 h-full">
        <SessionBody newSessionFlag={newSessionCount} />
      </div>
    </div>
  );
};

export default LLMChatPage;
