import { LLMChatSession, useLLMChatStore } from "@/store/llmchat";
import clsx from "clsx";
import React, { ForwardedRef } from "react";
import RelativeTime from "../relative-time";

export const LLMChatSessionListItem = React.forwardRef(
  (props: { session: LLMChatSession }, ref: ForwardedRef<HTMLLIElement>) => {
    const { currentSession, setCurrentSession } = useLLMChatStore();
    const session = props.session;

    return (
      <li
        className={clsx(
          "list-none rounded-2xl p-3 pl-6 grid gap-1 relative select-none border",
          "group hover:cursor-pointer text-xs",
          currentSession?.id === session.id
            ? "bg-white text-black"
            : "text-gray-700 border-transparent hover:bg-white hover:border-gray-200"
        )}
        onClick={() => {
          setCurrentSession(session);
        }}
        key={session.id}
        ref={ref}
      >
        <RelativeTime date={session.insert_time} />
        <div className="text-xs line-clamp-2 break-all">{session.title}</div>
      </li>
    );
  }
);
