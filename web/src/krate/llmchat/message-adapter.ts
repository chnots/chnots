import type { UIMessage } from "ai";
import type { TID } from "@/lib/id_util";
import type { ContentBlock } from "./po";
import type { LLMChatRecordVO } from "./vo";

function tidToStr(tid: TID): string {
  return String(tid);
}

function strToTid(s: string): TID {
  return Number(s);
}

export function recordVOToUIMessages(records: LLMChatRecordVO[]): UIMessage[] {
  return records.map((record) => ({
    id: tidToStr(record.otid),
    role: record.role as "system" | "user" | "assistant",
    parts: contentBlocksToParts(record.content),
  }));
}

export function uiMessageToRecordVO(
  msg: UIMessage,
  sessionOtid: TID,
  preRecordOtid?: TID,
  roleId?: TID,
): LLMChatRecordVO {
  return {
    otid: strToTid(msg.id),
    session_otid: sessionOtid,
    pre_record_otid: preRecordOtid,
    content: partsToContentBlocks(msg.parts),
    role: msg.role,
    role_id: roleId,
    tid: strToTid(msg.id),
  };
}

export function contentBlocksToParts(
  blocks: ContentBlock[],
): UIMessage["parts"] {
  const parts: UIMessage["parts"] = [];

  for (const block of blocks) {
    switch (block.type) {
      case "thinking":
        parts.push({
          type: "reasoning",
          text: block.data,
          state: "done",
        });
        break;
      case "content":
        parts.push({
          type: "text",
          text: block.data,
          state: "done",
        });
        break;
      case "error":
        parts.push({
          type: "data-error",
          data: { error: block.data },
        } as never);
        break;
    }
  }

  return parts;
}

export function partsToContentBlocks(
  parts: UIMessage["parts"],
): ContentBlock[] {
  const blocks: ContentBlock[] = [];

  for (const part of parts) {
    switch (part.type) {
      case "reasoning":
        if (part.text) {
          blocks.push({ type: "thinking", data: part.text });
        }
        break;
      case "text":
        if (part.text) {
          blocks.push({ type: "content", data: part.text });
        }
        break;
      default:
        if (part.type.startsWith("data-")) {
          const dataPart = part as { type: string; data: { error?: string } };
          if (dataPart.data?.error) {
            blocks.push({ type: "error", data: dataPart.data.error });
          }
        }
        break;
    }
  }

  return blocks;
}

export function extractTextFromUIMessage(msg: UIMessage): string {
  return msg.parts
    .filter((p) => p.type === "text")
    .map((p) => (p as { type: "text"; text: string }).text)
    .join("");
}
