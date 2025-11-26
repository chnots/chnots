import type { LLMChatSessionRecordFetchRsp } from "./dto";
import type { LLMChatRecord } from "./po";

export type LLMChatRecordVO = Omit<LLMChatRecord, "content"> & {
  body: string;
  thinking: string;
};

export type LLMChatSessionRecordFetchRspVO = Omit<
  LLMChatSessionRecordFetchRsp,
  "records"
> & {
  records: LLMChatRecordVO[];
};
