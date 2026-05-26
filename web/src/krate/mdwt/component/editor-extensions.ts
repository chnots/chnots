import type { Extension } from "@codemirror/state";
import {
  createCodemirrorTheme,
  Hashtag,
  headingBlocks,
  livePreview,
  MathConfig,
} from "../codemirror";
import {
  autocompletion,
  type CompletionContext,
  type CompletionResult,
} from "@codemirror/autocomplete";
import { EditorState } from "@codemirror/state";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { indentOnInput } from "@codemirror/language";
import { languages } from "@codemirror/language-data";
import { EditorView } from "@codemirror/view";
import { GFM } from "@lezer/markdown";
import { wrappedLineIndent } from "codemirror-wrapped-line-indent";
import { generateKeybinding } from "../codemirror";
import { genTID } from "@/lib/id_util";
import { Backlink, ChnotProps, todoHighlightPlugin } from "../codemirror/mdwt-extension";
import { chnotCompletions } from "./chnot-completions";
import { eventHandlers } from "./paste-handler";

export function buildEditorExtensions(config: {
  onCtrlEnter?: (view: EditorView) => boolean;
  readonly?: boolean;
  headingSource:
    | ((context: CompletionContext) => Promise<CompletionResult | null> | null)
    | null;
  extraExtensions?: Extension[];
}): Extension[] {
  const markdownExtension = markdown({
    base: markdownLanguage,
    codeLanguages: languages,
    addKeymap: true,
    completeHTMLTags: false,
    extensions: [Backlink, Hashtag, ChnotProps, GFM, MathConfig],
  });

  return [
    markdownExtension,
    generateKeybinding(config.onCtrlEnter),

    EditorView.lineWrapping,
    wrappedLineIndent,

    todoHighlightPlugin,

    livePreview(),
    headingBlocks({ genTID }),
    createCodemirrorTheme(),

    eventHandlers,

    indentOnInput(),
    autocompletion({
      override: [
        ...(config.headingSource ? [config.headingSource] : []),
        chnotCompletions,
      ],
    }),
    ...(config.readonly ? [EditorState.readOnly.of(true)] : []),
    EditorView.theme({
      "&": {
        height: "100%",
      },
      "&.cm-editor": {
        height: "100%",
      },
      ".cm-scroller": {
        height: "100%",
        overflow: "auto",
        justifyContent: "center",
      },
      ".cm-content": {
        minHeight: "100%",
        maxWidth: "56rem",
        width: "100%",
        flex: "none",
      },
    }),
    ...(config.extraExtensions ?? []),
  ];
}
