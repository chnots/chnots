import { useRef, useState } from "react";
import Icon from "../../icon";
import { LLMChatRecord, LLMChatSessionDetail } from "@/store/llmchat";
import { v4 as uuid } from "uuid";

const LLMChatSessionInput = ({
  disabled,
  sessionDetail,
  appendRecord,
}: {
  disabled: boolean;
  sessionDetail?: LLMChatSessionDetail;
  appendRecord: (record: LLMChatRecord) => Promise<boolean>;
}) => {
  const [message, setMessage] = useState<string>();
  const handleKeyDown = (e: {
    key: string;
    ctrlKey: any;
    preventDefault: () => void;
  }) => {
    if (
      e.key === "Enter" &&
      e.ctrlKey &&
      !disabled &&
      message &&
      sessionDetail
    ) {
      e.preventDefault();

      handleSendUserMsg(message, sessionDetail);
    }
  };

  const handleSendUserMsg = async (
    msg: string,
    sessionDetail: LLMChatSessionDetail
  ) => {
    let record: LLMChatRecord = {
      id: uuid(),
      session_id: sessionDetail.session.id,
      pre_record_id: sessionDetail.records.at(-1)?.id,
      content: msg,
      role: "user",
      insert_time: new Date(),
    };
    const flag = await appendRecord(record);
    if (flag) {
      setMessage("");
    }
  };

  return (
    <div className="pl-3 p-1 max-h-50 border-t border-gray-300 flex items-start space-x-2">
      <textarea
        className="w-full p-1 rounded-md border-none focus:outline-none focus:none resize-none"
        onChange={(e) => setMessage(e.target.value)}
        value={message}
        onKeyDown={handleKeyDown}
        placeholder="Type your message..."
      />
      <div className="h-full p-2 border-l">
        <button
          className="p-2 hover:bg-blue-100 h-auto w-auto rounded-xl"
          onClick={() => {
            if (message && sessionDetail) {
              handleSendUserMsg(message, sessionDetail);
            }
          }}
          disabled={disabled}
        >
          <Icon.Send></Icon.Send>
        </button>
      </div>
    </div>
  );
};

export default LLMChatSessionInput;
