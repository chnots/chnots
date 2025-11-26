import request from "@/lib/request";
import type {
  LLMChatBotListRsp,
  LLMChatSessionCommitReq,
  LLMChatSessionListRsp,
  LLMChatSessionRecordFetchReq,
  LLMChatSessionRecordFetchRsp,
  LLMChatSessionRecordTruncateReq,
  LLMChatTemplateArchiveReq,
  LLMChatTemplateListRsp,
} from "./dto";
import type { LLMChatBot, LLMChatTemplate } from "./po";
import type { LLMChatRecordVO, LLMChatSessionRecordFetchRspVO } from "./vo";

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
  req: LLMChatSessionRecordFetchReq,
): Promise<LLMChatSessionRecordFetchRspVO> => {
  const rsp: LLMChatSessionRecordFetchRsp = await request.postJson(
    "api/v1/llmchat-session-record-fetch",
    req,
  );
  return {
    session: rsp.session,
    records: rsp.records.map((r) => {
      let body: string, thinking: string;
      try {
        const parsed = JSON.parse(r.content);
        body = parsed.body;
        thinking = parsed.thinking;
      } catch (_) {
        body = r.content;
        thinking = "";
      }
      return {
        ...r,
        body,
        thinking,
      };
    }),
  };
};

export const llmchatSessionCommit = async (req: LLMChatSessionCommitReq) => {
  await request.postJson("api/v1/llmchat-session-commit", req);
};

export const llmchatSessionRecordTruncate = async (
  req: LLMChatSessionRecordTruncateReq,
) => {
  await request.postJson("api/v1/llmchat-session-record-truncate", {
    ...req,
  });
};

export const llmchatRecordCommit = async (record: LLMChatRecordVO) => {
  await request.postJson("api/v1/llmchat-record-commit", {
    record: {
      ...record,
      content: JSON.stringify({
        body: record.body,
        thinking: record.thinking,
      }),
    },
  });
};
