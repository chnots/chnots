import { EditorState, TransactionSpec } from '@codemirror/state';
export declare const toggleInlineFormat: (state: EditorState, formatName: string) => TransactionSpec;
export declare const toggleBold: (state: EditorState) => TransactionSpec;
export declare const toggleItalic: (state: EditorState) => TransactionSpec;
export declare const toggleStrikethrough: (state: EditorState) => TransactionSpec;
export declare const toggleInlineCode: (state: EditorState) => TransactionSpec;
export declare const toggleHighlight: (state: EditorState) => TransactionSpec;
export default toggleInlineFormat;
