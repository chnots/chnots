import { LLMChatSession, useLLMChatStore } from "@/store/llmchat";
import clsx from "clsx";
import React, { ForwardedRef } from "react";
import RelativeTime from "@/common/component/relative-time";
import KSVG from "@/common/component/svg";
import Icon from "@/common/component/icon";
import KListItem from "@/common/component/klistitem";

const LLMChatSessionListItem = React.forwardRef(
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
      <KListItem
        onClick={() => {
          return setCurrentSession(session);
        }}
        focused={currentSession?.id === session.id}
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
      </KListItem>
    );
  }
);

LLMChatSessionListItem.displayName = "LLMChatSessionListItem";
export default LLMChatSessionListItem;