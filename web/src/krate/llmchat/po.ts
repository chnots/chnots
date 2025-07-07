import { OmitTID, TID } from "@/lib/id_util";
import { DbText, Varchar } from "@/lib/types";

// LLMChatBot structure

export type LLMChatBot = {
  otid: TID;
  omit_tid?: OmitTID;
  name: Varchar<500>;
  body: DbText;
  svg_logo?: DbText;
  update_time?: Date;
  tid: TID;
};

// the body of LLMChatBot body.
export type LLMChatBotBodyOpenAIV1 = {
  url: string;
  token: string;
  model_name: string;
};

// LLMChatTemplate structure

export type LLMChatTemplate = {
  otid: TID;
  omit_tid?: OmitTID;
  name: Varchar<200>;
  prompt: DbText;
  svg_logo?: DbText;
  update_time?: Date;
  tid: TID;
};

// LLMChatSession structure

export type LLMChatSession = {
  otid: TID;
  omit_tid?: OmitTID;
  template_otid: TID;
  title: Varchar<500>;
  update_time?: Date;
  tid: TID;
};

// LLMChatRecord structure

export type LLMChatRecord = {
  otid: TID;
  omit_tid?: OmitTID;
  session_otid: TID;
  pre_record_otid?: TID;
  content: DbText;
  reasoning_content: DbText;
  role: Varchar<40>;
  role_id?: TID;
  tid: TID;
};
