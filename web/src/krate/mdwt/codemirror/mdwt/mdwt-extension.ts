import { syntaxTree } from "@codemirror/language";
import { type EditorState, RangeSetBuilder } from "@codemirror/state";
import {
  Decoration,
  type DecorationSet,
  type EditorView,
  ViewPlugin,
  type ViewUpdate,
} from "@codemirror/view";
import { tags as t } from "@lezer/highlight";

import type {
  BlockContext,
  InlineContext,
  Line,
  MarkdownConfig,
} from "@lezer/markdown";

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
      parse(cx: InlineContext, _next: number, pos: number) {
        if (cx.char(pos) !== 91 /* [ */ || cx.char(pos + 1) !== 91) {
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

/**
 * We should use MarkdownConfig instead of Decoration.
 * Most of the parsing happens on the backend, so
 * as long as the performance is good enough, we accept this approach.
 */
const toentTodoRE = /\[[A-Za-z]+( ![A-Z])?\]/;

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
      const entries: { from: number; to: number }[] = [];

      syntaxTree(state).iterate({
        enter: (node) => {
          if (node.name.startsWith("ATXHeading") || node.name === "ListItem") {
            const text = state.sliceDoc(node.from, node.to);

            const match: RegExpExecArray | null = toentTodoRE.exec(text);
            if (match !== null) {
              const start = node.from + match.index;
              const end = start + match[0].length;

              if (start !== end) {
                entries.push({ from: start, to: end });
              }
            }
          }
        },
      });

      entries.sort((a, b) => a.from - b.from || a.to - b.to);

      const builder = new RangeSetBuilder<Decoration>();
      for (const { from, to } of entries) {
        builder.add(from, to, todoHighlight);
      }
      return builder.finish();
    }
  },
  {
    decorations: (v) => v.decorations,
  },
);

const chnotPropsRE = /^\s*(;+)\s*([^:]+):(.*)$/;
const parseChnotProps = (cx: BlockContext, line: Line) => {
  const match = chnotPropsRE.exec(line.text);

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
  const root = cx.elt("ChnotProps", markerStart, valueEnd, [
    cx.elt("ChnotPropsMarker", markerStart, markerEnd),
    cx.elt("ChnotPropsKey", keyStart, keyEnd),
    cx.elt("ChnotPropsMarker", keyEnd, keyEnd + 1),
    cx.elt("ChnotPropsValue", valueStart, valueEnd),
  ]);

  cx.addElement(root);
  cx.nextLine();
  return true;
};

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
      parse(cx: BlockContext, line: Line) {
        return parseChnotProps(cx, line);
      },
      endLeaf(_cx: BlockContext, line, _leaf) {
        // try break the cx
        return chnotPropsRE.test(line.text);
      },
    },
  ],
};
