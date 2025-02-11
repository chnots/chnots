import { useCallback, useRef, useState } from "react";
import Icon from "@/common/component/icon";
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
  const textareaRef = useRef<HTMLTextAreaElement>(null);

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
    const record: LLMChatRecord = {
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

  const handleTextareaChange = useCallback(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, []);

  return (
    <div className="pl-3 p-1 flex justify-center space-x-2 mb-2">
      <div className="flex flex-row max-w-3xl w-3xl p-2 rounded-2xl border kborder">
        <textarea
          className="w-full p-1 h-auto max-h-60 border-none focus:outline-none focus:none resize-none"
          onChange={(e) => {
            handleTextareaChange();
            setMessage(e.target.value);
          }}
          value={message}
          onKeyDown={handleKeyDown}
          placeholder="Type your message..."
          ref={textareaRef}
        />
        <div className="flex justify-end">
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
    </div>
  );
};

export default LLMChatSessionInput;
