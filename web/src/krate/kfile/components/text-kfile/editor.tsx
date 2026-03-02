import { EditorView } from "@codemirror/view";
import useResizeObserver from "@react-hook/resize-observer";
import CodeMirror, { type ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { forwardRef, useLayoutEffect, useRef, useState } from "react";
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
    const containerRef = useRef<HTMLDivElement>(null);
    const [height, setHeight] = useState<number | undefined>(undefined);

    useLayoutEffect(() => {
      if (containerRef.current) {
        setHeight(containerRef.current.clientHeight);
      }
    }, []);

    useResizeObserver(containerRef, (entry) => {
      setHeight(entry.contentRect.height);
    });

    return (
      <div ref={containerRef} className={cn("h-full", className)}>
        {height !== undefined && (
          <CodeMirror
            value={value}
            height={`${height}px`}
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
          />
        )}
      </div>
    );
  },
);

TextEditor.displayName = "TextEditor";
