export interface HeadingBlock {
  otid: number;
  from: number;
  to: number;
  level: number;
  headingContent: string;
}

export const HEADING_OTID_RE = /^(\s{0,3}#{1,6}\s+)\[\[(\d{13,16})\]\]\s*(.*)/;

export function normalizeBlockContent(
  otid: number,
  content: string,
): string {
  const lines = content.split("\n");
  const firstLine = lines[0] || "";
  if (HEADING_OTID_RE.test(firstLine)) {
    return content.trimEnd();
  }
  const cleaned = firstLine.replace(/^#{1,6}\s+/, "");
  const rest = lines.slice(1);
  return [`## [[${otid}]] ${cleaned}`, ...rest].join("\n").trimEnd();
}
