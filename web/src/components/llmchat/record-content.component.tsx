import clsx from "clsx";
import MarkdownPreview from "@uiw/react-markdown-preview";
import Icon from "../icon";

const bgColor = (role: string) => {
  let color = "";
  if (role === "user") {
    color = "bg-red-50";
  } else if (role === "assistant") {
    color = "bg-blue-50";
  } else if (role === "system") {
    color = "bg-green-50";
  }

  return color;
};

const RecordContent = ({
  role,
  content,
  canAbort,
  canGenerate,
  onAbort,
  onRegenerate,
}: {
  role: string;
  content: string;
  canAbort?: boolean;
  onAbort?: () => void;
  canGenerate?: boolean;
  onRegenerate?: () => void;
}) => {
  return (
    <div className={clsx(bgColor(role), "p-3 border-b border-b-gray-200")}>
      <MarkdownPreview source={content} style={{ padding: 16 }} />
      {canAbort === true ? (
        <button onClick={onAbort}>
          <Icon.Ban />
        </button>
      ) : (
        <></>
      )}
      {canGenerate === true ? (
        <button onClick={onRegenerate}>
          <Icon.RotateCcw />
        </button>
      ) : (
        <></>
      )}
    </div>
  );
};

export default RecordContent;
