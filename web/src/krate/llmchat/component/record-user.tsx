import { Edit3, Paperclip, Save } from "lucide-react";
import { memo, useState } from "react";
import { Textarea } from "@/common/component/ui/textarea";
import type { ContentBlock } from "@/krate/llmchat/po";
import { getBlockContent } from "@/krate/llmchat/po";
import type { LLMChatRecordVO } from "../vo";
import RecordFrame, { RecordButton } from "./record-frame";
import { useLLMChatComStore } from "./session";

const AttachedFileList = ({ parts }: { parts: ContentBlock[] }) => {
  const files = parts.filter((b) => b.type === "image" || b.type === "file");
  if (files.length === 0) return null;

  return (
    <div className="flex flex-wrap gap-2 mb-2">
      {files.map((block, i) =>
        block.type === "image" ? (
          <img
            key={i}
            src={block.data}
            alt={block.filename ?? "image"}
            className="max-h-32 max-w-48 rounded-md object-contain border"
          />
        ) : (
          <div
            key={i}
            className="flex items-center gap-1 rounded-md border bg-muted/50 px-2 py-1 text-xs"
          >
            <Paperclip className="h-3 w-3" />
            <span className="max-w-32 truncate">
              {block.filename ?? "file"}
            </span>
          </div>
        ),
      )}
    </div>
  );
};

const RecordUser = memo(function RecordUser({
  record,
  viewMode,
}: {
  record: LLMChatRecordVO;
  viewMode: boolean;
}) {
  const { content: recordContent, otid } = record;
  const initialContent = getBlockContent(recordContent, "content");

  const [textContent, setTextContent] = useState(initialContent);
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
      maxWidth="max-w-[60%]"
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
                const newParts = recordContent.parts.map((b) =>
                  b.type === "content" ? { ...b, data: textContent } : b,
                );
                updateRecord({
                  ...record,
                  content: { ...recordContent, parts: newParts },
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
        <AttachedFileList parts={recordContent.parts} />
        {!editing ? (
          <div>{textContent}</div>
        ) : (
          <Textarea
            onChange={(changed) => {
              setTextContent(changed.target.value);
            }}
            defaultValue={textContent}
          />
        )}
      </div>
    </RecordFrame>
  );
});

export default RecordUser;
