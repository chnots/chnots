import { TID } from "@/lib/id_util";
import {
  LLMChatBot,
  LLMChatTemplate,
  LLMChatSession,
  LLMChatRecord,
} from "./po";

export type LLMChatListBotRsp = {
  bots: LLMChatBot[];
};      

export type LLMChatListTemplateRsp = {
  templates: LLMChatTemplate[];
};      

export type LLMChatListSessionRsp = {
  sessions: LLMChatSession[];
};      

export type LLMChatSessionDetailRsp = {
  session?: LLMChatSession;
  records: LLMChatRecord[];
};      

export type LLMChatSessionUpdateReq = {
  session_id: TID;
  delete?: boolean;
  title?: string;
};

export type LLMChatSessionTruncateReq = {
  session_id: TID;
  remove_rid_included: TID;
};

export type LLMChatSessionDetail = {
  session: LLMChatSession;
  records: LLMChatRecord[];

};

export type LLMChatOverwriteBotReq = {
  bot: LLMChatBot;
};      

export type LLMChatOverwriteBotRsp = object;      

export type LLMChatOverwriteTemplateReq = {
  template: LLMChatTemplate;
};      

export type LLMChatOverwriteTemplateRsp = object;      

export type LLMChatInsertSessionReq = {
  session: LLMChatSession;
};      

export type LLMChatInsertSessionRsp = object;      

export type LLMChatInsertRecordReq = {
  record: LLMChatRecord;
};      

export type LLMChatInsertRecordRsp = object;      

export type LLMChatListBotReq = object;      

export type LLMChatListTemplateReq = object;      

export type LLMChatListSessionReq = {
  session_id?: TID;
};      

export type LLMChatUpdateSessionReq = {
  title?: string;
  delete?: boolean;
  session_id: TID;
};      

export type LLMChatUpdateSessionRsp = object;      

export type LLMChatTruncateSessionReq = {
  remove_rid_included: TID;
  session_id: TID;
};      

export type LLMChatTruncateSessionRsp = {
  count: number;
};      

export type LLMChatSessionDetialReq = {
  session_id: TID;
  with_omit?: boolean;
};      

export type LLMChatDeleteBotReq = {
  bot_id: TID;
};      

export type LLMChatDeleteBotRsp = object;      

export type LLMChatDeleteTemplateReq = {
  template_id: TID;
};      

export type LLMChatDeleteTemplateRsp = object;      

export type LLMChatDeleteSessionReq = {
  session_id: TID;
};      

export type LLMChatDeleteSessionRsp = object;      
