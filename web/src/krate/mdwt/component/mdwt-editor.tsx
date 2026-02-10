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
import { decoratorExtension } from "jolpin-codemirror";
import React, { useEffect, useRef } from "react";
import { toast } from "sonner";
import { generateKeybinding } from "@/krate/mdwt/component/codemirror/keybinding";
import { html2mdAsync } from "@/lib/markdown-utils";
import {
  Backlink,
  ChnotProps,
  Hashtag,
  todoHighlightPlugin,
} from "./codemirror/mdwt-extension";
import { createCodemirrorTheme } from "./codemirror/theme";
import { chnotSingleSearch } from "@/krate/chnot/service";
import { toentTodoEventGuess } from "@/krate/toent/service";
import { chnotTagNameList } from "../service";

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

const MdwtEditor = ({
  content,
  foldGutter,
  height,
  onContentChange,
  placeholder,
  setCodeMirrorRef: setCMRef,
}: {
  content?: string;
  foldGutter: boolean;
  height?: number;
  placeholder?: string;
  onContentChange: (content: string) => void;
  setCodeMirrorRef?: (ref: React.RefObject<ReactCodeMirrorRef | null>) => void;
}) => {
  const codeMirror = useRef<ReactCodeMirrorRef>(null);
  useEffect(() => {
    if (setCMRef) {
      setCMRef(codeMirror);
    }
  }, [setCMRef]);

  const markdownExtension = markdown({
    base: markdownLanguage,
    codeLanguages: languages,
    addKeymap: true,
    completeHTMLTags: false,
    extensions: [Backlink, Hashtag, ChnotProps, GFM],
  });

  const extensions = [
    markdownExtension,
    generateKeybinding(),

    EditorView.lineWrapping,
    wrappedLineIndent,

    todoHighlightPlugin,

    decoratorExtension,
    createCodemirrorTheme(),

    eventHandlers,

    indentOnInput(),
    autocompletion({
      override: [chnotCompletions],
    }),
  ];

  return (
    <CodeMirror
      height={height ? `${height}px` : undefined}
      extensions={extensions}
      ref={codeMirror}
      style={{
        font: "sans-serif",
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
  );
};

export const MdwtEditorMemo = React.memo(MdwtEditor);

export default MdwtEditor;
