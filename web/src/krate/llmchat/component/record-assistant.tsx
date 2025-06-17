import { LLMChatBot, LLMChatRecord, LLMChatTemplate } from "@/krate/llmchat/po";
import RecordFrame from "./record-frame";
import KSVG from "@/common/component/svg";
import { useState } from "react";
import Icon from "@/common/component/icon";
import { useLLMChatStore } from "@/krate/llmchat/store";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

const RecordAssistant = ({
  onRegenerate,
  role,
  role_id,
  reasoning_content,
  content,
  logo,
  tid,
}: {
  onRegenerate?: () => void;
  logo?: string;
} & Omit<LLMChatRecord, "omit_tid">) => {
  const { bots, templates } = useLLMChatStore();

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
      timestamp={tid}
      logo={svgLogo}
      limitHeight={role === "system" ? true : undefined}
      onRegenerate={onRegenerate}
      onCopy={onCopy}
    >
      <div className="flex flex-col">
        {reasoning_content && (
          <ReactMarkdown
            className={
              "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2 p-2 border rounded-tr-2xl my-2 text-sm kc-inactive"
            }
            remarkPlugins={[remarkGfm]}
          >
            {reasoning_content}
          </ReactMarkdown>
        )}
        <ReactMarkdown
          className={
            "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2"
          }
          remarkPlugins={[remarkGfm]}
        >
          {content}
        </ReactMarkdown>
      </div>
    </RecordFrame>
  );
};

export default RecordAssistant;
