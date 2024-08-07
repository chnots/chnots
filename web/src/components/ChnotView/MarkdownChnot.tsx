import { Button, Divider } from "@mui/joy";
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
  // To Remove outline when focused, https://github.com/uiwjs/react-codemirror/issues/643
  "&.cm-editor.cm-focused": {
    outline: "none",
  },
  ".cm-line": {
    background: "transparent !important",
  },
});

const MarkdownChnotEditor = ({
  chnot: co,
  unique,
}: {
  chnot: Chnot;
  unique?: boolean;
}) => {
  const chnot =
    unique || !co
      ? {
          id: uuid(),
          perm_id: uuid(),
          content: "",
          domain: "public",
          type: ChnotType.MarkdownWithToent,
          insert_time: new Date(),
          update_time: new Date(),
        }
      : co;
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
        placeholder={"Tie a Knot"}
      />
      <Divider className="!mt-2 !mb-2" />

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

const MarkdownChnotViewer = ({ chnot }: { chnot: Chnot }) => {
  const update_time = new Date(chnot.update_time);
  const relativeTimeFormat =
    Date.now() - update_time.getTime() > 1000 * 60 * 60 * 24
      ? "datetime"
      : "auto";

  return (
    <>
      <div className="w-100% -mt-0.5 text-xs leading-tight text-gray-400 dark:text-gray-500 select-none">
        <relative-time
          datetime={update_time.toISOString()}
          format={relativeTimeFormat}
          tense="past"
        ></relative-time>
      </div>
      <div
        className={`w-100% flex flex-col justify-start items-start text-gray-800 dark:text-gray-400`}
      >
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
      </div>
    </>
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

  return (
    <div className="w-full">
      {viewMode === ChnotViewMode.Preview ? (
        <MarkdownChnotViewer chnot={chnot} />
      ) : (
        <div
          className={`${
            className ?? ""
          } relative w-full flex flex-col justify-start items-start bg-white dark:bg-zinc-800 p-4 rounded-lg border border-gray-200 dark:border-zinc-700`}
        >
          <MarkdownChnotEditor chnot={chnot} unique={true} />
        </div>
      )}
    </div>
  );
};
