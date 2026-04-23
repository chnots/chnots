import { syntaxTree } from "@codemirror/language";
import type { EditorState } from "@codemirror/state";
import { HEADING_OTID_RE, type HeadingBlock } from "./block-model";

const HEADING_NODE_RE = /^ATXHeading([1-6])$/;

export function parseHeadings(state: EditorState): HeadingBlock[] {
  const allHeadings: {
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
      allHeadings.push({
        from: node.from,
        level,
        otid: match ? Number(match[2]) : undefined,
        headingContent: match ? match[3] : undefined,
      });
    },
  });

  const blocks: HeadingBlock[] = [];
  for (let i = 0; i < allHeadings.length; i++) {
    const h = allHeadings[i];
    if (h.otid === undefined || h.headingContent === undefined) continue;

    let blockEnd = state.doc.length;
    for (let j = i + 1; j < allHeadings.length; j++) {
      if (allHeadings[j].level <= h.level) {
        blockEnd = allHeadings[j].from;
        break;
      }
    }

    blocks.push({
      otid: h.otid,
      from: h.from,
      to: blockEnd,
      level: h.level,
      headingContent: h.headingContent,
    });
  }

  return blocks;
}
