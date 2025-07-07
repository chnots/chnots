import { useLLMChatStore } from "@/krate/llmchat/store";
import LLMChatSessionListItem from "./session-list-item";

function LLMChatSessionList() {
  const { sessions } = useLLMChatStore();

  return (
    <ul className="w-full p-2 space-y-2">
      {[...sessions.values()].map((session) => (
        <LLMChatSessionListItem session={session} key={session.otid} />
      ))}
    </ul>
  );
}

export default LLMChatSessionList;
