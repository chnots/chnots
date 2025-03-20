import React, { ForwardedRef } from "react";
import RelativeTime from "@/common/component/relative-time";
import KSVG from "@/common/component/svg";
import Icon from "@/common/component/icon";
import KListItem from "@/common/component/klistitem";
import { LLMChatSession } from "@/store/llmchat/db";
import { useLLMChatStore } from "@/store/llmchat/store";
import { llmchatSessionUpdate } from "@/store/llmchat/service";

const LLMChatSessionListItem = React.forwardRef(
  (
    { session }: { session: LLMChatSession },
    ref: ForwardedRef<HTMLLIElement>
  ) => {
    const { currentSession, setCurrentSession, templates, deleteCacheSession } =
      useLLMChatStore();
    const logo = templates.get(session.template_id)?.svg_logo;

    const handleDelete = async () => {
      await llmchatSessionUpdate({
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
        className="relative flex-col space-y-1"
      >
        <div className="flex flex-row justify-between">
          <div className="flex flex-row">
            {logo ? <KSVG className="!w-4 !h-4 mr-2" inner={logo} /> : <Icon.MessageCircle className="h-4" />}
            <RelativeTime date={session.insert_time} />
          </div>
          <div className="opacity-0 hover:opacity-100">
            <button onClick={handleDelete}>
              <Icon.X className="h-4" />
            </button>
          </div>
        </div>
        <div className="text-xs line-clamp-2 break-all">{session.title}</div>
      </KListItem>
    );
  }
);

LLMChatSessionListItem.displayName = "LLMChatSessionListItem";
export default LLMChatSessionListItem;
