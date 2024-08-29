import { Button, Divider } from "@mui/joy";
import { v4 as uuid } from "uuid";
import { useRef, useState } from "react";
import { Chnot, ChnotType } from "@/store/v1/chnot";
import Icon from "../Icon";
import { ChnotViewState } from "../ChnotView";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";
import CodeMirror, {
  EditorState,
  type ReactCodeMirrorRef,
} from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { useDomainStore } from "@/store/v1/domain";
import { useChnotStore } from "@/store/v1/chnot";
import { toast } from "sonner";
import { html2mdAsync } from "@/utils/markdown-utils";
import { useAttachmentStore } from "@/store/v1/attachment";

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
                  insertions.push(`{{ KMGC/RES/${resource.id} }}`);
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
});

export const MarkdownChnotEditor = ({
  chnot: chnotIn,
  createInput,
}: {
  chnot: Chnot;
  createInput: boolean;
}) => {
  const domainStore = useDomainStore();
  const chnotStore = useChnotStore();

  const chnot: Chnot =
    createInput || !chnotIn
      ? {
          id: uuid(),
          perm_id: uuid(),
          content: "",
          domain: domainStore.current.name,
          type: ChnotType.MarkdownWithToent,
          insert_time: new Date(),
          update_time: new Date(),
          archive_time: undefined,
          pinned: false,
        }
      : chnotIn;

  const codeMirror = useRef<ReactCodeMirrorRef>(null);

  const extensions = [
    markdown({ base: markdownLanguage, codeLanguages: languages }),
    EditorView.lineWrapping,
    editorTheme,
    eventHandlers,
  ];

  const [state, setState] = useState<ChnotViewState>({
    isUploadingResource: false,
    isRequesting: false,
    isComposing: false,
  });

  const handleSend = async () => {
    if (state.isRequesting) {
      return;
    }

    setState((state) => {
      return {
        ...state,
        isRequesting: true,
      };
    });

    const doc = codeMirror.current?.view?.state.doc;
    if (doc) {
      const req = {
        chnot: {
          ...chnot,
          id: uuid(),
          content: doc.toString(),
          domain: domainStore.current.name,
        },
      };
      try {
        await chnotStore.overwriteChnot(req);
        toast.success("Tie a Knot Successfully!");
        chnotStore.refreshChnots();
      } finally {
        setState((state) => {
          return {
            ...state,
            isRequesting: false,
          };
        });
      }
      const state = EditorState.create({
        ...codeMirror.current?.state,
        doc: "",
        extensions,
      });
      codeMirror.current.view?.setState(state);
    }
  };

  return (
    <div className="w-full">
      <CodeMirror
        className={`chnot-md-inner`}
        extensions={extensions}
        ref={codeMirror}
        style={{
          font: "serif",
        }}
        value={chnot.content}
        basicSetup={{
          lineNumbers: false,
          highlightActiveLineGutter: false,
          foldGutter: false,
        }}
        placeholder={"Tie a Knot"}
      />
      <Divider className="!mt-2 !mb-2" />

      <div className="shrink-0 flex flex-row juchnotsstify-end items-center">
        <Button
          className="!font-normal"
          endDecorator={<Icon.Send className="w-4 h-auto" />}
          onClick={handleSend}
        >
          Save
        </Button>
      </div>
    </div>
  );
};
