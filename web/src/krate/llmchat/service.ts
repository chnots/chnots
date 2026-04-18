import type { TID } from "@/lib/id_util";
import request from "@/lib/request";
import type {
  LLMChatBotArchiveReq,
  LLMChatBotArchiveRsp,
  LLMChatBotListRsp,
  LLMChatSessionCommitReq,
  LLMChatSessionListRsp,
  LLMChatSessionRecordFetchReq,
  LLMChatSessionRecordFetchRsp,
  LLMChatTemplateArchiveReq,
  LLMChatTemplateListRsp,
} from "./dto";
import type { LLMChatBot, LLMChatTemplate, RecordContent } from "./po";
import { parseContent, stringifyContent } from "./po";
import type { LLMChatRecordVO, LLMChatSessionRecordFetchRspVO } from "./vo";

export const llmchatBotArchive = async (
  req: LLMChatBotArchiveReq,
): Promise<LLMChatBotArchiveRsp> => {
  return await request.postJson("api/v1/llmchat-bot-archive", req);
};

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
  req: LLMChatTemplateArchiveReq,
) => {
  return await request.postJson("api/v1/llmchat-template-archive", req);
};

export const llmchatBotCommit = async (bot: LLMChatBot) => {
  return await request.postJson("api/v1/llmchat-bot-commit", {
    bot,
  });
};

export const llmchatSessionRecordFetch = async (
  req: LLMChatSessionRecordFetchReq,
): Promise<LLMChatSessionRecordFetchRspVO> => {
  const rsp: LLMChatSessionRecordFetchRsp = await request.postJson(
    "api/v1/llmchat-session-record-fetch",
    req,
  );
  return {
    session: rsp.session,
    records: rsp.records.map((r) => {
      return { ...r, content: parseContent(r.content) };
    }),
  };
};

export const llmchatSessionRecordFetchAll = async (
  sessionOtid: TID,
): Promise<LLMChatRecordVO[]> => {
  const rsp = await llmchatSessionRecordFetch({
    session_otid: sessionOtid,
    include_hist: true,
  });
  return rsp.records;
};

export const llmchatSessionCommit = async (req: LLMChatSessionCommitReq) => {
  await request.postJson("api/v1/llmchat-session-commit", req);
};

export const llmchatRecordCommit = async (record: LLMChatRecordVO) => {
  await request.postJson("api/v1/llmchat-record-commit", {
    record: {
      ...record,
      content: stringifyContent(record.content),
    },
  });
};
