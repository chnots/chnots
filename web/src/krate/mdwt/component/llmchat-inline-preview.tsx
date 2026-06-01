import { Bot, Maximize2Icon } from "lucide-react";
import { useCallback } from "react";
import { Button } from "@/common/component/ui/button";
import { ChnotKind } from "@/krate/chnot/po";
import { getBlockContent } from "@/krate/llmchat/po";
import type { RecordContent } from "@/krate/llmchat/po";
import { useLLMChatStore } from "@/krate/llmchat/store";
import type { LLMChatSessionRecordFetchRspVO } from "@/krate/llmchat/vo";
import type { TID } from "@/lib/id_util";

import { useMdwtThreadStore } from "./mdwt-thread-store";

export type LLMChatPreviewMessage = {
  role: "user" | "assistant" | "system";
  preview: string;
};

export type LLMChatPreviewData = {
  botName?: string;
  templateName?: string;
  messageCount: number;
  lastMessages: LLMChatPreviewMessage[];
};

function truncateContent(content: RecordContent, maxLen: number): string {
  const text = getBlockContent(content, "content");
  const normalized = text.replace(/\s+/g, " ").trim();
  if (normalized.length <= maxLen) return normalized;
  return `${normalized.slice(0, maxLen)}...`;
}

export function buildLLMChatPreviewData(
  rsp: LLMChatSessionRecordFetchRspVO,
): LLMChatPreviewData {
  const records = rsp.records ?? [];

  const sorted = [...records].sort((a, b) => (a.otid > b.otid ? 1 : -1));

  // Resolve names from the zustand store (may not be hydrated yet)
  const { bots, templates } = useLLMChatStore.getState();

  const templateName = rsp.session?.template_otid
    ? templates.get(rsp.session.template_otid)?.name
    : undefined;

  const lastAssistant = [...sorted]
    .reverse()
    .find((r) => r.role === "assistant");
  const botName = lastAssistant?.role_id
    ? bots.get(lastAssistant.role_id)?.name
    : undefined;

  const lastMessages: LLMChatPreviewMessage[] = sorted.slice(-3).map((r) => ({
    role: r.role as "user" | "assistant" | "system",
    preview: truncateContent(r.content, 80),
  }));

  return {
    botName,
    templateName,
    messageCount: records.length,
    lastMessages,
  };
}

const ROLE_LABEL: Record<string, string> = {
  user: "You",
  assistant: "AI",
  system: "Sys",
};

const ROLE_COLOR: Record<string, string> = {
  user: "text-blue-500",
  assistant: "text-green-600",
  system: "text-muted-foreground",
};

interface LLMChatInlineWidgetProps {
  otid: TID;
  kindData: LLMChatPreviewData;
  onItemClick: (otid: TID, kind: ChnotKind) => void;
}

const LLMChatInlineWidget = ({
  otid,
  kindData,
  onItemClick,
}: LLMChatInlineWidgetProps) => {
  const selectedItem = useMdwtThreadStore((s) => s.selectedItem);
  const isSelected = selectedItem?.otid === otid;

  const handleExpand = useCallback(() => {
    onItemClick(otid, ChnotKind.LLMChat);
  }, [otid, onItemClick]);

  const displayName =
    kindData.botName ?? kindData.templateName ?? "LLM Chat";
  const msgLabel =
    kindData.messageCount === 1 ? "1 msg" : `${kindData.messageCount} msgs`;

  return (
    <div className="my-2 mx-1 rounded border bg-muted/30 overflow-hidden border-l-2 border-l-blue-400">
      {/* Header bar — always visible */}
      <div className="flex items-center h-8 px-2 gap-2 bg-muted/50 border-b">
        <Bot className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
        <span className="text-xs font-medium truncate">
          {displayName}
        </span>
        <span className="text-xs text-muted-foreground">({msgLabel})</span>
        <div className="flex-1" />
        <Button
          variant={isSelected ? "secondary" : "ghost"}
          size="icon"
          className="h-6 w-6"
          onClick={handleExpand}
        >
          <Maximize2Icon className="h-3.5 w-3.5" />
        </Button>
      </div>

      {/* Body — hidden when panel is open */}
      {!isSelected && (
        <div className="p-2 space-y-1 max-h-[200px] overflow-y-auto">
          {kindData.lastMessages.length === 0 ? (
            <p className="text-xs text-muted-foreground text-center py-2">
              No messages yet
            </p>
          ) : (
            kindData.lastMessages.map((msg, i) => (
              <div
                key={`${msg.role}-${i}`}
                className="flex items-start gap-1.5 text-xs"
              >
                <span
                  className={`shrink-0 font-medium w-8 text-right ${ROLE_COLOR[msg.role] ?? "text-muted-foreground"}`}
                >
                  {ROLE_LABEL[msg.role] ?? msg.role}:
                </span>
                <span className="text-muted-foreground line-clamp-1 break-all">
                  {msg.preview}
                </span>
              </div>
            ))
          )}
        </div>
      )}
    </div>
  );
};

export default LLMChatInlineWidget;
