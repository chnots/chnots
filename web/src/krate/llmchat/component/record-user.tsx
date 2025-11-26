import { useState } from "react";
import Icon from "@/common/component/icon";
import { Textarea } from "@/common/component/ui/textarea";
import type { LLMChatRecordVO } from "../vo";
import RecordFrame, { RecordButton } from "./record-frame";
import { useLLMChatComStore } from "./session";

const RecordUser = ({
  record,
  viewMode,
}: {
  record: LLMChatRecordVO;
  viewMode: boolean;
}) => {
  const { body: initialContent, otid } = record;

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
            <Icon.Edit3 className="h-4 w-4 text-gray-700" />
          </RecordButton>
          {editing && (
            <RecordButton
              onClick={() => {
                setEditing(false);
                updateRecord({
                  ...record,
                  body: content,
                });
              }}
            >
              <Icon.Save className="h-4 w-4 text-gray-700" />
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
};

export default RecordUser;
