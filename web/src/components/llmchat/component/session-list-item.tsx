import { LLMChatSession, useLLMChatStore } from "@/store/llmchat";
import clsx from "clsx";
import React, { ForwardedRef } from "react";
import RelativeTime from "../../relative-time";
import KSVG from "@/components/svg";
import Icon from "@/components/icon";

export const LLMChatSessionListItem = React.forwardRef(
  (props: { session: LLMChatSession }, ref: ForwardedRef<HTMLLIElement>) => {
    const {
      currentSession,
      setCurrentSession,
      templates,
      updateSession,
      deleteCacheSession,
    } = useLLMChatStore();
    const session = props.session;
    const logo = templates.get(session.template_id)?.svg_logo;

    const handleDelete = async () => {
      await updateSession({
        session_id: session.id,
        delete: true,
      });
      deleteCacheSession(session.id);
    };

    return (
      <li
        className={clsx(
          "flex flex-row list-none rounded-2xl p-3 pl-6 gap-1 relative select-none border w-full",
          "group hover:cursor-pointer text-xs space-x-2",
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
        <div>{logo ? <KSVG inner={logo} /> : <Icon.MessageCircle />}</div>
        <div className="w-full">
          <div className="flex flex-row justify-between">
            <RelativeTime date={session.insert_time} />
            <div className="opacity-0 hover:opacity-100">
              <button onClick={handleDelete}>
                <Icon.X className="h-4" />
              </button>
            </div>
          </div>
          <div className="text-xs line-clamp-2 break-all">{session.title}</div>
        </div>
      </li>
    );
  }
);
