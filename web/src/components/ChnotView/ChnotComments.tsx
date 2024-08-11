import { useRef } from "react";
import Icon from "../Icon";
import { EditorView } from "@codemirror/view";
import { languages } from "@codemirror/language-data";

import CodeMirror, {
  EditorState,
  type ReactCodeMirrorRef,
} from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";

import { ChnotComment } from "@/store/v1/chnot";

import MarkdownPreview from "@uiw/react-markdown-preview";
import { chnotShortDate as chnotShortDate } from "@/utils/date-formater";

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
  content,
  handleSendCallback,
}: {
  content: string;
  handleSendCallback: (content: string) => void;
}) => {
  console.log("=================== content reset");
  const codeMirror = useRef<ReactCodeMirrorRef>(null);

  const extensions = [
    markdown({ base: markdownLanguage, codeLanguages: languages }),
    EditorView.lineWrapping,
    editorTheme,
  ];

  const handleSend = () => {
    const doc = codeMirror.current?.view?.state.doc;
    if (doc) {
      handleSendCallback(doc.toString());
      const state = EditorState.create({
        ...codeMirror.current?.state,
        doc: "",
        extensions,
      });
      codeMirror.current.view?.setState(state);
    }
  };

  return (
    <>
      <CodeMirror
        className="chnot-md-inner"
        extensions={extensions}
        ref={codeMirror}
        style={{
          flexGrow: "1",
        }}
        value={content}
        basicSetup={{
          lineNumbers: false,
          highlightActiveLineGutter: false,
          foldGutter: false,
        }}
        placeholder={"Reply to this chnot."}
      />
      <button onClick={handleSend} className="text-xs text-gray-400 mr-3">
        <Icon.Check />
      </button>
    </>
  );
};

export const ChnotCommentViewer = ({ comment }: { comment: ChnotComment }) => {
  return (
    <div
      className={`w-full flex flex-row text-wrap break-words justify-start items-start text-gray-400 border-t py-2`}
    >
      <div>
        <Icon.MessageCircleIcon width="24px" className="w-4" />
      </div>
      <span className="px-2 text-sm">
        {chnotShortDate(comment.insert_time)}
      </span>
      <MarkdownPreview
        source={comment.content}
        style={{
          flexGrow: "1",
          fontSize: "0.875em",
          // @ts-ignore
          wordWrap: "anywhere",
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
