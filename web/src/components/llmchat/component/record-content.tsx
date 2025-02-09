import clsx from "clsx";
import MarkdownPreview from "@uiw/react-markdown-preview";
import rehypeSanitize from "rehype-sanitize";
import Icon from "@/components/icon";
import { LLMChatBot, useLLMChatStore } from "@/store/llmchat";
import KSVG from "@/components/svg";
import { log } from "console";

const getAvatar = (role: string) => {
  switch (role) {
    case "user":
      return <Icon.User className="h-8 w-8 mr-4" />;
    case "assistant":
      return <Icon.Bot className="h-8 w-8 mr-4" />;
    case "assistant-response":
      return <Icon.Bot className="h-8 w-8 mr-4" />;
    case "system":
      return <Icon.Contact className="h-8 w-8 mr-4" />;
    default:
      return <></>;
  }
};

const RecordContent = ({
  role,
  logo,
  timestamp,
  content,
  className,
  onAbort,
  onRegenerate,
  onCopy,
}: {
  role: string;
  logo?: string;
  content: string;
  className?: string;
  timestamp?: Date;
  onAbort?: () => void;
  onRegenerate?: () => void;
  onCopy?: () => void;
}) => {
  const handleCopy = () => {
    if (onCopy) {
      onCopy();
    } else {
      navigator.clipboard.writeText(content);
    }
  };

  const rehypePlugins = [rehypeSanitize];

  return (
    <div
      className={clsx(
        "flex md:flex-row md:space-y-0 md:space-x-4 mx-4 my-8",
        className,
        role === "user" && "justify-end"
      )}
    >
      <div className="w-8">
        {role !== "user" && (logo ? <KSVG inner={logo} /> : getAvatar(role))}
      </div>
      <div className="flex-col">
        <div className="text-gray-500 text-xs">
          {timestamp?.toISOString() ?? "Now"}
        </div>
        {role === "user" ? (
          <div className="border border-gray-200 rounded-l-2xl rounded-br-2xl p-4 text-sm whitespace-pre-wrap">
            {content}
          </div>
        ) : (
          <MarkdownPreview
            source={content}
            rehypePlugins={rehypePlugins}
            className="mr-4 text-sm"
          />
        )}
        <div className="space-x-2 mt-1">
          {onAbort && (
            <button
              onClick={onAbort}
              className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
              aria-label="Abort"
              tabIndex={0}
            >
              <Icon.Square className="h-4 w-4 text-gray-700" />
            </button>
          )}
          {onRegenerate && (
            <button
              onClick={onRegenerate}
              className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
              aria-label="Regenerate"
              tabIndex={0}
            >
              <Icon.RotateCcw className="h-4 w-4 text-gray-700" />
            </button>
          )}
          <button
            onClick={handleCopy}
            className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
            aria-label="Copy"
            tabIndex={0}
          >
            <Icon.Copy className="h-4 w-4 text-gray-700" />
          </button>
        </div>
      </div>
    </div>
  );
};

export default RecordContent;
