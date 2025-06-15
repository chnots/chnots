import { TID } from "@/lib/id_util";

// Date type definition for compatibility
type DateTime = Date;

// LLMChatBot structure
export type LLMChatBot = {
  tid: TID;
  name: string;
  body: string;
  svg_logo?: string;
  omit_tid?: DateTime; // Optional field
  update_time?: DateTime; // Optional field
};

// the body of LLMChatBot body.
export type LLMChatBotBodyOpenAIV1 = {
  url: string;
  token: string;
  model_name: string;
};

// LLMChatTemplate structure
export type LLMChatTemplate = {
  tid: TID;
  name: string;
  prompt: string;
  svg_logo?: string;
  omit_tid?: DateTime; // Optional field
  update_time?: DateTime; // Optional field
};

// LLMChatSession structure
export type LLMChatSession = {
  tid: TID;
  bot_id: TID;
  template_id: TID;
  title: string;
  kspace: string;
  omit_tid?: DateTime; // Optional field
  update_time?: DateTime; // Optional field
};

// LLMChatRecord structure
export type LLMChatRecord = {
  tid: TID;
  session_id: TID;
  pre_record_id?: TID; // Optional field
  content: string;
  reasoning_content: string;
  role: "user" | "system" | "assistant" | "response-assistant";
  role_id?: TID;
};
