// hashtag-highlight.ts
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { Tag } from "@lezer/highlight";

export const hashtagTag = Tag.define();
export const hashtagMarkTag = Tag.define();
export const hashtagLabelTag = Tag.define();

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
]);

export const mdwtHighlightExtension = syntaxHighlighting(mdwtHighlight);
