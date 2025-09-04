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
  return await request.get(`api/v1/llmchat/list-bots`);
};

export const llmchatTemplateList =
  async (): Promise<LLMChatListTemplateRsp> => {
    return await request.get(`api/v1/llmchat/list-templates`);
  };

export const llmchatSessionList = async (): Promise<LLMChatListSessionRsp> => {
  return await request.get(`api/v1/llmchat/list-sessions`);
};

export const llmchatTemplateAdd = async (template: LLMChatTemplate) => {
  return await request.putJson(`api/v1/llmchat/overwrite-template`, {
    template,
  });
};

export const llmchatTemplateDelete = async (
  template: LLMChatDeleteTemplateReq,
) => {
  return await request.postJson(`api/v1/llmchat/delete-template`, template);
};

export const llmchatBotAdd = async (bot: LLMChatBot) => {
  return await request.putJson(`api/v1/llmchat/overwrite-bot`, {
    bot,
  });
};

export const llmchatSessionRecords = async (
  session_otid: TID,
): Promise<LLMChatSessionDetailRsp> => {
  return await request.get(`api/v1/llmchat/get-session-and-records`, {
    session_otid,
  });
};

export const llmchatSessionOverwrite = async (session: LLMChatSession) => {
  await request.putJson(`api/v1/llmchat/session-overwrition`, {
    session: session,
  });
};

export const llmchatSessionUpdate = async (req: LLMChatSessionUpdateReq) => {
  await request.postJson(`api/v1/llmchat/session-updation`, {
    ...req,
  });
};

export const llmchatSessionTruncate = async (
  req: LLMChatSessionTruncateReq,
) => {
  await request.postJson(`api/v1/llmchat/truncate-session`, {
    ...req,
  });
};

export const llmchatRecordInsert = async (record: LLMChatRecord) => {
  await request.putJson(`api/v1/llmchat/record-overwrition`, {
    record: record,
  });
};
