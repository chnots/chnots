import { useLLMChatStore } from "@/store/llmchat";
import clsx from "clsx";
import { LLMChatSessionListItem } from "./session-list-item.component";

export interface LLMChatSessionListProps {}

function LLMChatSessionList(props: LLMChatSessionListProps) {
  const { listSessions, currentSession } = useLLMChatStore();

  const handleClick = (_: React.MouseEvent) => {};

  return (
    <ul className="w-full p-2 space-y-2">
      {listSessions().map((session) => (
        <LLMChatSessionListItem session={session} key={session.id} />
      ))}
    </ul>
  );
}

export default LLMChatSessionList;
