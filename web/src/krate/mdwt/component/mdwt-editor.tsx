import type { TableEditDetail } from "@chnots/md-codemirror";
import {
  createCodemirrorTheme,
  generateKeybinding,
  Hashtag,
  headingBlocks,
  livePreview,
  MathConfig,
  TABLE_EDIT_EVENT,
} from "@chnots/md-codemirror";
import {
  autocompletion,
  type Completion,
  type CompletionContext,
  type CompletionResult,
} from "@codemirror/autocomplete";
import { EditorState } from "@codemirror/state";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { indentOnInput } from "@codemirror/language";
import { languages } from "@codemirror/language-data";
import { EditorView } from "@codemirror/view";
import { GFM } from "@lezer/markdown";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { wrappedLineIndent } from "codemirror-wrapped-line-indent";
import { format } from "date-fns";
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { toast } from "sonner";
import { chnotSearch } from "@/krate/chnot/service";
import { toentTodoEventGuess } from "@/krate/toent/service";
import { genTID } from "@/lib/id_util";
import { html2mdAsync } from "@/lib/markdown-utils";
import { chnotTagNameList } from "../service";
import {
  headingChnotCompletion,
  type HeadingCompletionConfig,
} from "./heading-chnot-completion";
import "katex/dist/katex.min.css";
import {
  Backlink,
  ChnotProps,
  todoHighlightPlugin,
} from "./codemirror/mdwt-extension";
import { TableEditorDialog } from "./table-editor";
import "./table-editor/table-editor.css";

const eventHandlers = EditorView.domEventHandlers({
  paste(event, view) {
    // adopted from https://github.com/Zettlr/Zettlr/blob/develop/source/common/modules/markdown-editor/plugins/md-paste-drop-handlers.ts
    const data = event.clipboardData;

    if (
      data === null ||
      (data.types.length === 1 && data.types[0] === "text/plain")
    ) {
      return false; // Let the default handler take over
    }

    const textIntention = data.types.includes("text/plain");

    const insertions: string[] = [];
    const allPromises: Array<Promise<void>> = [];

    if (textIntention && data.types.includes("text/html")) {
      const html = data.getData("text/html");
      const plain = data.getData("text/plain");

      const promise = html2mdAsync(html)
        .then((md) => {
          if (!md || md.length === 0) {
            insertions.push(plain);
            toast.info("Empty markdown conversation.");
          } else {
            insertions.push(md);
          }
        })
        .catch((_err) => {
          insertions.push(plain);
        });

      allPromises.push(promise);
    } else if (textIntention) {
      const plain = data.getData("text/plain");
      insertions.push(plain);
    } else {
      for (const _file of data.files) {
        allPromises.push(
          new Promise((_resolve, _reject) => {
            // TODO!
            /*             kfileUpload(file)
              .then((kfile?) => {
                if (kfile !== undefined) {
                  insertions.push(
                    `![${new Date().toISOString()}](${kfile.otid})`
                  );
                }
                resolve();
              })
              .catch((err) => {
                toast.info(`unable to handle ${file}, ${err}`);
                reject(err);
              }); */
          }),
        );
      }
    }

    Promise.allSettled(allPromises)
      .then(() => {
        // After all promises have been resolved or rejected, the
        // insertions array will contain everything we have to paste.
        const transaction = view.state.replaceSelection(insertions.join("\n"));
        view.dispatch(transaction);
      })
      .catch((_err) => {});

    return true;
  },
});

const SLASH_COMMANDS: Completion[] = [
  {
    label: "/time",
    displayLabel: "/time",
    detail: "Insert current date & time",
    apply: (_view, _completion, from, to) => {
      _view.dispatch({
        changes: { from, to, insert: format(new Date(), "yyyy-MM-dd HH:mm") },
      });
    },
    type: "keyword",
    boost: 1,
  },
  {
    label: "/date",
    displayLabel: "/date",
    detail: "Insert current date",
    apply: (_view, _completion, from, to) => {
      _view.dispatch({
        changes: { from, to, insert: format(new Date(), "yyyy-MM-dd") },
      });
    },
    type: "keyword",
    boost: 1,
  },
];

const slashCommandCompletions = (
  context: CompletionContext,
): CompletionResult | null => {
  const word = context.matchBefore(/\/[a-zA-Z]*$/);
  if (!word || (word.from === word.to && !context.explicit)) return null;
  if (word.text === "/") {
    return { from: word.from, options: SLASH_COMMANDS, filter: false };
  }
  const query = word.text.toLowerCase();
  const filtered = SLASH_COMMANDS.filter((c) =>
    c.label.toLowerCase().startsWith(query),
  );
  if (filtered.length === 0) return null;
  return { from: word.from, options: filtered, filter: false };
};

