export interface HeadingBlock {
  otid: number;
  from: number;
  to: number;
  level: number;
  headingContent: string;
}

export const HEADING_OTID_RE = /^(\s{0,3}#{1,6}\s+)\[\[(\d{13,16})\]\]\s*(.*)/;
