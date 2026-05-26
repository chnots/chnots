import { type EditorView, WidgetType } from '@codemirror/view';
export declare class HorizontalRuleWidget extends WidgetType {
    constructor();
    eq(_other: HorizontalRuleWidget): boolean;
    toDOM(_view: EditorView): HTMLHRElement;
    ignoreEvent(): boolean;
}
export declare class CheckboxWidget extends WidgetType {
    private readonly checked;
    constructor(checked: boolean);
    eq(other: CheckboxWidget): boolean;
    toDOM(): HTMLInputElement;
    ignoreEvent(): boolean;
}
