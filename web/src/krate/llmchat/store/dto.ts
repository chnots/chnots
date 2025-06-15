import { TID } from "@/lib/id_util";
import {
  LLMChatBot,
  LLMChatTemplate,
  LLMChatSession,
  LLMChatRecord,
} from "./po";

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
  session_id: TID;
  delete?: boolean;
  title?: string;
}

export interface LLMChatSessionTruncateReq {
  session_id: TID;
  remove_rid_included: TID;
}

export interface LLMChatContainerSession {
  session: LLMChatSession;
  records: LLMChatRecord[];

  persistedIds: Set<TID>;
}
