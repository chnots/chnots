import { LLMChatRecord } from "@/krate/llmchat/po";
import RecordFrame from "./record-frame";

const RecordUser = ({ record }: { record: LLMChatRecord }) => {
  const { content, otid } = record;

  return (
    <RecordFrame
      timestamp={new Date(otid / 1e3).toISOString()}
      justifyEnd={true}
    >
      <div className="border border-cborder rounded-l-2xl rounded-br-2xl p-4 text-sm whitespace-pre-wrap kc-accent">
        {content}
      </div>
    </RecordFrame>
  );
};

export default RecordUser;
