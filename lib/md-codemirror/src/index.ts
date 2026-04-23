import decoratorExtension from "./editor/codemirror/decoratorExtension";
import insertLineAfter from "./editor/codemirror/editorCommands/insertLineAfter";
import livePreview from "./editor/codemirror/livePreview";
import {
  toggleInlineFormat,
  toggleBold,
  toggleItalic,
  toggleStrikethrough,
  toggleInlineCode,
  toggleHighlight,
} from "./editor/codemirror/utils/formatting/toggleInlineFormat";
import toggleSelectedLinesStartWith from "./editor/codemirror/utils/formatting/toggleSelectedLinesStartWith";
import isCursorAtBeginning from "./editor/codemirror/utils/isCursorAtBeginning";
import intersectsSyntaxNode from "./editor/codemirror/utils/isInSyntaxNode";
import renumberSelectedLists from "./editor/codemirror/utils/renumberSelectedLists";

export {
  decoratorExtension,
  insertLineAfter,
  livePreview,
  toggleInlineFormat,
  toggleBold,
  toggleItalic,
  toggleStrikethrough,
  toggleInlineCode,
  toggleHighlight,
  toggleSelectedLinesStartWith,
  isCursorAtBeginning,
  intersectsSyntaxNode,
  renumberSelectedLists,
};

export { TABLE_EDIT_EVENT } from "./editor/codemirror/livePreview/widgets/table-widget";
export type {
  TableEditDetail,
  ColumnAlign,
} from "./editor/codemirror/livePreview/widgets/table-widget";

export {
  generateKeybinding,
  increaseIndent,
  insertOrIncreaseIndent,
  decreaseIndent,
} from "./keybinding";
export { MathConfig } from "./math-config";
export { Hashtag } from "./hashtag";
export { createCodemirrorTheme } from "./theme";
export { parseMarkdownTable, generateMarkdownTable } from "./table-utils";
export type { ParsedTable } from "./table-utils";

export {
  headingBlocks,
  getBlockAtPos,
  getBlockByOtid,
  getAllBlocks,
  splitDocumentByBlocks,
  scrollToBlock,
  tidCompletion,
  HEADING_OTID_RE,
  normalizeBlockContent,
} from "./heading-block";
export type { HeadingBlock } from "./heading-block";
