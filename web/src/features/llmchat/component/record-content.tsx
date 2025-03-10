import clsx from "clsx";
import rehypeSanitize from "rehype-sanitize";
import Icon from "@/common/component/icon";
import KSVG from "@/common/component/svg";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

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
  roleName,
  logo,
  timestamp,
  content,
  className,
  limitedHeight,
  setLimitedHeight,
  onAbort,
  onRegenerate,
  onCopy,
}: {
  role: string;
  logo?: string;
  roleName?: string;
  content: string;
  className?: string;
  timestamp?: Date;
  limitedHeight?: boolean;
  setLimitedHeight?: () => void;
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

  return (
    <div
      className={clsx(
        "flex md:flex-row md:space-y-0 md:space-x-4 mx-4 my-20",
        className,
        role === "user" && "justify-end"
      )}
    >
      <div className="w-8">
        {role !== "user" && (logo ? <KSVG inner={logo} /> : getAvatar(role))}
      </div>
      <div className="flex-col overflow-y-hidden">
        <div className="text-gray-500 text-xs space-x-2">
          <span>{roleName}</span>
          <span>{timestamp?.toISOString() ?? "Now"}</span>
        </div>
        <div className={clsx(limitedHeight && "h-40 overflow-hidden")}>
          {role === "user" ? (
            <div className="border border-cborder rounded-l-2xl rounded-br-2xl p-4 text-sm whitespace-pre-wrap bg-secondary">
              {content}
            </div>
          ) : (
            <ReactMarkdown
              className={
                "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2"
              }
              remarkPlugins={[remarkGfm]}
            >
              {content}
            </ReactMarkdown>
          )}
        </div>
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
          {limitedHeight !== undefined && (
            <button
              onClick={setLimitedHeight}
              className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
              aria-label="Regenerate"
              tabIndex={0}
            >
              <Icon.Ellipsis className="h-4 w-4 text-gray-700" />
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
