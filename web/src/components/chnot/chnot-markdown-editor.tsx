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
import Icon from "../icon";

enum RequestState {
  Saved,
  Requesting,
  Error,
}

interface ChnotEditState {
  isUploadingResource: boolean;
  requestState: RequestState;
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
  onChange: RefObject<(metaId: string, _content: string) => void>;
}> = React.memo(({ metaId, onChange }) => {
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
      const chnot = await queryChnot({ meta_id: metaId });
      setContent(chnot?.record.content);
    };
    fetchData();
  }, [setContent, metaId]);

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
        onChange={(e) => onChange.current(metaId, e)}
      />
    </div>
  );
});

export const ChnotMarkdownEditor = ({}) => {
  console.log("Frame changed");

  const { currentChnot, setCurrentChnot, overwriteChnot } = useChnotStore();

  const [editState, setEditState] = useState<ChnotEditState>({
    isUploadingResource: false,
    requestState: RequestState.Saved,
    isComposing: false,
  });

  const saveContent = useCallback(
    async (metaId: string, content?: string) => {
      setEditState((state) => {
        return {
          ...state,
          requestState: RequestState.Requesting,
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
        let requestState;
        try {
          const rsp = await overwriteChnot(req, true);
          setCurrentChnot(rsp.chnot);
          requestState = RequestState.Saved;
        } catch {
          requestState = RequestState.Error;
        }
        setEditState((state) => {
          return {
            ...state,
            requestState,
          };
        });
      }
    },
    [setCurrentChnot, currentChnot, setEditState, editState]
  );

  const timerRef = React.useRef<ReturnType<typeof setTimeout>>(undefined);
  const onChange = useCallback(
    (metaId: string, content: string) => {
      clearTimeout(timerRef.current);
      timerRef.current = setTimeout(() => {
        saveContent(metaId, content);
      }, 1000);
    },
    [saveContent]
  );
  const onChangeRef = useRef(onChange);

  return (
    <div className="w-full h-full">
      <div className="border w-full p-1 flex bg-gray-100 rounded-lg shadow-sm">
        <div className="">
          {editState.requestState === RequestState.Requesting ? (
            <div className="flex items-center text-blue-600 transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.Loader2 className="animate-spin h-5 w-5 mr-2" />
            </div>
          ) : editState.requestState === RequestState.Error ? (
            <div className="flex items-center text-red-600 transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.LucideMessageCircleQuestion className="h-5 w-5 mr-2" />
            </div>
          ) : (
            <div className="flex items-center text-green-600 transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.CheckCircle className="h-5 w-5 mr-2" />
            </div>
          )}
        </div>
      </div>
      <CMEditor
        metaId={currentChnot?.meta.id ?? uuid()}
        onChange={onChangeRef}
      />
    </div>
  );
};
