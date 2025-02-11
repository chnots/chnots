import LLMChatSessionList from "@/components/llmchat/component/session-list";
import LLMChatSession from "@/components/llmchat/layout/session-container";
import { useLLMChatStore } from "@/store/llmchat";
import { useNamespaceStore } from "@/store/namespace";
import { useEffect } from "react";

const LLMChatPage = () => {
  const { refreshAll } = useLLMChatStore();
  const { currentNamespace } = useNamespaceStore();

  useEffect(() => {
    refreshAll();
  }, [currentNamespace]);

  return (
    <div className="bg-panel flex h-full max-h-full flex-1 overflow-hidden rounded-md">
      <div className="shrink-0 border-r kborder flex flex-col w-3/12 bg-secondary">
        <div className="overflow-auto h-full bg-background text-gray-900">
          <LLMChatSessionList />
        </div>
      </div>

      <div className="flex-1 h-full">{<LLMChatSession />}</div>
    </div>
  );
};

export default LLMChatPage;
