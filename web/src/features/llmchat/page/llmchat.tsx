import SessionList from "@/features/llmchat/component/session-list";
import SessionBody from "@/features/llmchat/component/session-body";
import { useNamespaceStore } from "@/store/namespace";
import { useEffect, useState } from "react";
import { useCommonStore } from "@/store/common";
import { useLLMChatStore } from "@/store/llmchat/store";

const LLMChatPage = () => {
  const { refreshAll } = useLLMChatStore();
  const { currentNamespace } = useNamespaceStore();
  const { showSidebar } = useCommonStore();

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
      </div>
      <div className="flex-1 h-full">
        <SessionBody />
      </div>
    </div>
  );
};

export default LLMChatPage;
