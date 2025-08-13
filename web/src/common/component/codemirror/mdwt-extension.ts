import { styleTags } from "@lezer/highlight";
import { InlineContext, MarkdownConfig } from "@lezer/markdown";
import {
  backlinkIDTag,
  backlinkMarkTag,
  backlinkTag,
  hashtagLabelTag,
  hashtagMarkTag,
  hashtagTag,
} from "./mdwt-highlight";

const backlinkRE = /[0-9a-zA-Z-]{6,}\]\]/;

export const Backlink: MarkdownConfig = {
  defineNodes: ["Backlink", "BacklinkMarker", "BacklinkID"],
  parseInline: [
    {
      name: "Backlink",
      before: "Link",
      parse(cx: InlineContext, next: number, pos: number) {
        console.log(pos);
        if (cx.char(pos) != 91 /* [ */ || cx.char(pos + 1) != 91) {
          return -1;
        }

        const start = pos;
        pos += 1;
        const match = backlinkRE.exec(cx.text.slice(pos - cx.offset));
        if (match && /\D/.test(match[0])) {
          pos += match[0].length + 1;
          return cx.addElement(
            cx.elt("Backlink", start, pos, [
              cx.elt("BacklinkMarker", start, start + 2),
              cx.elt("BacklinkID", start + 2, pos),
              cx.elt("BacklinkMarker", pos - 2, pos),
            ]),
          );
        }
        return -1;
      },
    },
  ],
  props: [
    styleTags({
      Backlink: backlinkTag,
      BacklinkMarker: backlinkMarkTag,
      BacklinkID: backlinkIDTag,
    }),
  ],
};

const hashtagRE =
  /^[^\u2000-\u206F\u2E00-\u2E7F'!"#$%&()*+,.:;<=>?@^`{|}~\[\]\\\s]+/;

export const Hashtag: MarkdownConfig = {
  defineNodes: ["Hashtag", "HashtagMark", "HashtagLabel"],
  parseInline: [
    {
      name: "Hashtag",
      parse(cx: InlineContext, next: number, pos: number) {
        if (next != 35 /* # */) {
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
  props: [
    styleTags({
      Hashtag: hashtagTag,
      HashtagMark: hashtagMarkTag,
      HashtagLabel: hashtagLabelTag,
    }),
  ],
};
