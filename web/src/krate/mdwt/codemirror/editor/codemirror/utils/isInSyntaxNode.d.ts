import { EditorState } from '@codemirror/state';
interface Range {
    from: number;
    to: number;
}
declare const intersectsSyntaxNode: (state: EditorState, range: Range, nodeName: string) => boolean;
export default intersectsSyntaxNode;
