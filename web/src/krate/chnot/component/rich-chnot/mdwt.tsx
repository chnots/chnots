import { useCallback, useEffect, useRef, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { SaveState } from "@/common/types";
import useDebounce from "@/hooks/use-debounce";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";
import type { MdwtCommitReq } from "@/krate/mdwt/dto";
import { mdwtCommit, mdwtRecordList } from "@/krate/mdwt/service";
import { ChnotKind } from "../../po";
import type { RichPropProps } from "./rich-mdwt-side";

const MarkdownViewer = ({
  content: initialContent,
  keepBreak,
}: {
  content: string;
  keepBreak?: boolean;
}) => {
  const content = keepBreak
    ? initialContent.replaceAll("\n", "  \n")
    : initialContent;
  return (
    <div
      className={
        "prose prose-sm max-w-none prose-code:text-wrap prose-code:break-all prose-code:!p-2 min-w-full break-all h-full"
      }
    >
      <ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown>
    </div>
  );
};

/**
 *
 * @param content if content is undefined, try to fetch mdwt, or just use it.
 * @returns
 */
const MdwtChnot = ({
  otid,
  readonly,
  placeholder,
  onPostSave,
  onContentChange,
  content: initialContent,
  fillParentHeight,
}: RichPropProps & {
  placeholder?: string;
  content?: string;
  onContentChange?: (content: string) => void;
  fillParentHeight?: boolean;
}) => {
  // use RefObject to avoid rerender
  const cachedContentRef = useRef<string | undefined>(initialContent);
  const toSaveArg = useRef<MdwtCommitReq>(null);
  const [content, setContent] = useState<string | undefined>(initialContent);

  useEffect(() => {
    if (initialContent === undefined) {
      mdwtRecordList({
        mdwt_otids: [otid],
      }).then((rsp) => {
        const mdwt = rsp.mdwt_map[otid];
        if (mdwt?.content) {
          setContent(mdwt.content);
        } else {
          setContent("");
        }
        if (onContentChange && mdwt?.content) {
          onContentChange(mdwt.content);
        }
        cachedContentRef.current = mdwt?.content;
      });
    }
  }, [initialContent, otid]);

  const directlySave = async () => {
    if (toSaveArg.current) {
      try {
        onPostSave({
          otid,
          saveState: SaveState.Saving,
          kind: ChnotKind.MDWT,
        });
        const rsp = await mdwtCommit(toSaveArg.current);
        onPostSave({
          otid,
          saveState: SaveState.Saved,
          title: rsp.title,
          kind: ChnotKind.MDWT,
        });
        toSaveArg.current = null;
      } catch (_ex) {
        onPostSave({
          otid,
          saveState: SaveState.Error,
          title: "<unable to save>",
          kind: ChnotKind.MDWT,
        });
      }
    }
  };

  const debounceSave = useDebounce(
    async () => {
      directlySave();
    },
    {
      duration: 2000,
      executeOnUnmount: true,
    },
  );
  const handleContentChange = useCallback(
    (content: string) => {
      if (content !== cachedContentRef.current) {
        if (onContentChange) {
          onContentChange(content);
        }
        cachedContentRef.current = content;
      }
      const req: MdwtCommitReq = {
        mdwt: {
          otid: otid,
          content: content,
        },
      };
      toSaveArg.current = req;
      debounceSave();
    },
    [debounceSave, otid],
  );

  return readonly ? (
    <MarkdownViewer content={cachedContentRef.current ?? ""} keepBreak={true} />
  ) : (
    content !== undefined && (
      <div
        className="flex flex-col w-full h-full min-h-0 break-all"
        onBlur={() => directlySave()}
        role="none"
      >
        <div
          className={
            fillParentHeight
              ? "flex-1 min-h-0 h-full overflow-hidden"
              : undefined
          }
        >
          <MdwtEditorMemo
            placeholder={placeholder}
            content={content}
            onContentChange={handleContentChange}
            foldGutter={false}
            fillParentHeight={fillParentHeight}
          />
        </div>
      </div>
    )
  );
};

export default MdwtChnot;
