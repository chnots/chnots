import type { TableEditDetail } from "../codemirror";
import { TABLE_EDIT_EVENT } from "../codemirror";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import {
  headingChnotCompletion,
  type HeadingCompletionConfig,
} from "../codemirror/mdwt/heading-chnot-completion";
import { buildEditorExtensions } from "./editor-extensions";
import { TableEditorDialog } from "./table-editor";
import "./table-editor/table-editor.css";
import "katex/dist/katex.min.css";

export type EditorCustomization = {
  onCtrlEnter?: (view: import("@codemirror/view").EditorView) => boolean;
  autoFocus?: boolean;
};

export const EditorCustomContext = React.createContext<
  EditorCustomization | undefined
>(undefined);

const MdwtEditor = ({
  content,
  foldGutter,
  onContentChange,
  placeholder,
  setCodeMirrorRef: setCMRef,
  extraExtensions,
  headingCompletionConfig,
  readonly,
}: {
  content?: string;
  foldGutter: boolean;
  placeholder?: string;
  onContentChange: (content: string) => void;
  setCodeMirrorRef?: (ref: React.RefObject<ReactCodeMirrorRef | null>) => void;
  extraExtensions?: import("@codemirror/state").Extension[];
  headingCompletionConfig?: HeadingCompletionConfig;
  readonly?: boolean;
}) => {
  const codeMirror = useRef<ReactCodeMirrorRef>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const editorCustom = React.useContext(EditorCustomContext);

  const headingConfigRef = useRef<HeadingCompletionConfig | undefined>(
    headingCompletionConfig,
  );
  headingConfigRef.current = headingCompletionConfig;

  const headingSource = useMemo(
    () =>
      headingConfigRef.current
        ? headingChnotCompletion(() => headingConfigRef.current!)
        : null,
    [],
  );

  const [tableEditOpen, setTableEditOpen] = useState(false);
  const [tableEditRawText, setTableEditRawText] = useState("");
  const [tableEditFrom, setTableEditFrom] = useState(0);
  const [tableEditTo, setTableEditTo] = useState(0);

  const handleTableEdit = useCallback((e: Event) => {
    const detail = (e as CustomEvent<TableEditDetail>).detail;
    setTableEditRawText(detail.rawText);
    setTableEditFrom(detail.from);
    setTableEditTo(detail.to);
    setTableEditOpen(true);
  }, []);

  const handleTableSave = useCallback(
    (newText: string) => {
      const view = codeMirror.current?.view;
      if (!view) return;
      view.dispatch({
        changes: { from: tableEditFrom, to: tableEditTo, insert: newText },
      });
    },
    [tableEditFrom, tableEditTo],
  );

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    container.addEventListener(TABLE_EDIT_EVENT, handleTableEdit);
    return () => {
      container.removeEventListener(TABLE_EDIT_EVENT, handleTableEdit);
    };
  }, [handleTableEdit]);

  useEffect(() => {
    if (setCMRef) {
      setCMRef(codeMirror);
    }
  }, [setCMRef]);

  useEffect(() => {
    if (editorCustom?.autoFocus) {
      const timer = setTimeout(() => {
        const view = codeMirror.current?.view;
        if (view) {
          const end = view.state.doc.length;
          view.dispatch({
            selection: { anchor: end },
          });
          view.focus();
        }
      }, 50);
      return () => clearTimeout(timer);
    }
  }, [editorCustom?.autoFocus]);

  const extensions = buildEditorExtensions({
    onCtrlEnter: editorCustom?.onCtrlEnter,
    readonly,
    headingSource,
    extraExtensions,
  });

  return (
    <div ref={containerRef} style={{ height: "100%" }}>
      <CodeMirror
        height="100%"
        extensions={extensions}
        ref={codeMirror}
        style={{
          font: "sans-serif",
          height: "100%",
        }}
        value={content}
        basicSetup={{
          lineNumbers: false,
          highlightActiveLineGutter: false,
          foldGutter: foldGutter,
          closeBrackets: false,
        }}
        placeholder={placeholder ?? "Take a chnot"}
        onChange={(e) => onContentChange(e)}
      />
      <TableEditorDialog
        open={tableEditOpen}
        rawText={tableEditRawText}
        onOpenChange={setTableEditOpen}
        onSave={handleTableSave}
      />
    </div>
  );
};

export const MdwtEditorMemo = React.memo(MdwtEditor);

export default MdwtEditor;
