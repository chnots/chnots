import { syntaxTree } from "@codemirror/language";
import type { EditorState } from "@codemirror/state";
import { HEADING_OTID_RE, type HeadingBlock } from "./block-model";

const HEADING_NODE_RE = /^ATXHeading([1-6])$/;

export function parseHeadings(state: EditorState): HeadingBlock[] {
  const rawHeadings: {
    from: number;
    level: number;
    otid?: number;
    headingContent?: string;
  }[] = [];

  syntaxTree(state).iterate({
    enter(node) {
      const m = node.name.match(HEADING_NODE_RE);
      if (!m) return;
      const level = parseInt(m[1]);
      const text = state.sliceDoc(node.from, node.to);
      const match = text.match(HEADING_OTID_RE);
      rawHeadings.push({
        from: node.from,
        level,
        otid: match ? Number(match[2]) : undefined,
        headingContent: match ? match[3] : undefined,
      });
    },
  });

  // Single backward pass: compute each heading's boundary as the start of
  // the next heading at the same or lower level (i.e. a higher-level section).
  const ends = new Array<number>(rawHeadings.length).fill(state.doc.length);
  const stack: { index: number; level: number }[] = [];
  for (let i = rawHeadings.length - 1; i >= 0; i--) {
    const lvl = rawHeadings[i].level;
    while (stack.length > 0 && stack[stack.length - 1].level > lvl) {
      ends[stack.pop()!.index] = rawHeadings[i].from;
    }
    stack.push({ index: i, level: lvl });
  }

  const blocks: HeadingBlock[] = [];
  for (let i = 0; i < rawHeadings.length; i++) {
    const h = rawHeadings[i];
    if (h.otid === undefined || h.headingContent === undefined) continue;
    blocks.push({
      otid: h.otid,
      from: h.from,
      to: ends[i],
      level: h.level,
      headingContent: h.headingContent,
    });
  }

  return blocks;
}
