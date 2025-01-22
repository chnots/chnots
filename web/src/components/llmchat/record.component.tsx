import { LLMChatRecord } from "@/store/llmchat";
import clsx from "clsx";
const bgColor = (role: string) => {
  let color = "bg-green-50";
  if (role === "user") {
    color = "bg-red-50";
  } else if (role === "assistant") {
    color = "bg-blue-50";
  }
  return color;
};

export const Record = ({ record }: { record: LLMChatRecord }) => {
  return (
    <div
      className={clsx(bgColor(record.role), "p-3 border-b border-b-gray-200")}
      key={record.id}
    >
      {record.content}
    </div>
  );
};

export const ResponseRecord = ({
  previousRecordId,
}: {
  previousRecordId?: string;
}) => {
  return (
    <div
      className={clsx(bgColor(""), "p-3 border-b border-b-gray-200")}
    >
    </div>
  );
};
