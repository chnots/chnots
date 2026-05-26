import { EditorState, SelectionRange } from '@codemirror/state';
declare const growSelectionToNode: (state: EditorState, sel: SelectionRange, nodeNames: string | string[] | null) => SelectionRange;
export default growSelectionToNode;
