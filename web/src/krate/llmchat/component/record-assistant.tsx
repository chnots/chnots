import { LLMChatBot, LLMChatRecord, LLMChatTemplate } from "@/krate/llmchat/po";
import RecordFrame from "./record-frame";
import KSVG from "@/common/component/svg";
import { useState } from "react";
import Icon from "@/common/component/icon";
import { useLLMChatStore } from "@/krate/llmchat/store";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { useLLMChatComStore } from "./llm-chat-session";

const RecordAssistant = ({
  otid,
  role,
  role_id,
  reasoning_content,
  content,
  logo,
  timestamp,
}: {
  logo?: string;
  timestamp: string;
} & LLMChatRecord) => {
  const { bots, templates } = useLLMChatStore();
  const { onRegenrate, viweMode: viewMode } = useLLMChatComStore((store) => {
    return {
      onRegenrate: store.regenrate,
      viweMode: store.viewMode,
    };
  });

  const [bt] = useState<LLMChatBot | LLMChatTemplate | undefined>(() => {
    if (!role_id) {
      return undefined;
    }
    if (role === "assistant" || role === "response-assistant") {
      const bot = bots.get(role_id);
      return bot;
    } else if (role === "system") {
      const template = templates.get(role_id);
      return template;
    }
  });

  const svgLogo = logo ? (
    <KSVG inner={logo} />
  ) : bt?.svg_logo ? (
    <KSVG inner={bt.svg_logo} />
  ) : (
    <Icon.Bot />
  );

  const onCopy = () => {
    navigator.clipboard.writeText(content);
  };

  return (
    <RecordFrame
      name={bt?.name ?? "A Bot"}
      timestamp={timestamp}
      logo={svgLogo}
      limitHeight={role === "system" ? true : undefined}
      onRegenerate={() => {
        onRegenrate(otid);
      }}
      onCopy={onCopy}
    >
      <div className="flex flex-col">
        {reasoning_content && !viewMode && (
          <div
            className={
              "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2 p-2 border rounded-tr-2xl my-2 text-sm kc-inactive"
            }
          >
            <ReactMarkdown remarkPlugins={[remarkGfm]}>
              {reasoning_content}
            </ReactMarkdown>
          </div>
        )}
        <div
          className={
            "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2"
          }
        >
          <ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown>
        </div>
      </div>
    </RecordFrame>
  );
};

export default RecordAssistant;
