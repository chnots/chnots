import useResizeObserver from "@react-hook/resize-observer";
import { useEffect, useRef, useState } from "react";
import { CompletionContext, CompletionResult } from "@codemirror/autocomplete";
import { MdwtEditorMemo } from "@/krate/mdwt/component/codemirror/mdwt-editor";
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
  readonly,
  onPostSave,
  otid,
  content: initialContent,
  onContentChange,
  tryFetch,
}: RichPropProps & {
  content?: string;
  onContentChange?: (content: string) => void;
  tryFetch: boolean;
}) => {
  console.log("render MdwtRecord", otid);

  const bodyRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  // use RefObject to avoid rerender
  const cachedContentRef = useRef<string>(initialContent ?? "");
  const saveStateRef = useRef<SaveState>(SaveState.Dirty);
  const toSaveArg = useRef<MdwtCommitReq>(null);
  const [refreshFlag, setRefreshFlag] = useState<boolean>();

  useEffect(() => {
    if (initialContent) {
      cachedContentRef.current = initialContent;
    } else {
      if (tryFetch) {
        mdwtRecordList({
          mdwt_otids: [otid],
        }).then((rsp) => {
          const mdwt = rsp.mdwt_map[otid];
          cachedContentRef.current = mdwt?.content ?? "";
          setRefreshFlag((prev) => !prev);
        });
      }
    }
  }, []);

  const directlySave = async () => {
    if (toSaveArg.current) {
      const title =
        toSaveArg.current.mdwt.content.length > 300
          ? toSaveArg.current.mdwt.content.substring(0, 300)
          : toSaveArg.current.mdwt.content;
      try {
        onPostSave({
          saveState: SaveState.Saving,
          title,
        });
        await mdwtCommit(toSaveArg.current);
        onPostSave({
          saveState: SaveState.Saved,
          title,
        });
        toSaveArg.current = null;
      } catch (_ex) {
        onPostSave({ saveState: SaveState.Error, title });
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
      className="w-full h-full break-all"
      onBlur={() => directlySave()}
      ref={bodyRef}
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
        height={height}
        foldGutter={false}
      />
    </div>
  );
};

export default MdwtChnot;
