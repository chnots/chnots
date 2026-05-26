import { type EditorView, WidgetType } from '@codemirror/view';
export declare class ImageWidget extends WidgetType {
    private readonly url;
    private readonly alt;
    constructor(url: string, alt: string);
    eq(other: ImageWidget): boolean;
    toDOM(_view: EditorView): HTMLDivElement;
    ignoreEvent(): boolean;
}
export declare function parseImage(text: string): {
    alt: string;
    url: string;
} | null;
