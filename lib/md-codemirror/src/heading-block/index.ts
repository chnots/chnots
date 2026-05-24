import type { Extension, EditorState } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";
import { headingBlockField } from "./block-field";
import { blockDecorationField } from "./block-decorations";
import { otidInjector } from "./otid-inject";
import type { HeadingBlock } from "./block-model";

export type { HeadingBlock } from "./block-model";
export { HEADING_OTID_RE, normalizeBlockContent } from "./block-model";

export function headingBlocks(config: { genTID: () => number }): Extension {
  return [otidInjector(config.genTID), headingBlockField, blockDecorationField];
}

export function getBlockAtPos(
  state: EditorState,
  pos: number,
): HeadingBlock | null {
  const blocks = state.field(headingBlockField, false) ?? [];
  let lo = 0;
  let hi = blocks.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >>> 1;
    const block = blocks[mid];
    if (pos < block.from) {
      hi = mid - 1;
    } else if (pos > block.to) {
      lo = mid + 1;
    } else {
      return block;
    }
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
