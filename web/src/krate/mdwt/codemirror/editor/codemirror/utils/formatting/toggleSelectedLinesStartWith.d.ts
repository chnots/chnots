import { EditorState, TransactionSpec } from '@codemirror/state';
declare const toggleSelectedLinesStartWith: (state: EditorState, regex: RegExp, template: string, matchEmpty: boolean, lineContentStartRegex?: RegExp, nodeName?: string) => TransactionSpec;
export default toggleSelectedLinesStartWith;
