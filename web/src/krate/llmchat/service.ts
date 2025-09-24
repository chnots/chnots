import request from "@/lib/request";
import {
  LLMChatTemplateArchiveReq,
  LLMChatBotListRsp,
  LLMChatSessionListRsp,
  LLMChatTemplateListRsp,
  LLMChatSessionRecordFetchRsp,
  LLMChatSessionRecordTruncateReq,
} from "./dto";
import {
  LLMChatBot,
  LLMChatRecord,
  LLMChatSession,
  LLMChatTemplate,
} from "./po";
import { TID } from "@/lib/id_util";

export const llmchatBotList = async (): Promise<LLMChatBotListRsp> => {
  return await request.postJson("api/v1/llmchat-bot-list");
};

export const llmchatTemplateList =
  async (): Promise<LLMChatTemplateListRsp> => {
    return await request.postJson("api/v1/llmchat-template-list");
  };

export const llmchatSessionList = async (): Promise<LLMChatSessionListRsp> => {
  return await request.postJson("api/v1/llmchat-session-list");
};

export const llmchatTemplateCommit = async (template: LLMChatTemplate) => {
  return await request.postJson("api/v1/llmchat-template-commit", {
    template,
  });
};

export const llmchatTemplateArchive = async (
  template: LLMChatTemplateArchiveReq,
) => {
  return await request.postJson("api/v1/llmchat-template-archive", template);
};

export const llmchatBotCommit = async (bot: LLMChatBot) => {
  return await request.postJson("api/v1/llmchat-bot-commit", {
    bot,
  });
};

export const llmchatSessionRecordFetch = async (
  session_otid: TID,
): Promise<LLMChatSessionRecordFetchRsp> => {
  return await request.postJson("api/v1/llmchat-session-record-fetch", {
    session_otid,
  });
};

export const llmchatSessionCommit = async (session: LLMChatSession) => {
  await request.postJson("api/v1/llmchat-session-commit", {
    session: session,
  });
};

export const llmchatSessionRecordTruncate = async (
  req: LLMChatSessionRecordTruncateReq,
) => {
  await request.postJson("api/v1/llmchat-session-record-truncate", {
    ...req,
  });
};

export const llmchatRecordCommit = async (record: LLMChatRecord) => {
  await request.postJson("api/v1/llmchat-record-commit", {
    record: record,
  });
};
