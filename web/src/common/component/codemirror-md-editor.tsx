import { RefObject, useEffect, useRef, useState } from "react";
import { EditorView, KeyBinding } from "@codemirror/view";
import { languages } from "@codemirror/language-data";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { deleteMarkupBackward, insertNewlineContinueMarkup, markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { toast } from "sonner";
import { html2mdAsync } from "@/utils/markdown-utils";
import { useAttachmentStore } from "@/store/attachment";
import React from "react";
import clsx from "clsx";

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
            useAttachmentStore
              .getState()
              .upload(file)
              .then((resource?) => {
                if (resource !== undefined) {
                  insertions.push(
                    `![${new Date().toISOString()}](${resource.id})`
                  );
                }
                resolve();
              })
              .catch((err) => {
                toast.info(`unable to handle ${file}, ${err}`);
                reject(err);
              });
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
  "&.cm-editor.cm-focused": {
    outline: "none",
  },
  ".cm-line": {
    background: "transparent !important",
  },
  ".cm-content": {
    padding: "1em",
  },
});

export const markdownKeymap: readonly KeyBinding[] = [
  {key: "Enter", run: insertNewlineContinueMarkup},
  {key: "Backspace", run: deleteMarkupBackward}
]

const CodeMirrorEditor = ({
  id,
  onChange,
  className,
  fetchDefaultValue,
}: {
  id: string;
  onChange: RefObject<(metaId: string, content: string) => void>;
  fetchDefaultValue: (id: string) => Promise<string | undefined>;
  className?: string;
}) => {
  const cmRef = React.useRef<HTMLDivElement>(null);
  const codeMirror = useRef<ReactCodeMirrorRef>(null);
  const [content, setContent] = useState<string>();
  const md = 
    markdown({
      base: markdownLanguage,
      codeLanguages: languages,
      addKeymap: true,
      completeHTMLTags: false
    });
    md.support;

  const extensions = [
    md,
    EditorView.lineWrapping,
    editorTheme,
    eventHandlers,
  ];

  useEffect(() => {
    const fetchData = async () => {
      const content = await fetchDefaultValue(id);
      setContent(content);
    };
    fetchData();
  }, [setContent, id]);

  return (
    <div className={clsx("w-full h-full", className)} ref={cmRef}>
      <CodeMirror
        height={`${cmRef.current?.getBoundingClientRect().height ?? 0}px`}
        extensions={extensions}
        ref={codeMirror}
        style={{
          font: "serif",
        }}
        value={content}
        basicSetup={{
          lineNumbers: false,
          highlightActiveLineGutter: false,
          foldGutter: false,
        }}
        placeholder={"Chnot"}
        onChange={(e) => onChange.current(id, e)}
      />
    </div>
  );
};

export const CodeMirrorEditorMemo = React.memo(CodeMirrorEditor);

export default CodeMirrorEditor;
