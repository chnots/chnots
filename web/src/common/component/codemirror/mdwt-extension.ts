import { InlineContext, MarkdownConfig } from "@lezer/markdown";

const BacklinkDelim = { resolve: "Backlink", mark: "BacklinkMarker" };

export const Backlink: MarkdownConfig = {
  defineNodes: ["Backlink", "BacklinkMarker"],
  parseInline: [
    {
      name: "BacklinkInline",
      parse(cx: InlineContext, next: number, pos: number) {
        if (next == 37 && cx.char(pos + 1) == 37) {
          let canClose = true;
          if (
            cx.slice(cx.offset, pos).lastIndexOf("\n") >
            cx.slice(cx.offset, pos).lastIndexOf("%%")
          ) {
            canClose = false;
          }
          return cx.addDelimiter(BacklinkDelim, pos, pos + 2, true, canClose);
        }
        return -1;
      },
    },
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
          console.log("parse hashtag successfully");

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
