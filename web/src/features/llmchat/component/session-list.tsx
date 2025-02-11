import { useLLMChatStore } from "@/store/llmchat";
import LLMChatSessionListItem from "./session-list-item";

function LLMChatSessionList() {
  const { sessions, setCurrentSession } = useLLMChatStore();

  return (
    <ul className="w-full p-2 space-y-2">
      {[...sessions.values()].map((session) => (
        <LLMChatSessionListItem session={session} key={session.id} />
      ))}
    </ul>
  );
}

export default LLMChatSessionList;
