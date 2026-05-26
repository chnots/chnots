export interface HeadingBlock {
    otid: number;
    from: number;
    to: number;
    level: number;
    headingContent: string;
}
export declare const HEADING_OTID_RE: RegExp;
export declare function normalizeBlockContent(otid: number, content: string): string;
