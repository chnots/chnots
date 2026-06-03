import { tags as t } from "@lezer/highlight";

import type {
  BlockContext,
  InlineContext,
  Line,
  MarkdownConfig,
} from "@lezer/markdown";

const blockMathDollarRE = /^\s*\$\$\s*$/;
const blockMathBracketOpenRE = /^\s*\\\[/
const blockMathBracketCloseRE = /^\s*\\\]/
const blankLineRE = /^\s*$/;

const parseBlockMath = (cx: BlockContext, line: Line): boolean => {
  // $$ ... $$ style
  if (blockMathDollarRE.test(line.text)) {
    const blockStart = cx.lineStart;

    while (cx.nextLine()) {
      if (blankLineRE.test(line.text)) return false;
      if (blockMathDollarRE.test(line.text)) {
        cx.addElement(
          cx.elt("BlockMath", blockStart, cx.lineStart + line.text.length),
        );
        cx.nextLine();
        return true;
      }
    }

    return false;
  }

  // \[ ... \] style
  if (blockMathBracketOpenRE.test(line.text)) {
    const blockStart = cx.lineStart;

    while (cx.nextLine()) {
      if (blockMathBracketCloseRE.test(line.text)) {
        cx.addElement(
          cx.elt("BlockMath", blockStart, cx.lineStart + line.text.length),
        );
        cx.nextLine();
        return true;
      }
    }

    return false;
  }

  return false;
};

export const MathConfig: MarkdownConfig = {
  defineNodes: [
    { name: "InlineMath", style: t.monospace },
    { name: "BlockMath", block: true, style: t.monospace },
  ],
  parseInline: [
    {
      name: "InlineMath",
      before: "Escape",
      parse(cx: InlineContext, next: number, pos: number) {
        // $ ... $ and $$ ... $$ style
        if (next === 36) {
          const isDouble = cx.char(pos + 1) === 36;
          const delimLen = isDouble ? 2 : 1;

          if (!isDouble && /\s/.test(cx.slice(pos + 1, pos + 2))) return -1;

          let endPos = pos + delimLen;
          const lineEnd = cx.offset + cx.text.length;

          while (endPos < lineEnd) {
            if (cx.char(endPos) === 36) {
              if (isDouble) {
                if (cx.char(endPos + 1) !== 36) {
                  endPos++;
                  continue;
                }
                return cx.addElement(cx.elt("InlineMath", pos, endPos + 2));
              } else {
                if (/\s/.test(cx.slice(endPos - 1, endPos))) {
                  endPos++;
                  continue;
                }
                return cx.addElement(cx.elt("InlineMath", pos, endPos + 1));
              }
            }
            endPos++;
          }
          return -1;
        }

        // \( ... \) style
        if (next === 92 && cx.char(pos + 1) === 40) {
          let endPos = pos + 2;
          const lineEnd = cx.offset + cx.text.length;

          while (endPos < lineEnd) {
            if (cx.char(endPos) === 92 && cx.char(endPos + 1) === 41) {
              return cx.addElement(cx.elt("InlineMath", pos, endPos + 2));
            }
            endPos++;
          }
          return -1;
        }

        return -1;
      },
    },
  ],
  parseBlock: [
    {
      name: "BlockMath",
      parse: parseBlockMath,
      endLeaf(_cx: BlockContext, line: Line) {
        return blockMathDollarRE.test(line.text) || blockMathBracketOpenRE.test(line.text);
      },
    },
  ],
};
