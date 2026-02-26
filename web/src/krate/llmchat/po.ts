import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";

export const parseContent = (content: string) => {
  try {
    const jn = JSON.parse(content);
    return [jn.body, jn.thinking];
  } catch (_ex) {
    return [content, undefined];
  }
};

export const stringifyContent = (body: string, thinking?: string) => {
  return JSON.stringify({ body, thinking });
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
  title: Varchar<500>;
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
