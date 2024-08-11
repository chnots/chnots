import { Button } from "@mui/joy";
import { useRef, useState } from "react";
import Icon from "../Icon";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";

import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";

import { Chnot, ChnotComment } from "@/store/v1/chnot";

import MarkdownPreview from "@uiw/react-markdown-preview";

const editorTheme = EditorView.theme({
  // To Remove outline when focused, https://github.com/uiwjs/react-codemirror/issues/643
  "&.cm-editor.cm-focused": {
    outline: "none",
  },
  ".cm-line": {
    background: "transparent !important",
  },
});

export const ChnotCommentEditor = ({
  content: _content,
  handleSendCallback,
}: {
  content: string;
  handleSendCallback: (content: string) => void;
}) => {
  const [content, setContent] = useState(_content);

  const codeMirror = useRef<ReactCodeMirrorRef>(null);

  const extensions = [
    markdown({ base: markdownLanguage, codeLanguages: languages }),
    EditorView.lineWrapping,
    editorTheme,
  ];

  const handleSend = () => {
    handleSendCallback(content);
  };

  return (
    <div className="w-full flex flex-row text-xs">
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
        placeholder={"Reply to this chnot."}
      />

      <div className="shrink-0 flex flex-row juchnotsstify-end items-center">
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

export const ChnotCommentViewer = ({ comment }: { comment: ChnotComment }) => {
  return (
    <div
      className={`w-100% flex flex-row justify-start items-start text-gray-800 dark:text-gray-400 p-3`}
    >
      <span className="w-1/4 text-gray-300 px-2 text-sm">
        {comment.insert_time}
      </span>
      <div className="w-3/4">
        <MarkdownPreview
          source={comment.content}
          style={{
            width: "100%",
            fontSize: "0.9em",
          }}
          className="border-t"
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
