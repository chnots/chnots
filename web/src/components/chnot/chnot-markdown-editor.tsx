import { v4 as uuid } from "uuid";
import { RefObject, useCallback, useEffect, useRef, useState } from "react";
import { ChnotOverwriteReq } from "@/store/chnot";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { useChnotStore } from "@/store/chnot";
import { toast } from "sonner";
import { html2mdAsync } from "@/utils/markdown-utils";
import { useAttachmentStore } from "@/store/attachment";
import React from "react";

export interface ChnotEditState {
  isUploadingResource: boolean;
  isRequesting: boolean;
  isComposing: boolean;
}

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
              .then((resource) => {
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

const CMEditor: React.FC<{
  metaId: string;
  onChange: RefObject<(_content: string) => void>;
}> = React.memo(({ metaId: meta_id, onChange }) => {
  console.log("CMEditor changed");

  const cmRef = React.useRef<HTMLDivElement>(null);
  const codeMirror = useRef<ReactCodeMirrorRef>(null);
  const [content, setContent] = useState<string>();
  const { queryChnot } = useChnotStore();

  const extensions = [
    markdown({ base: markdownLanguage, codeLanguages: languages }),
    EditorView.lineWrapping,
    editorTheme,
    eventHandlers,
  ];

  useEffect(() => {
    const fetchData = async () => {
      const chnot = await queryChnot({ meta_id: meta_id });
      setContent(chnot?.record.content);
    };
    fetchData();
  }, [setContent, meta_id]);

  return (
    <div className="w-full h-full" ref={cmRef}>
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
        onChange={(e) => onChange.current(e)}
      />
    </div>
  );
});

export const ChnotMarkdownEditor = ({}) => {
  console.log("Frame changed");

  const { currentChnot, setCurrentChnot, overwriteChnot } = useChnotStore();

  const [editState, setEditState] = useState<ChnotEditState>({
    isUploadingResource: false,
    isRequesting: false,
    isComposing: false,
  });

  const metaId = currentChnot ? currentChnot.meta.id : uuid();

  const saveContent = useCallback(
    async (content?: string) => {
      setEditState((state) => {
        return {
          ...state,
          isRequesting: true,
        };
      });

      if (content) {
        const req: ChnotOverwriteReq = {
          chnot: {
            id: uuid(),
            content: content,
            insert_time: new Date(),
            meta_id: metaId,
          },
          kind: "mdwt",
        };
        try {
          const rsp = await overwriteChnot(req, true);
          if (!currentChnot) {
            toast.success("Tie a Knot Successfully!");
          }
          if (currentChnot?.record.id !== rsp.chnot.record.id) {
            setCurrentChnot(rsp.chnot);
          }
        } finally {
          setEditState((state) => {
            return {
              ...state,
              isRequesting: false,
            };
          });
        }
      }
    },
    [setCurrentChnot, currentChnot, setEditState, metaId]
  );

  const timerRef = React.useRef<ReturnType<typeof setTimeout>>(undefined);
  const onChange = useCallback(
    (content: string) => {
      clearTimeout(timerRef.current);
      timerRef.current = setTimeout(() => {
        saveContent(content);
      }, 1000);
    },
    [saveContent]
  );
  const onChangeRef = useRef(onChange);

  return (
    <div className="w-full h-full">
      <div className="border w-full">{editState.isRequesting ? "R" : "S"}</div>
      <CMEditor metaId={metaId} onChange={onChangeRef} />
    </div>
  );
};
