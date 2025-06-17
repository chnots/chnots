import { OmitTID, TID } from "@/lib/id_util";

// Date type definition for compatibility
type DateTime = Date;

// LLMChatBot structure

export type LLMChatBot = {
  tid: TID;
  omit_tid?: OmitTID;
  name: string;
  body: string;
  svg_logo?: string;
  update_time?: Date;
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
  omit_tid?: OmitTID;
  name: string;
  prompt: string;
  svg_logo?: string;
  update_time?: Date;
};             

// LLMChatSession structure

export type LLMChatSession = {
  tid: TID;
  omit_tid?: OmitTID;
  template_id: TID;
  title: string;
  update_time?: Date;
};             

// LLMChatRecord structure

export type LLMChatRecord = {
  tid: TID;
  omit_tid?: OmitTID;
  session_id: TID;
  pre_record_id?: TID;
  content: string;
  reasoning_content: string;
  role: string;
  role_id?: TID;
};             
