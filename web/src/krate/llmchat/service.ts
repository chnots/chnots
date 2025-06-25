import request from "@/lib/request";
import {
  LLMChatDeleteTemplateReq,
  LLMChatListBotRsp,
  LLMChatListSessionRsp,
  LLMChatListTemplateRsp,
  LLMChatSessionDetailRsp,
  LLMChatSessionTruncateReq,
  LLMChatSessionUpdateReq,
} from "./dto";
import {
  LLMChatBot,
  LLMChatRecord,
  LLMChatSession,
  LLMChatTemplate,
} from "./po";
import { TID } from "@/lib/id_util";

export const llmchatBotList = async (): Promise<LLMChatListBotRsp> => {
  return await request.get(`api/v1/llmchat/bots`);
};

export const llmchatTemplateList =
  async (): Promise<LLMChatListTemplateRsp> => {
    return await request.get(`api/v1/llmchat/templates`);
  };

export const llmchatSessionList = async (): Promise<LLMChatListSessionRsp> => {
  return await request.get(`api/v1/llmchat/sessions`);
};

export const llmchatTemplateAdd = async (template: LLMChatTemplate) => {
  return await request.put(`api/v1/llmchat/template`, {
    template,
  });
};

export const llmchatTemplateDelete = async (
  template: LLMChatDeleteTemplateReq,
) => {
  return await request.post(`api/v1/llmchat/template`, template);
};

export const llmchatBotAdd = async (bot: LLMChatBot) => {
  return await request.put(`api/v1/llmchat/bot`, {
    bot,
  });
};

export const llmchatSessionRecords = async (
  session_tid: TID,
): Promise<LLMChatSessionDetailRsp> => {
  return await request.get(`api/v1/llmchat/session`, {
    session_tid,
  });
};

export const llmchatSessionOverwrite = async (session: LLMChatSession) => {
  await request.put(`api/v1/llmchat/session`, {
    session: session,
  });
};

export const llmchatSessionUpdate = async (req: LLMChatSessionUpdateReq) => {
  await request.post(`api/v1/llmchat/session`, {
    ...req,
  });
};

export const llmchatSessionTruncate = async (
  req: LLMChatSessionTruncateReq,
) => {
  await request.post(`api/v1/llmchat/truncate-session`, {
    ...req,
  });
};

export const llmchatRecordInsert = async (record: LLMChatRecord) => {
  await request.put(`api/v1/llmchat/record`, {
    record: record,
  });
};
