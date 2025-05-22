import { useRef } from "react";
import { EditorView, KeyBinding } from "@codemirror/view";
import { languages } from "@codemirror/language-data";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import {
  deleteMarkupBackward,
  insertNewlineContinueMarkup,
  markdown,
  markdownLanguage,
} from "@codemirror/lang-markdown";
import { toast } from "sonner";
import { html2mdAsync } from "@/utils/markdown-utils";
import React from "react";
import { useAttachmentStore } from "@/store/kfile/store";
import { CompletionContext, CompletionResult } from "@codemirror/autocomplete";
import { autocompletion } from "@codemirror/autocomplete";

import { indentationMarkers } from "@replit/codemirror-indentation-markers";
import { wrappedLineIndent } from "codemirror-wrapped-line-indent";
import { MatchDecorator, ViewPlugin, Decoration } from "@codemirror/view";
import { kfileUpload } from "@/store/kfile/service";

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
        .catch((err) => {
          console.error(err);
          insertions.push(plain);
        });

      allPromises.push(promise);
    } else if (textIntention) {
      const plain = data.getData("text/plain");
      insertions.push(plain);
    } else {
      for (const file of data.files) {
        allPromises.push(
          new Promise((resolve, reject) => {
            // TODO!
            /*             kfileUpload(file)
              .then((kfile?) => {
                if (kfile !== undefined) {
                  insertions.push(
                    `![${new Date().toISOString()}](${kfile.id})`
                  );
                }
                resolve();
              })
              .catch((err) => {
                toast.info(`unable to handle ${file}, ${err}`);
                reject(err);
              }); */
          })
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
      .catch((err) => console.error(err));

    return true;
  },
});

const editorTheme = EditorView.theme({
  "&.cm-editor": {
    background: "transparent !important",
  },
  // To Remove outline when focused, https://github.com/uiwjs/react-codemirror/issues/643
  /*   "&.cm-editor.cm-focused": {
    outline: "none",
  }, */
  /*   ".cm-line": {
    background: "transparent !important",
  }, */
  ".cm-content": {
    padding: "1em",
  },
  ".cm-lineWrapping": {
    wordBreak: "break-all",
  },
  ".hashtag": {
    border: "1px solid #602533",
    padding: "1px",
    borderRadius: "0.2em",
    color: "#682d4b",
  },
});

export const markdownKeymap: readonly KeyBinding[] = [
  { key: "Enter", run: insertNewlineContinueMarkup },
  { key: "Backspace", run: deleteMarkupBackward },
];

const CodeMirrorEditor = ({
  content,
  onContentChange,
  autoCompletion,
  foldGutter,
  height,
}: {
  content?: string;
  onContentChange: (content: string) => void;
  autoCompletion: (
    context: CompletionContext
  ) => Promise<CompletionResult | null>;
  foldGutter: boolean;
  height: number;
}) => {
  const codeMirror = useRef<ReactCodeMirrorRef>(null);

  const mentionDeco = Decoration.mark({ class: "mention" });
  const tagDeco = Decoration.mark({ class: "hashtag" });
  const highlightDeco = Decoration.mark({ class: "highlight" });
  const decorator = new MatchDecorator({
    regexp: /(@\w+)|(::.*?::)|(#[^ #[\]]+)/g,
    decoration: (m) => (m[1] ? mentionDeco : m[2] ? highlightDeco : tagDeco),
  });

  const markPlugin = ViewPlugin.define(
    (view) => ({
      decorations: decorator.createDeco(view),
      update(u) {
        this.decorations = decorator.updateDeco(u, this.decorations);
      },
    }),
    {
      decorations: (v) => v.decorations,
    }
  );

  const md = markdown({
    base: markdownLanguage,
    codeLanguages: languages,
    addKeymap: true,
    completeHTMLTags: false,
  });

  const _extensions = [
    md,
    EditorView.lineWrapping,
    editorTheme,
    eventHandlers,
    autocompletion({
      override: [(context) => autoCompletion(context)],
    }),
    indentationMarkers(),
    wrappedLineIndent,
    markPlugin.extension,
  ];

  return (
    <CodeMirror
      height={`${height}px`}
      extensions={_extensions}
      ref={codeMirror}
      style={{
        font: "serif",
      }}
      value={content}
      basicSetup={{
        lineNumbers: false,
        highlightActiveLineGutter: false,
        foldGutter: foldGutter,
        closeBrackets: false,
      }}
      placeholder={"Take a Chnot"}
      onChange={(e) => onContentChange(e)}
    />
  );
};

export const CodeMirrorEditorMemo = React.memo(CodeMirrorEditor);

export default CodeMirrorEditor;
