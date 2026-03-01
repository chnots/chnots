import { EditorView } from "@codemirror/view";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { forwardRef } from "react";
import { cn } from "@/lib/utils";

type TextEditorProps = {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  className?: string;
  showLineNumbers?: boolean;
};

export const TextEditor = forwardRef<ReactCodeMirrorRef, TextEditorProps>(
  (
    {
      value,
      onChange,
      placeholder = "Enter your text...",
      className,
      showLineNumbers = true,
    },
    ref,
  ) => {
    return (
      <>
        <CodeMirror
          value={value}
          extensions={[EditorView.lineWrapping]}
          onChange={onChange}
          ref={ref}
          basicSetup={{
            lineNumbers: showLineNumbers,
            highlightActiveLineGutter: showLineNumbers,
            foldGutter: true,
            closeBrackets: true,
          }}
          placeholder={placeholder}
          className={cn("flex-shrink-0", className)}
        />
        <div
          role="none"
          className="flex-grow cursor-text min-h-0 p-0 m-0"
          onClick={() => {
            if (ref && "current" in ref && ref?.current) {
              const editorView = ref.current.view;
              if (editorView) {
                const docLength = editorView.state.doc.length;
                editorView.dispatch({
                  selection: { anchor: docLength },
                  scrollIntoView: true,
                });
                editorView.focus();
              }
            }
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
            }
          }}
        />
      </>
    );
  },
);

TextEditor.displayName = "TextEditor";
