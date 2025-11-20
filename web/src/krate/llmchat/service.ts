import type {
  LLMChatBotListRsp,
  LLMChatSessionCommitReq,
  LLMChatSessionListRsp,
  LLMChatSessionRecordFetchReq,
  LLMChatSessionRecordFetchRsp,
  LLMChatSessionRecordTruncateReq,
  LLMChatTemplateArchiveReq,
  LLMChatTemplateListRsp,
} from './dto';
import type { LLMChatBot, LLMChatRecord, LLMChatTemplate } from './po';
import request from '@/lib/request';

export const llmchatBotList = async (): Promise<LLMChatBotListRsp> => {
  return await request.postJson('api/v1/llmchat-bot-list');
};

export const llmchatTemplateList = async (): Promise<LLMChatTemplateListRsp> => {
  return await request.postJson('api/v1/llmchat-template-list');
};

export const llmchatSessionList = async (): Promise<LLMChatSessionListRsp> => {
  return await request.postJson('api/v1/llmchat-session-list');
};

export const llmchatTemplateCommit = async (template: LLMChatTemplate) => {
  return await request.postJson('api/v1/llmchat-template-commit', {
    template,
  });
};

export const llmchatTemplateArchive = async (template: LLMChatTemplateArchiveReq) => {
  return await request.postJson('api/v1/llmchat-template-archive', template);
};

export const llmchatBotCommit = async (bot: LLMChatBot) => {
  return await request.postJson('api/v1/llmchat-bot-commit', {
    bot,
  });
};

export const llmchatSessionRecordFetch = async (
  req: LLMChatSessionRecordFetchReq,
): Promise<LLMChatSessionRecordFetchRsp> => {
  return await request.postJson('api/v1/llmchat-session-record-fetch', req);
};

export const llmchatSessionCommit = async (req: LLMChatSessionCommitReq) => {
  await request.postJson('api/v1/llmchat-session-commit', req);
};

export const llmchatSessionRecordTruncate = async (req: LLMChatSessionRecordTruncateReq) => {
  await request.postJson('api/v1/llmchat-session-record-truncate', {
    ...req,
  });
};

export const llmchatRecordCommit = async (record: LLMChatRecord) => {
  await request.postJson('api/v1/llmchat-record-commit', {
    record: record,
  });
};
