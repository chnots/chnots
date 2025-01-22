import { useState } from "react";
import Icon from "../icon";

const Input = ({
  onSend,
  disabled,
}: {
  onSend: (msg: string) => void;
  disabled: boolean;
}) => {
  const [message, setMessage] = useState("");

  const handleSend = () => {
    if (message.trim()) {
      onSend(message.trim());
      setMessage("");
    }
  };

  const handleKeyDown = (e: {
    key: string;
    ctrlKey: any;
    preventDefault: () => void;
  }) => {
    if (e.key === "Enter" && e.ctrlKey && !disabled) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="pl-3 p-1 border-t border-gray-300 flex items-start space-x-2">
      <textarea
        className="w-full p-1 rounded-md border-none  focus:outline-none focus:none resize-none"
        value={message}
        onChange={(e) => setMessage(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Type your message..."
      />
      <div className="h-full p-2 border-l">
        <button
          className="p-2 hover:bg-blue-100 h-auto w-auto rounded-xl"
          onClick={handleSend}
          disabled={disabled}
        >
          <Icon.Send></Icon.Send>
        </button>
      </div>
    </div>
  );
};

export default Input;
