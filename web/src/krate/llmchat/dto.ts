import type {
  LLMChatBot,
  LLMChatRecord,
  LLMChatSession,
  LLMChatTemplate,
} from "./po";
import type { TID } from "@/lib/id_util";

export type LLMChatBotListRsp = {
  bots: LLMChatBot[];
};

export type LLMChatTemplateListRsp = {
  templates: LLMChatTemplate[];
};

export type LLMChatSessionListRsp = {
  sessions: LLMChatSession[];
};

export type LLMChatSessionRecordFetchRsp = {
  session?: LLMChatSession;
  records: LLMChatRecord[];
};

export type LLMChatSessionTruncateReq = {
  session_otid: TID;
  remove_rid_included: TID;
};

export type LLMChatBotCommitReq = {
  bot: LLMChatBot;
};

export type LLMChatBotCommitRsp = object;

export type LLMChatTemplateCommitReq = {
  template: LLMChatTemplate;
};

export type LLMChatTemplateCommitRsp = object;

export type LLMChatSessionCommitReq = {
  session: LLMChatSession;
};

export type LLMChatSessionCommitRsp = object;

export type LLMChatRecordCommitReq = {
  record: LLMChatRecord;
};

export type LLMChatRecordCommitRsp = object;

export type LLMChatBotListReq = object;

export type LLMChatTemplateListReq = object;

export type LLMChatSessionListReq = {
  session_otid?: TID;
};

export type LLMChatSessionRecordTruncateReq = {
  remove_rid_included: TID;
  session_otid: TID;
};

export type LLMChatSessionRecordTruncateRsp = {
  count: number;
};

export type LLMChatSessionRecordFetchReq = {
  session_otid: TID;
};

export type LLMChatBotArchiveReq = {
  bot_otid: TID;
};

export type LLMChatBotArchiveRsp = object;

export type LLMChatTemplateArchiveReq = {
  template_otid: TID;
};

export type LLMChatTemplateArchiveRsp = object;

export type LLMChatSessionArchiveReq = {
  session_otid: TID;
};

export type LLMChatSessionArchiveRsp = object;

export type LLMChatUpdateSessionReq = {
  title?: string;
  delete?: boolean;
  session_otid: TID;
};
export type LLMChatUpdateSessionRsp = object;
