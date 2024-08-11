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
import { DomainSelect } from "./DomainSelect";

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
        chnot: { ...chnot, id: uuid(), content: doc.toString() },
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
        <DomainSelect />
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
