import { useLLMChatStore } from "@/store/llmchat";
import { LLMChatSessionListItem } from "./session-list-item.component";

function LLMChatSessionList() {
  const { sessions } = useLLMChatStore();

  return (
    <ul className="w-full p-2 space-y-2">
      {[...sessions.values()].map((session) => (
        <LLMChatSessionListItem session={session} key={session.id} />
      ))}
    </ul>
  );
}

export default LLMChatSessionList;
