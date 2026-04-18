import { Edit3, Save } from "lucide-react";
import { memo, useState } from "react";
import { Textarea } from "@/common/component/ui/textarea";
import { getBlockContent } from "@/krate/llmchat/po";
import type { LLMChatRecordVO } from "../vo";
import RecordFrame, { RecordButton } from "./record-frame";
import { useLLMChatComStore } from "./session";

const RecordUser = memo(function RecordUser({
  record,
  viewMode,
}: {
  record: LLMChatRecordVO;
  viewMode: boolean;
}) {
  const { content: blocks, otid } = record;
  const initialContent = getBlockContent(blocks, "content");

  const [content, setContent] = useState(initialContent);
  const [editing, setEditing] = useState<boolean>(false);
  const { updateRecord } = useLLMChatComStore((store) => {
    return {
      updateRecord: store.updateRecord,
    };
  });

  return (
    <RecordFrame
      timestamp={new Date(otid / 1e3).toISOString()}
      justifyEnd={true}
      buttons={
        <>
          <RecordButton
            onClick={() => {
              setEditing((prev) => !prev);
            }}
          >
            <Edit3 className="h-4 w-4 text-gray-700" />
          </RecordButton>
          {editing && (
            <RecordButton
              onClick={() => {
                setEditing(false);
                const newBlocks = blocks.map((b) =>
                  b.type === "content" ? { ...b, data: content } : b,
                );
                updateRecord({
                  ...record,
                  content: newBlocks,
                });
              }}
            >
              <Save className="h-4 w-4 text-gray-700" />
            </RecordButton>
          )}
        </>
      }
      viewMode={viewMode}
    >
      <div className="border border-cborder rounded-l-2xl rounded-br-2xl p-4 text-sm whitespace-pre-wrap kc-accent">
        {!editing ? (
          <div>{content}</div>
        ) : (
          <Textarea
            onChange={(changed) => {
              setContent(changed.target.value);
            }}
            defaultValue={content}
          />
        )}
      </div>
    </RecordFrame>
  );
});

export default RecordUser;
