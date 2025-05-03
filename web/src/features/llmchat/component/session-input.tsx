import { ReactNode, useCallback, useEffect, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { v4 as uuid } from "uuid";
import { LLMChatRecord } from "@/store/llmchat/db";
import { LLMChatSessionDetail } from "@/store/llmchat/dto";
import { useLLMChatStore } from "@/store/llmchat/store";

const LLMChatSessionInput = ({
  disabled,
  sessionDetail,
  appendRecord,
  botSelect,
}: {
  disabled: boolean;
  sessionDetail?: LLMChatSessionDetail;
  appendRecord: (record: LLMChatRecord) => Promise<boolean>;
  botSelect?: ReactNode;
}) => {
  const [message, setMessage] = useState<string>();
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [newSessionCount, setNewSessionCount] = useState(0);
  const { setCurrentSession } = useLLMChatStore();

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
      reasoning_content: "",
      role: "user",
      insert_time: new Date(),
    };
    const flag = await appendRecord(record);
    if (flag) {
      setMessage("");
    }
  };

  useEffect(() => {
    if (textareaRef.current) {
      if (message?.length == 0) {
        textareaRef.current.style.height = "auto";
      } else {
        textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
      }
    }
  }, [textareaRef, message]);

  return (
    <div className="pl-3 p-1 flex justify-center space-x-2 mb-2">
      <div className="flex flex-col max-w-3xl w-3xl p-2 rounded-2xl border kborder shadow-xl">
        <textarea
          className="w-full p-1 h-auto max-h-60 border-none focus:outline-none focus:none resize-none"
          onChange={(e) => {
            setMessage(e.target.value);
          }}
          value={message}
          onKeyDown={handleKeyDown}
          placeholder="Type your question..."
          ref={textareaRef}
        />
        <div className="flex justify-between">
          <div className="flex space-x-4 align-middle items-center">
            <div
              onClick={() => {
                setCurrentSession(undefined);
              }}
              className="kborder bg-accent border rounded-lg p-1 h-7 mx-2 flex items-center space-x-1 hover:cursor-pointer hover:bg-green-50 text-xs"
            >
              <Icon.BadgePlus className="w-4 h-4" />
              <span>New</span>
            </div>

            <div>{botSelect}</div>
          </div>
          <button
            className="p-1 hover:bg-blue-100 h-auto w-auto rounded-xl"
            onClick={() => {
              if (message && sessionDetail) {
                handleSendUserMsg(message, sessionDetail);
              }
            }}
            disabled={disabled}
          >
            <Icon.Send />
          </button>
        </div>
      </div>
    </div>
  );
};

export default LLMChatSessionInput;
