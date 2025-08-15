import { InlineContext, MarkdownConfig } from "@lezer/markdown";
import { tags as t } from "@lezer/highlight";
import {
  Decoration,
  DecorationSet,
  EditorView,
  ViewPlugin,
  ViewUpdate,
} from "@codemirror/view";
import { EditorState, RangeSetBuilder } from "@codemirror/state";
import { syntaxTree } from "@codemirror/language";

const backlinkRE = /[0-9a-zA-Z-]{6,}\]\]/;

export const Backlink: MarkdownConfig = {
  defineNodes: [
    "Backlink",
    {
      name: "BacklinkMarker",
      style: t.escape,
    },
    {
      name: "BacklinkID",
      style: t.heading1,
    },
  ],
  parseInline: [
    {
      name: "Backlink",
      before: "Link",
      parse(cx: InlineContext, next: number, pos: number) {
        if (cx.char(pos) != 91 /* [ */ || cx.char(pos + 1) != 91) {
          return -1;
        }

        const start = pos;
        pos += 1;
        const match = backlinkRE.exec(cx.text.slice(pos + 1 - cx.offset));

        if (match) {
          pos += match[0].length + 1;
          return cx.addElement(
            cx.elt("Backlink", start, pos, [
              cx.elt("BacklinkMarker", start, start + 2),
              cx.elt("BacklinkID", start + 2, pos - 2),
              cx.elt("BacklinkMarker", pos - 2, pos),
            ]),
          );
        }
        return -1;
      },
    },
  ],
};

const hashtagRE =
  /^[^\u2000-\u206F\u2E00-\u2E7F'!"#$%&()*+,.:;<=>?@^`{|}~\[\]\\\s]+/;

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
        if (next != 35 /* # */) {
          return -1;
        }
        const start = pos;
        pos += 1;
        const match = hashtagRE.exec(cx.text.slice(pos - cx.offset));
        if (match && /\D/.test(match[0])) {
          pos += match[0].length;
          console.log("...");
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

/**
 * We should use MarkdownConfig instead of Decoration.
 * Most of the parsing happens on the backend, so
 * as long as the performance is good enough, we accept this approach.
 */
const toentTodoRE = /\[[a-zA-Z]+\]/;

const todoHighlight = Decoration.mark({
  class: "cm-todo-highlight",
  attributes: { "aria-label": "TODO item" },
});

export const todoHighlightPlugin = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = this.findTodos(view.state);
    }

    update(update: ViewUpdate) {
      if (update.docChanged || update.viewportChanged) {
        this.decorations = this.findTodos(update.state);
      }
    }

    findTodos(state: EditorState): DecorationSet {
      const builder = new RangeSetBuilder<Decoration>();

      syntaxTree(state).iterate({
        enter: (node) => {
          if (node.name.startsWith("ATXHeading") || node.name === "ListItem") {
            const text = state.sliceDoc(node.from, node.to);

            let match;
            if ((match = toentTodoRE.exec(text)) !== null) {
              const start = node.from + match.index;
              const end = start + match[0].length;

              if (start !== end) {
                builder.add(start, end, todoHighlight);
              }
            }
          }
        },
      });

      return builder.finish();
    }
  },
  {
    decorations: (v) => v.decorations,
  },
);

export const ChnotProps: MarkdownConfig = {
  defineNodes: [
    { name: "ChnotProps", block: true, style: t.meta },
    { name: "ChnotPropsKey", style: t.atom },
    { name: "ChnotPropsMarker", style: t.comment },
    { name: "ChnotPropsValue", style: t.atom },
  ],

  parseBlock: [
    {
      name: "ChnotProps",
      parse(cx, line) {
        const match = /^\s*(;)\s*([^:]+):(.*)$/.exec(line.text);
        if (!match) return false;

        const base = cx.lineStart + match.index;
        const markerStart = base + match[0].indexOf(";");
        const markerEnd = markerStart + 1;

        const keyStart =
          markerEnd +
          (match[1].length - 1) +
          (match[0].indexOf(match[2]) - match[0].indexOf(";") - 1);
        const keyEnd = keyStart + match[2].length;

        const colonPos = line.text.indexOf(":", keyStart - line.pos);
        const valueStart = colonPos >= 0 ? line.pos + colonPos + 1 : keyEnd;
        const valueEnd = base + line.text.length;

        console.log(base, markerStart, keyStart, keyEnd, valueStart, valueEnd);
        const root = cx.elt("ChnotProps", markerStart, valueEnd, [
          cx.elt("ChnotPropsMarker", markerStart, markerEnd),
          cx.elt("ChnotPropsKey", keyStart, keyEnd),
          cx.elt("ChnotPropsMarker", keyEnd, keyEnd + 1),
          cx.elt("ChnotPropsValue", valueStart, valueEnd),
        ]);

        cx.addElement(root);
        cx.nextLine();
        return true;
      },
    },
  ],
};
