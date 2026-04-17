import type { LLMChatSessionRecordFetchRsp } from "./dto";
import type { LLMChatRecord, RecordContent } from "./po";

export type LLMChatRecordVO = Omit<LLMChatRecord, "content"> & {
  content: RecordContent;
};

export type LLMChatSessionRecordFetchRspVO = Omit<
  LLMChatSessionRecordFetchRsp,
  "records"
> & {
  records: LLMChatRecordVO[];
};
