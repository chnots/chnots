import { LLMChatRecord } from "@/store/llmchat";
import RecordContent from "./record-content.component";

export const Record = ({ record }: { record: LLMChatRecord }) => {
  return (
    <RecordContent
      content={record.content}
      canRegenerate={record.role === "assistant"}
      role={record.role}
      key={record.id}
    />
  );
};
