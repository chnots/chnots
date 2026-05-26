import type { Extension, EditorState } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";
import type { HeadingBlock } from "./block-model";
export type { HeadingBlock } from "./block-model";
export { HEADING_OTID_RE, normalizeBlockContent } from "./block-model";
export declare function headingBlocks(config: {
    genTID: () => number;
}): Extension;
export declare function getBlockAtPos(state: EditorState, pos: number): HeadingBlock | null;
export declare function getBlockByOtid(state: EditorState, otid: number): HeadingBlock | null;
export declare function getAllBlocks(state: EditorState): HeadingBlock[];
export declare function splitDocumentByBlocks(state: EditorState): Map<number, string>;
export declare function scrollToBlock(view: EditorView, otid: number): void;
