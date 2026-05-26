import { type EditorView, WidgetType } from '@codemirror/view';
export declare class HeadingWidget extends WidgetType {
    private readonly level;
    private readonly text;
    constructor(level: number, text: string);
    eq(other: HeadingWidget): boolean;
    toDOM(_view: EditorView): HTMLDivElement;
    ignoreEvent(): boolean;
}
