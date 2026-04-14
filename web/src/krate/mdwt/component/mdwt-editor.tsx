import {
  autocompletion,
  type Completion,
  type CompletionContext,
  type CompletionResult,
} from "@codemirror/autocomplete";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { indentOnInput } from "@codemirror/language";
import { languages } from "@codemirror/language-data";
import { EditorView } from "@codemirror/view";
import { GFM } from "@lezer/markdown";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { wrappedLineIndent } from "codemirror-wrapped-line-indent";
import type { TableEditDetail } from "jolpin-codemirror";
import { livePreview, TABLE_EDIT_EVENT } from "jolpin-codemirror";
import React, { useCallback, useEffect, useRef, useState } from "react";
import { toast } from "sonner";
import { chnotSearch } from "@/krate/chnot/service";
import { generateKeybinding } from "@/krate/mdwt/component/codemirror/keybinding";
import { toentTodoEventGuess } from "@/krate/toent/service";
import { html2mdAsync } from "@/lib/markdown-utils";
import { chnotTagNameList } from "../service";
import "katex/dist/katex.min.css";
import {
  Backlink,
  ChnotProps,
  Hashtag,
  MathConfig,
  todoHighlightPlugin,
} from "./codemirror/mdwt-extension";
import { createCodemirrorTheme } from "./codemirror/theme";
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
  height,
  fillParentHeight,
  onContentChange,
  placeholder,
  setCodeMirrorRef: setCMRef,
}: {
  content?: string;
  foldGutter: boolean;
  height?: number | string;
  fillParentHeight?: boolean;
  placeholder?: string;
  onContentChange: (content: string) => void;
  setCodeMirrorRef?: (ref: React.RefObject<ReactCodeMirrorRef | null>) => void;
}) => {
  const codeMirror = useRef<ReactCodeMirrorRef>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const editorCustom = React.useContext(EditorCustomContext);

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
    createCodemirrorTheme(),

    eventHandlers,

    indentOnInput(),
    autocompletion({
      override: [chnotCompletions],
    }),
    ...(fillParentHeight
      ? [
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
            },
            ".cm-content": {
              minHeight: "100%",
            },
          }),
        ]
      : []),
  ];

  return (
    <div ref={containerRef} style={{ height: "100%" }}>
      <CodeMirror
        height={
          fillParentHeight
            ? "100%"
            : typeof height === "number"
              ? `${height}px`
              : typeof height === "string"
                ? height
                : undefined
        }
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
