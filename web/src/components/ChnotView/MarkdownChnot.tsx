import { Button, Divider } from "@mui/joy";
import { v4 as uuid } from "uuid";
import { useRef, useState } from "react";
import { Chnot, ChnotType } from "@/model";
import Icon from "../Icon";
import { ChnotViewState } from ".";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";

import MarkdownPreview from "@uiw/react-markdown-preview";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
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
  chnot: co,
  unique,
}: {
  chnot: Chnot;
  unique?: boolean;
}) => {
  const domainStore = useDomainStore();
  const chnotStore = useChnotStore();

  const chnot: Chnot =
    unique || !co
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
      : co;

  const [content, setContent] = useState(chnot.content);

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

  const handleSaveBtnClick = async () => {
    if (state.isRequesting) {
      return;
    }

    setState((state) => {
      return {
        ...state,
        isRequesting: true,
      };
    });

    const req = {
      chnot: { ...chnot, id: uuid(), content: content ? content : "" },
    };
    try {
      await chnotStore.overwriteChnot(req);
      setContent("");
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
  };

  const handleSend = () => {
    handleSaveBtnClick();
  };

  return (
    <div className="w-full">
      <CodeMirror
        className={`chnot-md-inner`}
        extensions={extensions}
        ref={codeMirror}
        onChange={(c) => {
          setContent(c);
        }}
        style={{
          font: "serif",
        }}
        value={content}
        basicSetup={{
          lineNumbers: false,
          highlightActiveLineGutter: false,
          foldGutter: false,
        }}
        placeholder={"Tie a Knot"}
      />
      <Divider className="!mt-2 !mb-2" />

      <div className="shrink-0 flex flex-row justify-end items-center">
        <DomainSelect />
        <Button
          className="!font-normal"
          endDecorator={<Icon.Send className="w-4 h-auto" />}
          disabled={!content}
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
