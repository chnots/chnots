import { OmitTID, TID } from "@/lib/id_util";
import { DbText, Varchar } from "@/lib/types";

// LLMChatBot structure

export type LLMChatBot = {
  tid: TID;
  omit_tid?: OmitTID;
  name: Varchar<500>;
  body: DbText;
  svg_logo?: DbText;
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
  name: Varchar<200>;
  prompt: DbText;
  svg_logo?: DbText;
  update_time?: Date;
};

// LLMChatSession structure

export type LLMChatSession = {
  tid: TID;
  omit_tid?: OmitTID;
  template_tid: TID;
  title: Varchar<200>;
  update_time?: Date;
};

// LLMChatRecord structure

export type LLMChatRecord = {
  tid: TID;
  omit_tid?: OmitTID;
  session_tid: TID;
  pre_record_tid?: TID;
  content: DbText;
  reasoning_content: DbText;
  role: Varchar<40>;
  role_id?: TID;
};
