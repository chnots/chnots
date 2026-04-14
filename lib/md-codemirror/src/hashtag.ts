import { tags as t } from "@lezer/highlight";

import type { InlineContext, MarkdownConfig } from "@lezer/markdown";

const hashtagRE =
  /^[^\u2000-\u206F\u2E00-\u2E7F'!"#$%&()*+,.:;<=>?@^`{|}~[\]\\\s]+/;

export const Hashtag: MarkdownConfig = {
  defineNodes: [
    "Hashtag",
    {
      name: "HashtagLabel",
      style: t.tagName,
    },
    {
      name: "HashtagMark",
      style: t.escape,
    },
  ],
  parseInline: [
    {
      name: "Hashtag",
      parse(cx: InlineContext, next: number, pos: number) {
        if (next !== 35 /* # */) {
          return -1;
        }
        const start = pos;
        pos += 1;
        const match = hashtagRE.exec(cx.text.slice(pos - cx.offset));
        if (match && /\D/.test(match[0])) {
          pos += match[0].length;
          return cx.addElement(
            cx.elt("Hashtag", start, pos, [
              cx.elt("HashtagMark", start, start + 1),
              cx.elt("HashtagLabel", start + 1, pos),
            ]),
          );
        }
        return -1;
      },
    },
  ],
};
