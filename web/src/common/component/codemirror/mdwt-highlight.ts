// hashtag-highlight.ts
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { Tag } from "@lezer/highlight";

export const hashtagTag = Tag.define();
export const hashtagMarkTag = Tag.define();
export const hashtagLabelTag = Tag.define();

export const backlinkIDTag = Tag.define();
export const backlinkMarkTag = Tag.define();
export const backlinkTag = Tag.define();

export const mdwtHighlight = HighlightStyle.define([
  {
    tag: hashtagTag,
    class: "cm-hashtag",
  },
  {
    tag: hashtagMarkTag,
    class: "cm-hashtag-mark",
  },
  {
    tag: hashtagLabelTag,
    class: "cm-hashtag-label",
  },
  {
    tag: backlinkTag,
    class: "cm-backlink",
  },
  {
    tag: backlinkMarkTag,
    class: "cm-backlink-mark",
  },
  {
    tag: backlinkIDTag,
    class: "cm-backlink-id",
  },
]);

export const mdwtHighlightExtension = syntaxHighlighting(mdwtHighlight);
