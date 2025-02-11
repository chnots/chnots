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
  let roleName: string | undefined;
  if (record.role_id) {
    if (record.role === "system") {
      const template = templates.get(record.role_id);
      logo = template?.svg_logo;
      roleName = template?.name;
    } else if (record.role === "assistant" && record.role_id) {
      const bot = bots.get(record.role_id);
      logo = bot?.svg_logo;
      roleName = bot?.name;
    }
  }

  return (
    <RecordContent
      className={className}
      content={record.content}
      timestamp={record.insert_time}
      onRegenerate={record.role === "assistant" ? () => {} : undefined}
      role={record.role}
      roleName={roleName}
      logo={logo}
      key={record.id}
    />
  );
};
