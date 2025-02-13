import { LLMChatRecord, useLLMChatStore } from "@/store/llmchat";
import RecordContent from "./record-content";
import { useState } from "react";

export const Record = ({
  record,
  className,
  refreshTrigger,
}: {
  record: LLMChatRecord;
  className?: string;
  refreshTrigger?: () => void;
}) => {
  const { bots, templates, truncateSession } = useLLMChatStore();
  const [limitHeight, setLimitHeight] = useState(
    record.role === "system" ? true : undefined
  );
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
      onRegenerate={
        record.role === "assistant"
          ? async () => {
              await truncateSession({
                session_id: record.session_id,
                remove_rid_included: record.id,
              });
              if (refreshTrigger) refreshTrigger();
            }
          : undefined
      }
      role={record.role}
      roleName={roleName}
      logo={logo}
      key={record.id}
      limitedHeight={limitHeight}
      setLimitedHeight={() => {
        setLimitHeight((prev) => !prev);
      }}
    />
  );
};
