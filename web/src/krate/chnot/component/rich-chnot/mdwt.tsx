import type {
  Completion,
  CompletionContext,
  CompletionResult,
} from "@codemirror/autocomplete";
import {
  EditorSelection,
  type ReactCodeMirrorRef,
} from "@uiw/react-codemirror";
import {
  type RefObject,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { SaveState } from "@/common/types";
import useDebounce from "@/hooks/use-debounce";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";
import type { MdwtCommitReq } from "@/krate/mdwt/dto";
import {
  chnotTagNameList,
  mdwtCommit,
  mdwtRecordList,
} from "@/krate/mdwt/service";
import { toentTodoEventGuess } from "@/krate/toent/service";
import { chnotSingleSearch } from "../../service";
import type { RichPropProps } from "./rich-chnot";

const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const word = context.matchBefore(/#[^# ]*|^#* \[|^[ ]*- \[|\[\[/);
  let options: Completion[];
  if (!word || (word?.from === word?.to && !context.explicit)) {
    return null;
  } else if (word.text.startsWith("#")) {
    options = (
      await chnotTagNameList({
        query: word.text,
        start_index: 0,
        page_size: 20,
      })
    ).data.map((name) => {
      return { label: name, type: "hashtag" };
    });
  } else if (word.text.startsWith("[[")) {
    // [{ label: `[[backlink-ph]]`, type: "backlink" }]
    options = (
      await chnotSingleSearch({
        query: word.text.substring(3),
        start_index: 0,
        page_size: 10,
        kinds: [],
      })
    ).data.map((chnot) => {
      return {
        label: chnot.title ?? "",
        apply: `[[${chnot.meta.otid}]]`,
        type: "backlink",
      };
    });
  } else if (word.text.includes("# [") || word.text.includes("- [")) {
    options = (
      await toentTodoEventGuess({ input: word.text.replace(/.*\[/, "") })
    ).toents.map((toent) => {
      return { label: `{${toent}}`, type: "toent" };
    });
  } else {
    return null;
  }

  options.sort((e1, e2) => e1.label.length - e2.label.length);

  return {
    from: word.from,
    options: options,
    filter: false,
  };
};

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
  onPostSave,
  content: initialContent,
  onContentChange,
}: RichPropProps & {
  content?: string;
  onContentChange?: (content: string) => void;
}) => {
  // use RefObject to avoid rerender
  const cachedContentRef = useRef<string | undefined>(initialContent);
  const toSaveArg = useRef<MdwtCommitReq>(null);
  const [codeMirrorRef, setCodeMirrorRef] =
    useState<RefObject<ReactCodeMirrorRef | null>>();
  const [content, setContent] = useState<string | undefined>(initialContent);

  useEffect(() => {
    if (initialContent === undefined) {
      mdwtRecordList({
        mdwt_otids: [otid],
      }).then((rsp) => {
        const mdwt = rsp.mdwt_map[otid];
        if (mdwt?.content) {
          setContent(mdwt.content);
          if (onContentChange && cachedContentRef.current !== mdwt.content) {
            onContentChange(mdwt.content);
          }
        } else {
          setContent("");
        }
        cachedContentRef.current = mdwt?.content;
      });
    }
  }, [initialContent, onContentChange, otid]);

  const directlySave = async () => {
    if (toSaveArg.current) {
      try {
        onPostSave({
          otid,
          saveState: SaveState.Saving,
        });
        await mdwtCommit(toSaveArg.current);
        onPostSave({
          otid,
          saveState: SaveState.Saved,
        });
        toSaveArg.current = null;
      } catch (_ex) {
        onPostSave({ otid, saveState: SaveState.Error });
      }
    }
  };

  const debounceSave = useDebounce(
    async () => {
      directlySave();
    },
    2000,
    true,
  );
  const handleContentChange = useCallback(
    (content: string) => {
      cachedContentRef.current = content;
      const req: MdwtCommitReq = {
        mdwt: {
          otid: otid,
          content: content,
        },
      };
      toSaveArg.current = req;
      if (onContentChange) {
        onContentChange(content);
      }
      debounceSave();
    },
    [debounceSave, onContentChange, otid],
  );

  return readonly ? (
    <MarkdownViewer content={cachedContentRef.current ?? ""} keepBreak={true} />
  ) : (
    content !== undefined && (
      <div
        className="flex flex-col w-full h-full break-all"
        onBlur={() => directlySave()}
        role="none"
      >
        <MdwtEditorMemo
          content={content}
          onContentChange={handleContentChange}
          autoCompletion={chnotCompletions}
          foldGutter={false}
          setCodeMirrorRef={setCodeMirrorRef}
        />
        {/* dirty: fix codemirror height */}
        <div
          role="none"
          className="flex-grow cursor-text min-h-0 p-0 m-0"
          onClick={() => {
            if (codeMirrorRef?.current) {
              const editorView = codeMirrorRef.current.view;
              if (editorView) {
                const docLength = editorView.state.doc.length;
                editorView.dispatch({
                  selection: EditorSelection.cursor(docLength),
                  scrollIntoView: true,
                });
                editorView.focus();
              }
            }
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
            }
          }}
        ></div>
      </div>
    )
  );
};

export default MdwtChnot;
