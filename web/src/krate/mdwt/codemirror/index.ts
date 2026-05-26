import decoratorExtension from "./decoratorExtension";
import insertLineAfter from "./commands/insertLineAfter";
import livePreview from "./livePreview";
import {
  toggleInlineFormat,
  toggleBold,
  toggleItalic,
  toggleStrikethrough,
  toggleInlineCode,
  toggleHighlight,
} from "./utils/formatting/toggleInlineFormat";
import toggleSelectedLinesStartWith from "./utils/formatting/toggleSelectedLinesStartWith";
import isCursorAtBeginning from "./utils/isCursorAtBeginning";
import intersectsSyntaxNode from "./utils/isInSyntaxNode";
import renumberSelectedLists from "./utils/renumberSelectedLists";

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

export { TABLE_EDIT_EVENT } from "./livePreview/widgets/table-widget";
export type {
  TableEditDetail,
  ColumnAlign,
} from "./livePreview/widgets/table-widget";

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
  Backlink,
  ChnotProps,
  todoHighlightPlugin,
} from "./mdwt/mdwt-extension";

export {
  headingBlocks,
  getBlockAtPos,
  getBlockByOtid,
  getAllBlocks,
  splitDocumentByBlocks,
  scrollToBlock,
  HEADING_OTID_RE,
  normalizeBlockContent,
} from "./mdwt/heading-block";
export type { HeadingBlock } from "./mdwt/heading-block";
