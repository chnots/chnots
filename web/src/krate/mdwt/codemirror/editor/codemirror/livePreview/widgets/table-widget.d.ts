import { type EditorView, WidgetType } from '@codemirror/view';
export type ColumnAlign = 'left' | 'center' | 'right';
export declare function parseCells(line: string): string[];
export declare function parseAlignment(delimiterLine: string): ColumnAlign[];
export declare function isDelimiterRow(line: string): boolean;
export interface TableEditDetail {
    rawText: string;
    from: number;
    to: number;
}
export declare const TABLE_EDIT_EVENT = "cm-table-edit";
export declare class TableWidget extends WidgetType {
    private readonly rawText;
    private readonly from;
    private readonly to;
    constructor(rawText: string, from: number, to: number);
    eq(other: TableWidget): boolean;
    toDOM(_view: EditorView): HTMLDivElement;
    ignoreEvent(): boolean;
}
