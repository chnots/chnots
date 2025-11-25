import { useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

import RecordFrame, { RecordButton } from "./record-frame";
import { useLLMChatComStore } from "./session";
import LLMChatTemplateList from "./template-list";

import type React from "react";
import Icon from "@/common/component/icon";
import KSVG from "@/common/component/svg";
import type { LLMChatBot, LLMChatTemplate } from "@/krate/llmchat/po";
import { useLLMChatStore } from "@/krate/llmchat/store";
import type { LLMChatRecordVO } from "../vo";

const RecordCommon = ({
  body,
  thinking,
  timestamp,
  name,
  logo,
  buttons,
  limitHeight,
  viewMode,
}: {
  timestamp: string;
  name: string;
  logo: React.ReactElement;
  buttons?: React.ReactNode;
  limitHeight?: boolean;
  viewMode: boolean;
} & Omit<LLMChatRecordVO, "role">) => {
  const onCopy = () => {
    navigator.clipboard.writeText(body);
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
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{thinking}</ReactMarkdown>
          </div>
        )}
        <div
          className={
            "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2"
          }
        >
          <ReactMarkdown remarkPlugins={[remarkGfm]}>{body}</ReactMarkdown>
        </div>
      </div>
    </RecordFrame>
  );
};

export const RecordSystem = ({
  otid,
  role_id,
  body,
  thinking,
  logo,
  timestamp,
  session_otid,
  tid,
  viewMode,
}: {
  logo?: string;
  timestamp: string;
  viewMode: boolean;
} & LLMChatRecordVO) => {
  const { templates } = useLLMChatStore();
  const [setTemplate, records, setShowTemplateForm] = useLLMChatComStore((store) => {
    return [store.setTemplate, store.records, store.setShowTemplateForm];
  });

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
    <Icon.Sparkles />
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
        body={body}
        thinking={thinking}
        tid={tid}
        buttons={
          records?.length === 1 &&
          records.at(0)?.role === "system" && (
            <RecordButton
              onClick={() => {
                setShowTemplates((prev) => !prev);
              }}
            >
              <Icon.Glasses />
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
};

const RecordAssistant = ({
  otid,
  role_id,
  body,
  thinking,
  logo,
  timestamp,
  session_otid,
  tid,
  viewMode,
}: {
  logo?: string;
  timestamp: string;
  viewMode: boolean;
} & Omit<LLMChatRecordVO, "role">) => {
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
    <Icon.Bot />
  );

  return (
    <RecordCommon
      timestamp={timestamp}
      name={bt?.name ?? "Unknown Bot"}
      logo={svgLogo}
      viewMode={viewMode}
      otid={otid}
      session_otid={session_otid}
      body={body}
      thinking={thinking}
      tid={tid}
      buttons={
        <RecordButton
          onClick={(): void => {
            onRegenrate(otid);
          }}
        >
          <Icon.RotateCw />
        </RecordButton>
      }
    />
  );
};

export default RecordAssistant;
