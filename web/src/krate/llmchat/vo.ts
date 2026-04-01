import type { LLMChatSessionRecordFetchRsp } from "./dto";
import type { ContentBlock, LLMChatRecord } from "./po";

export type LLMChatRecordVO = Omit<LLMChatRecord, "content"> & {
  content: ContentBlock[];
};

export type LLMChatSessionRecordFetchRspVO = Omit<
  LLMChatSessionRecordFetchRsp,
  "records"
> & {
  records: LLMChatRecordVO[];
};
