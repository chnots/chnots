import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

const MarkdownViewer = ({ content }: { content: string }) => {
  return (
    <div
      className={
        "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2 min-w-full"
      }
    >
      <ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown>
    </div>
  );
};

export default MarkdownViewer;
