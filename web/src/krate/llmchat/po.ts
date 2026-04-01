import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";

export type ContentBlockType = "thinking" | "content" | "error";

export type ContentBlock = {
  type: ContentBlockType;
  data: string;
};

export const buildContentBlocks = (
  body: string,
  thinking?: string,
  error?: string,
): ContentBlock[] => {
  const blocks: ContentBlock[] = [];
  if (thinking) {
    blocks.push({ type: "thinking", data: thinking });
  }
  if (error) {
    blocks.push({ type: "error", data: error });
  }
  blocks.push({ type: "content", data: body });
  return blocks;
};

export const getBlockContent = (
  blocks: ContentBlock[],
  type: ContentBlockType,
): string => {
  const block = blocks.find((b) => b.type === type);
  return block?.data ?? "";
};

export const parseContent = (content: string): ContentBlock[] => {
  try {
    const blocks: ContentBlock[] = JSON.parse(content);
    return blocks;
  } catch (_ex) {
    return [{ type: "content", data: content }];
  }
};

export const stringifyContent = (blocks: ContentBlock[]): string => {
  return JSON.stringify(blocks);
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
