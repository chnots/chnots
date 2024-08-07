import { Button } from "@mui/joy";
import { v4 as uuid } from "uuid";
import { useRef, useState } from "react";
import { Chnot, ChnotType } from "@/model";
import Icon from "../Icon";
import { overwriteChnot } from "@/helpers/data-agent";
import { ChnotViewMode, ChnotViewState } from ".";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";

import MarkdownPreview from "@uiw/react-markdown-preview";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export interface MarkdownChnotProps {
  viewMode: ChnotViewMode;

  className?: string;

  chnot?: Chnot;
}

const editorTheme = EditorView.theme({
  ".cm-line": {
    background: "transparent !important",
  },
  ".cm-focused": {
    outline: "none !important",
  },
  ".cm-editor": {
    outline: "none !important",
  },
});

const MarkdownChnotEditor = ({ chnot }: { chnot: Chnot }) => {
  const queryClient = useQueryClient();

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
      await overwriteChnot(req);
      setContent("");
    } finally {
      setState((state) => {
        return {
          ...state,
          isRequesting: false,
        };
      });
    }
  };

  const mutation = useMutation({
    mutationFn: handleSaveBtnClick,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["chnots"] });
    },
  });

  const handleSend = () => {
    mutation.mutate();
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
        placeholder={"Tie a knot"}
      />
      <div className="shrink-0 flex flex-row justify-end items-center">
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

export const MarkdownChnot = ({
  chnot: co,
  className,
  viewMode,
}: MarkdownChnotProps) => {
  const chnot = co ?? {
    id: uuid(),
    perm_id: uuid(),
    content: "",
    domain: "public",
    type: ChnotType.MarkdownWithToent,
    insert_time: new Date(),
    update_time: new Date(),
  };

  return viewMode === ChnotViewMode.Preview ? (
    <MarkdownPreview
      source={chnot.content}
      style={{
        width: "100%",
      }}
      rehypeRewrite={(node, index, parent) => {
        if (
          node.tagName === "a" &&
          parent &&
          /^h(1|2|3|4|5|6)/.test(parent.tagName)
        ) {
          parent.children = parent.children.slice(1);
        }
      }}
    />
  ) : (
    <div
      className={`${
        className ?? ""
      } relative w-full flex flex-col justify-start items-start bg-white dark:bg-zinc-800 p-4 rounded-lg border border-gray-200 dark:border-zinc-700`}
    >
      <MarkdownChnotEditor chnot={chnot} />
    </div>
  );
};
