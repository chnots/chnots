import { type EditorView, WidgetType } from '@codemirror/view';
export declare class InlineMathWidget extends WidgetType {
    private readonly latex;
    private readonly displayMode;
    constructor(latex: string, displayMode?: boolean);
    eq(other: InlineMathWidget): boolean;
    toDOM(_view: EditorView): HTMLSpanElement;
    ignoreEvent(): boolean;
}
export declare class BlockMathWidget extends WidgetType {
    private readonly latex;
    constructor(latex: string);
    eq(other: BlockMathWidget): boolean;
    toDOM(_view: EditorView): HTMLDivElement;
    ignoreEvent(): boolean;
}
