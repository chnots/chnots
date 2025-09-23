import { useEffect, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import LLMChatBotSelect from "./bot-select";

const UserInput = ({
  disabled,
  onAppendRecord,
}: {
  disabled: boolean;
  onAppendRecord: (content: string) => boolean;
}) => {
  const [message, setMessage] = useState<string>();
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const handleKeyDown = (e: {
    key: string;
    ctrlKey: any;
    preventDefault: () => void;
  }) => {
    if (e.key === "Enter" && e.ctrlKey && !disabled && message) {
      e.preventDefault();
      handleSendUserMsg(message);
    }
  };

  const handleSendUserMsg = (msg: string) => {
    if (onAppendRecord(msg)) {
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
      <div className="flex flex-col max-w-3xl w-3xl p-2 rounded-xl border shadow-xl">
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
          <LLMChatBotSelect />
          <Button
            onClick={() => {
              if (message) {
                handleSendUserMsg(message);
              }
            }}
            disabled={disabled}
          >
            <Icon.Send className="w-4 h-4" />
          </Button>
        </div>
      </div>
    </div>
  );
};

export default UserInput;
