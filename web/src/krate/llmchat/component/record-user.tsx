import { LLMChatRecord } from "@/krate/llmchat/store/db";
import RecordFrame from "./record-frame";

const RecordUser = ({ record }: { record: LLMChatRecord }) => {
  const { content, insert_time } = record;

  return (
    <RecordFrame timestamp={insert_time} justifyEnd={true}>
      <div className="border border-cborder rounded-l-2xl rounded-br-2xl p-4 text-sm whitespace-pre-wrap kc-accent">
        {content}
      </div>
    </RecordFrame>
  );
};

export default RecordUser;
