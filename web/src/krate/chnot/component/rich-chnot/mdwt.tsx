import useResizeObserver from "@react-hook/resize-observer";
import { RefObject, useEffect, useRef, useState } from "react";
import { CompletionContext, CompletionResult } from "@codemirror/autocomplete";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";
import useDebounce from "@/hooks/use-debounce";
import { SaveState } from "@/common/types";
import { RichPropProps } from "./rich-chnot";
import {
  chnotTagNameList,
  mdwtCommit,
  mdwtRecordList,
} from "@/krate/mdwt/service";
import { MdwtCommitReq } from "@/krate/mdwt/dto";
import { toentTodoEventGuess } from "@/krate/toent/service";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { EditorSelection, ReactCodeMirrorRef } from "@uiw/react-codemirror";

const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const word = context.matchBefore(/#[^# ]*|^#* \[|^[ ]*- \[|\[\[/);
  let options;
  if (!word || (word?.from == word?.to && !context.explicit)) {
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
    options = [{ label: `[[backlink-ph]]`, type: "backlink" }];
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

const MdwtChnot = ({
  otid,
  readonly,
  onPostSave,
  content: initialContent,
  tryFetch,
  onContentChange,
}: RichPropProps & {
  content?: string;
  tryFetch: boolean;
  onContentChange?: (content: string) => void;
}) => {
  // use RefObject to avoid rerender
  const cachedContentRef = useRef<string>(initialContent ?? "");
  const saveStateRef = useRef<SaveState>(SaveState.Dirty);
  const toSaveArg = useRef<MdwtCommitReq>(null);
  const [refreshFlag, setRefreshFlag] = useState<boolean>();
  const [codeMirrorRef, setCodeMirrorRef] =
    useState<RefObject<ReactCodeMirrorRef | null>>();

  useEffect(() => {
    if (initialContent) {
      cachedContentRef.current = initialContent;
    } else {
      if (tryFetch) {
        mdwtRecordList({
          mdwt_otids: [otid],
        }).then((rsp) => {
          const mdwt = rsp.mdwt_map[otid];
          if (
            onContentChange &&
            mdwt.content &&
            cachedContentRef.current !== mdwt.content
          ) {
            onContentChange(mdwt.content);
          }
          cachedContentRef.current = mdwt?.content ?? "";
          setRefreshFlag((prev) => !prev);
        });
      }
    }
  }, []);

  const directlySave = async () => {
    if (toSaveArg.current) {
      try {
        onPostSave({
          saveState: SaveState.Saving,
        });
        await mdwtCommit(toSaveArg.current);
        onPostSave({
          saveState: SaveState.Saved,
        });
        toSaveArg.current = null;
      } catch (_ex) {
        onPostSave({ saveState: SaveState.Error });
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

  return readonly ? (
    <MarkdownViewer content={cachedContentRef.current ?? ""} keepBreak={true} />
  ) : (
    <div
      className="flex flex-col w-full h-full break-all"
      onBlur={() => directlySave()}
    >
      <MdwtEditorMemo
        content={cachedContentRef.current}
        onContentChange={(content) => {
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
        }}
        autoCompletion={chnotCompletions}
        foldGutter={false}
        setCodeMirrorRef={setCodeMirrorRef}
      />
      {/* dirty: fix codemirror height */}
      <div
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
      ></div>
    </div>
  );
};

export default MdwtChnot;
