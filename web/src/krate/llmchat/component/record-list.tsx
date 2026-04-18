import { useMemo } from "react";
import type { LLMChatRecordVO } from "../vo";
import RecordAssistant, { RecordSystem } from "./record-assistant";
import RecordUser from "./record-user";

function RecordItem({
  record,
  viewMode,
}: {
  record: LLMChatRecordVO;
  viewMode: boolean;
}) {
  const timestamp = new Date(record.otid / 1e3).toISOString();
  switch (record.role) {
    case "user":
      return <RecordUser viewMode={viewMode} record={record} />;
    case "system":
      return (
        <RecordSystem
          viewMode={viewMode}
          timestamp={timestamp}
          {...record}
          key={record.otid}
        />
      );
    default:
      return (
        <RecordAssistant
          viewMode={viewMode}
          timestamp={timestamp}
          {...record}
          key={record.otid}
        />
      );
  }
}

export function RecordList({
  records,
  viewMode,
}: {
  records: LLMChatRecordVO[];
  viewMode: boolean;
}) {
  const sorted = useMemo(
    () => records.toSorted((a, b) => (a.otid > b.otid ? 1 : -1)),
    [records],
  );

  return (
    <>
      {sorted.map((record) => (
        <RecordItem record={record} viewMode={viewMode} key={record.otid} />
      ))}
    </>
  );
}
