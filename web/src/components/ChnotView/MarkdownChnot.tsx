import { Button, Divider } from "@mui/joy";
import { v4 as uuid } from "uuid";
import { useRef, useState } from "react";
import { Chnot, ChnotType } from "@/store/v1/chnot";
import Icon from "../Icon";
import { ChnotViewState } from ".";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";
import MarkdownPreview from "@uiw/react-markdown-preview";
import CodeMirror, {
  EditorState,
  type ReactCodeMirrorRef,
} from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { useDomainStore } from "@/store/v1/domain";
import { useChnotStore } from "@/store/v1/chnot";
import { toast } from "sonner";
import { html2mdAsync } from "@/utils/markdown-utils";

const imageRE = /\.(?:png|jpe?g|gif|bmp|svg|tiff?)$/i;

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

    // Now that we have proper clipboardData to access, we have to determine if
    // this is an image or a text call. Here, we have a set of problems.
    //
    // 1. OS images (and other files) will be represented as file items, so
    //    data.types only includes "Files"
    // 2. Firefox and Chrome will, if the user copies an image, have both the
    //    type "Files" and "text/html", the latter of which often includes the
    //    URL or the image data as a string.
    // 3. Microsoft Office is a POS and will just write EVERYTHING to the
    //    clipboard, i.e. "text/plain", "text/html", "text/rtf", and "image/png"
    // 4. LibreOffice will also write plain, HTML, and RTF, but no image.
    //
    // In effect, we cannot rely on the presence of a "Files" type in the
    // clipboard data to tell us whether we should initiate a paste image or
    // paste text action.
    //
    // BUT, what I found out is that whenever the intention is to paste text,
    // and an image only serves as a fallback, there will be "text/plain" in the
    // clipboard. In other words, as long as there is "text/plain" in the
    // clipboard, the user intends to paste text, not an image.
    const textIntention = data.types.includes("text/plain");

    const insertions: string[] = [];
    const allPromises: Array<Promise<void>> = [];
    if (textIntention && data.types.includes("text/html")) {
      // The user intends to paste text, and there is formatted HTML in the
      // clipboard that we need to turn into HTML.
      const html = data.getData("text/html");
      console.log("Converting from HTML ...");
      const promise = html2mdAsync(html)
        .then((md) => {
          console.log("Done!");

          insertions.push(md);
        })

        .catch((err) => {
          console.error(err);
          // On error, fall back to the plain text
          insertions.push(data.getData("text/plain"));
        });

      allPromises.push(promise);env = ELECTRON_OZONE_PLATFORM_HINT,auto
    } else if (textIntention) {
      // The user intends to paste text, but there's only plain text in the
      // clipboard.
      const text = data.getData("text/plain");
      insertions.push(text);
    } else {
      // The user intends to paste an image or a series of files

      for (const file of data.files) {
        if (imageRE.test(file.name)) {
          if (file.path === "") {
            // This image resides only within the clipboard, so prompt the user
            // to save it down. The command will already wrap everything into
            // `![]()`.
            allPromises.push(
              new Promise((resolve, reject) => {
                saveImageFromClipboard(basePath)
                  .then((tag) => {
                    if (tag !== undefined) {
                      insertions.push(tag);
                    }
                    resolve();
                  })
                  .catch((err) => reject(err));
              })
            );
          } else {
            // There is a path in the file item
            insertions.push(
              `![${file.name}](${normalizePathForInsertion(
                file.path,
                basePath
              )})`
            );
          }
        } else {
          // Not an image, so simply link it.
          insertions.push(
            `[${file.name}](${normalizePathForInsertion(file.path, basePath)})`
          );
        }
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

export const MarkdownChnotViewer = ({ chnot }: { chnot: Chnot }) => {
  return (
    <div
      className={`w-100% flex flex-col justify-start items-start text-gray-800 dark:text-gray-400 p-3`}
    >
      <MarkdownPreview
        source={chnot.content}
        style={{
          width: "100%",
        }}
        rehypeRewrite={(node, _index, parent) => {
          if (
            parent &&
            "tagName" in node &&
            "tagName" in parent &&
            node.tagName === "a" &&
            /^h(1|2|3|4|5|6)/.test(parent.tagName)
          ) {
            parent.children = parent.children.slice(1);
          }
        }}
      />
    </div>
  );
};
