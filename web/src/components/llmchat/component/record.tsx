import { LLMChatRecord, useLLMChatStore } from "@/store/llmchat";
import RecordContent from "./record-content";

export const Record = ({
  record,
  className,
}: {
  record: LLMChatRecord;
  className?: string;
}) => {
  const { bots, templates } = useLLMChatStore();
  let logo: string | undefined;
  if (record.role_id) {
    if (record.role === "system") {
      logo = templates.get(record.role_id)?.svg_logo;
    } else if (record.role === "assistant" && record.role_id) {
      logo = bots.get(record.role_id)?.svg_logo;
    }
  }
  return (
    <RecordContent
      className={className}
      content={record.content}
      timestamp={record.insert_time}
      onRegenerate={record.role === "assistant" ? () => {} : undefined}
      role={record.role}
      logo={logo}
      key={record.id}
    />
  );
};
