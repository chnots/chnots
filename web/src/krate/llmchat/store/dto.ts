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

export type LLMChatContainerSession = {
  session: LLMChatSession;
  records: LLMChatRecord[];

  persistedIds: Set<TID>;
};
