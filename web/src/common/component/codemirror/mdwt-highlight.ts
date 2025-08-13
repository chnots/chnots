// hashtag-highlight.ts
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { Tag } from "@lezer/highlight";

export const hashtagTag = Tag.define();
export const hashtagMarkTag = Tag.define();
export const hashtagLabelTag = Tag.define();

export const backlinkIDTag = Tag.define();
export const backlinkMarkTag = Tag.define();
export const backlinkTag = Tag.define();

export const toentTag = Tag.define();
export const toentMarkTag = Tag.define();
export const toentTodoTag = Tag.define();
export const toentEventTag = Tag.define();

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
  {
    tag: toentTag,
    class: "cm-toent",
  },
  {
    tag: toentMarkTag,
    class: "cm-toent-mark",
  },
  {
    tag: toentTodoTag,
    class: "cm-toent-todo",
  },
  {
    tag: toentEventTag,
    class: "cm-toent-event",
  },
]);

export const mdwtHighlightExtension = syntaxHighlighting(mdwtHighlight);