const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const slashResult = slashCommandCompletions(context);
  if (slashResult) return slashResult;

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
    options = (
      await chnotSearch({
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

export type EditorCustomization = {
  onCtrlEnter?: (view: EditorView) => boolean;
  autoFocus?: boolean;
};

export const EditorCustomContext = React.createContext<
  EditorCustomization | undefined
>(undefined);

const MdwtEditor = ({
  content,
  foldGutter,
  onContentChange,
  placeholder,
  setCodeMirrorRef: setCMRef,
  extraExtensions,
  headingCompletionConfig,
  readonly,
}: {
  content?: string;
  foldGutter: boolean;
  placeholder?: string;
  onContentChange: (content: string) => void;
  setCodeMirrorRef?: (ref: React.RefObject<ReactCodeMirrorRef | null>) => void;
  extraExtensions?: import("@codemirror/state").Extension[];
  headingCompletionConfig?: HeadingCompletionConfig;
  readonly?: boolean;
}) => {
  const codeMirror = useRef<ReactCodeMirrorRef>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const editorCustom = React.useContext(EditorCustomContext);

  const headingConfigRef = useRef<HeadingCompletionConfig | undefined>(
    headingCompletionConfig,
  );
  headingConfigRef.current = headingCompletionConfig;

  const headingSource = useMemo(
    () =>
      headingConfigRef.current
        ? headingChnotCompletion(() => headingConfigRef.current!)
        : null,
    [],
  );

  const [tableEditOpen, setTableEditOpen] = useState(false);
  const [tableEditRawText, setTableEditRawText] = useState("");
  const [tableEditFrom, setTableEditFrom] = useState(0);
  const [tableEditTo, setTableEditTo] = useState(0);

  const handleTableEdit = useCallback((e: Event) => {
    const detail = (e as CustomEvent<TableEditDetail>).detail;
    setTableEditRawText(detail.rawText);
    setTableEditFrom(detail.from);
    setTableEditTo(detail.to);
    setTableEditOpen(true);
  }, []);

  const handleTableSave = useCallback(
    (newText: string) => {
      const view = codeMirror.current?.view;
      if (!view) return;
      view.dispatch({
        changes: { from: tableEditFrom, to: tableEditTo, insert: newText },
      });
    },
    [tableEditFrom, tableEditTo],
  );

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    container.addEventListener(TABLE_EDIT_EVENT, handleTableEdit);
    return () => {
      container.removeEventListener(TABLE_EDIT_EVENT, handleTableEdit);
    };
  }, [handleTableEdit]);

  useEffect(() => {
    if (setCMRef) {
      setCMRef(codeMirror);
    }
  }, [setCMRef]);

  useEffect(() => {
    if (editorCustom?.autoFocus) {
      const timer = setTimeout(() => {
        const view = codeMirror.current?.view;
        if (view) {
          const end = view.state.doc.length;
          view.dispatch({
            selection: { anchor: end },
          });
          view.focus();
        }
      }, 50);
      return () => clearTimeout(timer);
    }
  }, [editorCustom?.autoFocus]);

  const markdownExtension = markdown({
    base: markdownLanguage,
    codeLanguages: languages,
    addKeymap: true,
    completeHTMLTags: false,
    extensions: [Backlink, Hashtag, ChnotProps, GFM, MathConfig],
  });

  const extensions = [
    markdownExtension,
    generateKeybinding(editorCustom?.onCtrlEnter),

    EditorView.lineWrapping,
    wrappedLineIndent,

    todoHighlightPlugin,

    livePreview(),
    headingBlocks({ genTID }),
    createCodemirrorTheme(),

    eventHandlers,

    indentOnInput(),
    autocompletion({
      override: [
        ...(headingSource ? [headingSource] : []),
        chnotCompletions,
      ],
    }),
    ...(readonly ? [EditorState.readOnly.of(true)] : []),
    EditorView.theme({
      "&": {
        height: "100%",
      },
      "&.cm-editor": {
        height: "100%",
      },
      ".cm-scroller": {
        height: "100%",
        overflow: "auto",
        justifyContent: "center",
      },
      ".cm-content": {
        minHeight: "100%",
        maxWidth: "56rem",
        width: "100%",
        flex: "none",
      },
    }),
    ...(extraExtensions ?? []),
  ];

  return (
    <div ref={containerRef} style={{ height: "100%" }}>
      <CodeMirror
        height="100%"
        extensions={extensions}
        ref={codeMirror}
        style={{
          font: "sans-serif",
          height: "100%",
        }}
        value={content}
        basicSetup={{
          lineNumbers: false,
          highlightActiveLineGutter: false,
          foldGutter: foldGutter,
          closeBrackets: false,
        }}
        placeholder={placeholder ?? "Take a chnot"}
        onChange={(e) => onContentChange(e)}
      />
      <TableEditorDialog
        open={tableEditOpen}
        rawText={tableEditRawText}
        onOpenChange={setTableEditOpen}
        onSave={handleTableSave}
      />
    </div>
  );
};

export const MdwtEditorMemo = React.memo(MdwtEditor);

export default MdwtEditor;
