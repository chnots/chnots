import { code } from "@streamdown/code";
import { createMathPlugin } from "@streamdown/math";
import { mermaid } from "@streamdown/mermaid";
import type React from "react";
import { memo, useState } from "react";
import { Streamdown } from "streamdown";
import "katex/dist/katex.min.css";

const math = createMathPlugin({ singleDollarTextMath: true });

import { Bot, Glasses, RotateCw, Sparkles } from "lucide-react";
import KSVG from "@/common/component/svg";
import type { LLMChatBot, LLMChatTemplate } from "@/krate/llmchat/po";
import { getBlockContent } from "@/krate/llmchat/po";
import { contentBlocksToMarkdown } from "@/krate/llmchat/record-markdown";
import { useLLMChatStore } from "@/krate/llmchat/store";
import type { LLMChatRecordVO } from "../vo";
import RecordFrame, { RecordButton } from "./record-frame";
import { useLLMChatComStore } from "./session";
import LLMChatTemplateList from "./template-list";

type RecordCommonProps = {
  timestamp: string;
  name: string;
  logo: React.ReactElement;
  buttons?: React.ReactNode;
  limitHeight?: boolean;
  viewMode: boolean;
  isAnimating?: boolean;
} & Omit<LLMChatRecordVO, "role">;

const RecordCommon = memo(function RecordCommon({
  content,
  timestamp,
  name,
  logo,
  buttons,
  limitHeight,
  viewMode,
  isAnimating,
}: RecordCommonProps) {
  const body = getBlockContent(content, "content");
  const thinking = getBlockContent(content, "thinking");
  const onCopy = () => {
    navigator.clipboard.writeText(contentBlocksToMarkdown(content));
  };

  return (
    <RecordFrame
      name={name}
      timestamp={timestamp}
      logo={logo}
      limitHeight={limitHeight}
      onCopy={onCopy}
      buttons={buttons}
      viewMode={viewMode}
    >
      <div className="flex flex-col">
        {thinking && !viewMode && (
          <div
            className={
              "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2 p-2 border rounded-tr-2xl my-2 text-sm kc-inactive"
            }
          >
            <Streamdown
              plugins={{ code, math, mermaid }}
              isAnimating={isAnimating ?? false}
            >
              {thinking}
            </Streamdown>
          </div>
        )}
        <div
          className={
            "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2"
          }
        >
          <span className={isAnimating ? "typing-cursor" : ""}>
            <Streamdown
              plugins={{ code, math, mermaid }}
              isAnimating={isAnimating ?? false}
            >
              {body}
            </Streamdown>
          </span>
        </div>
      </div>
    </RecordFrame>
  );
});

type RecordSystemProps = {
  logo?: string;
  timestamp: string;
  viewMode: boolean;
} & LLMChatRecordVO;

export const RecordSystem = memo(function RecordSystem({
  otid,
  role_id,
  content,
  logo,
  timestamp,
  session_otid,
  tid,
  viewMode,
}: RecordSystemProps) {
  const { templates } = useLLMChatStore();
  const [setTemplate, records, setShowTemplateForm] = useLLMChatComStore(
    (store) => {
      return [store.setTemplate, store.records, store.setShowTemplateForm];
    },
  );

  const [showTemplates, setShowTemplates] = useState<boolean>();

  const [tmpl] = useState<LLMChatTemplate | undefined>(() => {
    if (!role_id) {
      return undefined;
    }
    const bot = templates.get(role_id);
    return bot;
  });

  const svgLogo = logo ? (
    <KSVG src={logo} />
  ) : tmpl?.svg_logo ? (
    <KSVG src={tmpl.svg_logo} />
  ) : (
    <Sparkles />
  );

  return (
    <>
      <RecordCommon
        timestamp={timestamp}
        name={tmpl?.name ?? "Unknown Template"}
        logo={svgLogo}
        viewMode={!!viewMode}
        otid={otid}
        session_otid={session_otid}
        limitHeight={true}
        content={content}
        tid={tid}
        buttons={
          records?.length === 1 &&
          records.at(0)?.role === "system" && (
            <RecordButton
              onClick={() => {
                setShowTemplates((prev) => !prev);
              }}
            >
              <Glasses />
            </RecordButton>
          )
        }
      />
      {showTemplates && (
        <LLMChatTemplateList
          onSelectTemplate={(template: LLMChatTemplate): void => {
            setTemplate(template);
          }}
          onNew={(): void => {
            setShowTemplateForm(true);
          }}
        />
      )}
    </>
  );
});

type RecordAssistantProps = {
  logo?: string;
  timestamp: string;
  viewMode: boolean;
  isAnimating?: boolean;
} & Omit<LLMChatRecordVO, "role">;

const RecordAssistant = memo(function RecordAssistant({
  otid,
  role_id,
  content,
  logo,
  timestamp,
  session_otid,
  tid,
  viewMode,
  isAnimating,
}: RecordAssistantProps) {
  const { bots } = useLLMChatStore();
  const { onRegenrate } = useLLMChatComStore((store) => {
    return {
      onRegenrate: store.regenrate,
    };
  });

  const [bt] = useState<LLMChatBot | undefined>(() => {
    if (!role_id) {
      return undefined;
    }
    const bot = bots.get(role_id);
    return bot;
  });

  const svgLogo = logo ? (
    <KSVG src={logo} />
  ) : bt?.svg_logo ? (
    <KSVG src={bt.svg_logo} />
  ) : (
    <Bot />
  );

  return (
    <RecordCommon
      timestamp={timestamp}
      name={bt?.name ?? "Unknown Bot"}
      logo={svgLogo}
      viewMode={viewMode}
      otid={otid}
      session_otid={session_otid}
      content={content}
      tid={tid}
      isAnimating={isAnimating}
      buttons={
        !isAnimating && (
          <RecordButton
            onClick={(): void => {
              onRegenrate(otid);
            }}
          >
            <RotateCw />
          </RecordButton>
        )
      }
    />
  );
});

export default RecordAssistant;
