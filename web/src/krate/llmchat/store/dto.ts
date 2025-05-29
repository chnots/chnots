import {
  LLMChatBot,
  LLMChatTemplate,
  LLMChatSession,
  LLMChatRecord,
} from "./db";

export interface LLMChatListBotRsp {
  bots: LLMChatBot[];
}

export interface LLMChatListTemplateRsp {
  templates: LLMChatTemplate[];
}

export interface LLMChatListSessionRsp {
  sessions: LLMChatSession[];
}

export interface LLMChatSessionDetailRsp {
  session?: LLMChatSession;
  records: LLMChatRecord[];
}

export interface LLMChatSessionUpdateReq {
  session_id: string;
  delete?: boolean;
  title?: string;
}

export interface LLMChatSessionTruncateReq {
  session_id: string;
  remove_rid_included: string;
}

export interface LLMChatContainerSession {
  session: LLMChatSession;
  records: LLMChatRecord[];

  persistedIds: Set<string>;
}
