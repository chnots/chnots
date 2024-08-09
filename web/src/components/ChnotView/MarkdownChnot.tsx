import { Button, Divider } from "@mui/joy";
import { v4 as uuid } from "uuid";
import { useRef, useState } from "react";
import { Chnot, ChnotType } from "@/model";
import Icon from "../Icon";
import { ChnotViewMode, ChnotViewState, DomainSelect } from ".";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";

import MarkdownPreview from "@uiw/react-markdown-preview";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { useDomainStore } from "@/store/v1/domain";
import { useChnotStore } from "@/store/v1/chnot";
import { toast } from "sonner";

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
  const domainStore = useDomainStore();
  const chnotStore = useChnotStore();

  const chnot =
    unique || !co
      ? {
          id: uuid(),
          perm_id: uuid(),
          content: "",
          domain: domainStore.current.name,
          type: ChnotType.MarkdownWithToent,
          insert_time: new Date(),
          update_time: new Date(),
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

const MarkdownChnotViewer = ({ chnot }: { chnot: Chnot }) => {
  const update_time = new Date(chnot.update_time);
  const relativeTimeFormat =
    Date.now() - update_time.getTime() > 7 * 1000 * 60 * 60 * 24
      ? "datetime"
      : "relative";

  return (
    <div className="w-full">
      <div className="w-full -mt-0.5 text-xs flex flex-row leading-tight text-gray-400 dark:text-gray-500 select-none">
        <relative-time
          datetime={update_time.toISOString()}
          format={relativeTimeFormat}
          tense="past"
        ></relative-time>

        <div className="text-xs ml-1 px-1 text-red-500 italic">
          {chnot.domain}
        </div>
      </div>
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
    <div className="group relative flex flex-col justify-start items-start w-full px-4 py-3 mb-2 gap-2 bg-white dark:bg-zinc-800 rounded-lg border border-white dark:border-zinc-800 hover:border-gray-200 dark:hover:border-zinc-700">
      <MarkdownChnotViewer chnot={chnot} />
    </div>
  ) : (
    <div
      className={`${
        className ?? ""
      } relative w-full flex flex-col justify-start items-start bg-white dark:bg-zinc-800 p-4 rounded-lg border border-gray-200 dark:border-zinc-700`}
    >
      <MarkdownChnotEditor chnot={chnot} unique={true} />
    </div>
  );
};
