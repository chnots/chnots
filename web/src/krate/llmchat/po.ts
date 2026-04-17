import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";

export type ContentBlockType =
  | "thinking"
  | "content"
  | "error"
  | "image"
  | "file";

export type ContentBlock = {
  type: ContentBlockType;
  data: string;
  mediaType?: string;
  filename?: string;
};

export type RecordUsage = {
  inputTokens?: number;
  outputTokens?: number;
};

export type RecordContent = {
  usage: RecordUsage;
  parts: ContentBlock[];
};

export const buildRecordContent = (
  body: string,
  thinking?: string,
  error?: string,
): RecordContent => {
  const blocks: ContentBlock[] = [];
  if (thinking) {
    blocks.push({ type: "thinking", data: thinking });
  }
  if (error) {
    blocks.push({ type: "error", data: error });
  }
  blocks.push({ type: "content", data: body });
  return { usage: {}, parts: blocks };
};

export const buildRecordContentFromParts = (
  parts: ContentBlock[],
  usage?: RecordUsage,
): RecordContent => {
  return { usage: usage ?? {}, parts };
};

export const getBlockContent = (
  content: RecordContent,
  type: ContentBlockType,
): string => {
  const block = content.parts.find((b) => b.type === type);
  return block?.data ?? "";
};

export const parseContent = (content: string): RecordContent => {
  try {
    const parsed = JSON.parse(content);
    if (Array.isArray(parsed)) {
      return { usage: {}, parts: parsed as ContentBlock[] };
    }
    return parsed as RecordContent;
  } catch (_ex) {
    return { usage: {}, parts: [{ type: "content", data: content }] };
  }
};

export const stringifyContent = (content: RecordContent): string => {
  return JSON.stringify(content);
};

// LLMChatBot structure
export type LLMChatBot = {
  otid: TID;
  name: Varchar<500>;
  body: DbText;
  svg_logo?: DbText;
  update_time?: Date;
  tid: TID;
};

// Legacy format (existing) - the body of LLMChatBot body.
export type LLMChatBotBodyOpenAIV1 = {
  url: string;
  token: string;
  model_name: string;
};

// New unified format using AI SDK
export type LLMProvider =
  | "openai"
  | "anthropic"
  | "google"
  | "openai-compatible";

export type LLMChatBotBodyAI = {
  provider: LLMProvider;
  api_key?: string;
  base_url: string;
  model_name: string;
};

export type LLMChatBotBody = LLMChatBotBodyOpenAIV1 | LLMChatBotBodyAI;

// LLMChatTemplate structure
export type LLMChatTemplate = {
  otid: TID;
  name: Varchar<200>;
  prompt: DbText;
  svg_logo?: DbText;
  update_time?: Date;
  tid: TID;
};

// LLMChatSession structure
export type LLMChatSession = {
  otid: TID;
  template_otid: TID;
  update_time?: Date;
  tid: TID;
};

// LLMChatRecord structure
export type LLMChatRecord = {
  otid: TID;
  session_otid: TID;
  pre_record_otid?: TID;
  content: DbText;
  role: Varchar<40>;
  role_id?: TID;
  tid: TID;
};
