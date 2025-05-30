import React, { ForwardedRef } from "react";
import RelativeTime from "@/common/component/relative-time";
import KSVG from "@/common/component/svg";
import Icon from "@/common/component/icon";
import KListItem from "@/common/component/klistitem";
import { LLMChatSession } from "@/krate/llmchat/store/db";
import { useLLMChatStore } from "@/krate/llmchat/store/store";
import { llmchatSessionUpdate } from "@/krate/llmchat/store/service";
import * as Separator from "@radix-ui/react-separator";
import { Button as KButton } from "@/common/component/ui/button";

const LLMChatSessionListItem = React.forwardRef(
  (
    { session }: { session: LLMChatSession },
    ref: ForwardedRef<HTMLLIElement>
  ) => {
    const {
      currentSessionId,
      setCurrentSessionId,
      templates,
      deleteCacheSession,
    } = useLLMChatStore();
    const template = templates.get(session.template_id);
    const logo = template?.svg_logo;
    const tmplName = template?.name;

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
          return setCurrentSessionId(session.id);
        }}
        focused={currentSessionId === session.id}
        key={session.id}
        ref={ref}
        className="relative flex space-x-2 justify-center"
      >
        <div className="p-1">
          {logo ? (
            <KSVG className="!w-6 !h-6" inner={logo} />
          ) : (
            <Icon.MessageCircle className="h-4" />
          )}
        </div>
        <div className="w-full">
          <div className="flex justify-between">
            <span>{tmplName}</span>

            <div className="flex align-middle">
              <RelativeTime date={session.insert_time} />
              <KButton onClick={handleDelete}>
                <Icon.X className="h-4 opacity-0 group-hover:opacity-100" />
              </KButton>
            </div>
          </div>
          <div className="text-xs line-clamp-2 break-all" title={session.title}>
            {session.title}
          </div>
        </div>
      </KListItem>
    );
  }
);

LLMChatSessionListItem.displayName = "LLMChatSessionListItem";
export default LLMChatSessionListItem;
