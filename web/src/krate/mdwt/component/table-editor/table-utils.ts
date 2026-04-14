import type { ColumnAlign } from "jolpin-codemirror";

export interface ParsedTable {
  headers: string[];
  alignments: ColumnAlign[];
  rows: string[][];
}

export function parseMarkdownTable(rawText: string): ParsedTable | null {
  const lines = rawText.split("\n").filter((line) => line.trim());
  if (lines.length < 2) return null;

  const headers: string[] = [];
  const alignments: ColumnAlign[] = [];
  const rows: string[][] = [];

  for (const line of lines) {
    const trimmed = line.trim();
    if (/^\|?\s*[-:]+[-|\s:]*$/.test(trimmed)) {
      alignments.push(
        ...trimmed
          .replace(/^\|/, "")
          .replace(/\|$/, "")
          .split("|")
          .map((cell) => {
            const c = cell.trim();
            if (c.startsWith(":") && c.endsWith(":")) return "center";
            if (c.endsWith(":")) return "right";
            return "left";
          }),
      );
      continue;
    }

    const cells = trimmed
      .replace(/^\|/, "")
      .replace(/\|$/, "")
      .split("|")
      .map((c) => c.trim());

    if (headers.length === 0) {
      headers.push(...cells);
    } else {
      rows.push(cells);
    }
  }

  if (headers.length === 0) return null;

  const colCount = headers.length;
  for (const row of rows) {
    while (row.length < colCount) row.push("");
    if (row.length > colCount) row.length = colCount;
  }

  while (alignments.length < colCount) alignments.push("left");

  return { headers, alignments, rows };
}

export function generateMarkdownTable(table: ParsedTable): string {
  const { headers, alignments, rows } = table;
  const colCount = headers.length;

  const maxLens = Array.from({ length: colCount }, (_, i) => {
    const headerLen = headers[i]?.length ?? 0;
    const rowLens = rows.map((r) => r[i]?.length ?? 0);
    return Math.max(headerLen, ...rowLens, 3);
  });

  const padCell = (cell: string, colIdx: number) => {
    const len = maxLens[colIdx] ?? 3;
    return cell.padEnd(len);
  };

  const headerLine = `| ${headers.map((h, i) => padCell(h, i)).join(" | ")} |`;
  const delimiterLine = `| ${alignments
    .map((a, i) => {
      const len = maxLens[i] ?? 3;
      if (a === "center") return `:${"-".repeat(Math.max(len - 2, 1))}:`;
      if (a === "right") return `${"-".repeat(Math.max(len - 1, 2))}:`;
      return "-".repeat(Math.max(len, 3));
    })
    .join(" | ")} |`;
  const rowLines = rows.map(
    (row) => `| ${row.map((c, i) => padCell(c, i)).join(" | ")} |`,
  );

  return [headerLine, delimiterLine, ...rowLines].join("\n");
}
