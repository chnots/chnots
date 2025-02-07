import { v4 as uuid } from "uuid";
import { useRef, useState } from "react";
import { ChnotRecord } from "@/store/chnot";
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

// const imageRE = /\.(?:png|jpe?g|gif|bmp|svg|tiff?)$/i;

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

export const ChnotMarkdownEditor = ({}) => {
  console.log("ChnotMarkdownEditor changed");
  const boxRef = React.useRef<HTMLDivElement>(null);

  const chnotStore = useChnotStore();
  const currentChnot = chnotStore.getCurrentChnot();
  const [editState, setEditState] = useState<ChnotEditState>({
    isUploadingResource: false,
    isRequesting: false,
    isComposing: false,
  });
  const codeMirror = useRef<ReactCodeMirrorRef>(null);

  const chnotRecord: ChnotRecord = currentChnot
    ? currentChnot.record
    : {
      id: uuid(),
      meta_id: uuid(),
      content: "",
      insert_time: new Date(),
    };

  const extensions = [
    markdown({ base: markdownLanguage, codeLanguages: languages }),
    EditorView.lineWrapping,
    editorTheme,
    eventHandlers,
  ];


  const handleSend = async (content?: string) => {
    setEditState((state) => {
      return {
        ...state,
        isRequesting: true,
      };
    });

    if (content) {
      const req = {
        chnot: {
          ...chnotRecord,
          id: uuid(),
          content: content,
          insert_time: new Date(),
        },
        kind: "mdwt",
      };
      try {
        const rsp = await chnotStore.overwriteChnot(req, true);
        if (!currentChnot) {
          toast.success("Tie a Knot Successfully!");
        }
        if (currentChnot?.record.id !== rsp.chnot.record.id) {
          chnotStore.setCurrentChnot(rsp.chnot);
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
  };

  const timerRef = React.useRef<ReturnType<typeof setTimeout>>(undefined);

  const handleChange = (content: string) => {
    clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => {
      handleSend(content);
    }, 1000);
  };

  return (
    <div className="w-full h-full">
      <div className="border w-full">
        {editState.isRequesting ? "Requesting" : "saved"}
      </div>
      <div className="w-full h-full" ref={boxRef}>
        <CodeMirror
          height={`${boxRef.current?.getBoundingClientRect().height ?? 0}px`}
          extensions={extensions}
          ref={codeMirror}
          style={{
            font: "serif",
          }}
          value={chnotRecord.content}
          basicSetup={{
            lineNumbers: false,
            highlightActiveLineGutter: false,
            foldGutter: false,
          }}
          placeholder={"Input something."}
          onChange={(e) => handleChange(e)}
        />
      </div>
    </div>
  );
};
