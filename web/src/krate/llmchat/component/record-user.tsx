import { LLMChatRecord } from "@/krate/llmchat/po";
import RecordFrame from "./record-frame";
import { useState } from "react";
import { Textarea } from "@/common/component/ui/textarea";
import { useLLMChatComStore } from "./llm-chat-session";

const RecordUser = ({ record }: { record: LLMChatRecord }) => {
  const { content: initialContent, otid } = record;

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
      onEdit={() => {
        setEditing((prev) => !prev);
      }}
      onOk={
        editing
          ? () => {
              setEditing(false);
              updateRecord({
                ...record,
                content: content,
              });
            }
          : undefined
      }
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
};

export default RecordUser;
