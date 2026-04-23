import type { Extension, EditorState } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";
import { headingBlockField } from "./block-field";
import { blockDecorationField } from "./block-decorations";
import { parseHeadings } from "./block-parser";
import type { HeadingBlock } from "./block-model";

export type { HeadingBlock } from "./block-model";
export { HEADING_OTID_RE } from "./block-model";
export { tidCompletion } from "./block-completion";

export function headingBlocks(): Extension {
  return [headingBlockField, blockDecorationField];
}

export function getBlockAtPos(
  state: EditorState,
  pos: number,
): HeadingBlock | null {
  const blocks = state.field(headingBlockField, false) ?? [];
  for (const block of blocks) {
    if (block.from <= pos && pos <= block.to) return block;
  }
  return null;
}

export function getBlockByOtid(
  state: EditorState,
  otid: number,
): HeadingBlock | null {
  const blocks = state.field(headingBlockField, false) ?? [];
  return blocks.find((b) => b.otid === otid) ?? null;
}

export function getAllBlocks(state: EditorState): HeadingBlock[] {
  return state.field(headingBlockField, false) ?? [];
}

export function splitDocumentByBlocks(
  state: EditorState,
): Map<number, string> {
  const blocks = state.field(headingBlockField, false) ?? [];
  const result = new Map<number, string>();
  for (const block of blocks) {
    const content = state.sliceDoc(block.from, block.to).trimEnd();
    result.set(block.otid, content);
  }
  return result;
}

export function scrollToBlock(view: EditorView, otid: number): void {
  const blocks = view.state.field(headingBlockField, false) ?? [];
  const block = blocks.find((b) => b.otid === otid);
  if (!block) return;
  view.dispatch({
    selection: { anchor: block.from },
    scrollIntoView: true,
  });
}
