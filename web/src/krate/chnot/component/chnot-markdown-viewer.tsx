import ReactMarkdown from "react-markdown"
import remarkGfm from "remark-gfm";

const MarkdownViewer = ({ content }: { content: string }) => {
    return <ReactMarkdown
        className={
            "prose prose-code:text-wrap prose-code:break-all prose-code:overflow-x-hidden prose-code:!p-2 min-w-full"
        }
        remarkPlugins={[remarkGfm]}
    >
        {content}
    </ReactMarkdown>
}

export default MarkdownViewer;