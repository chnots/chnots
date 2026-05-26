import { RangeSetBuilder, StateField } from "@codemirror/state";
import { Decoration, type DecorationSet, EditorView } from "@codemirror/view";
import type { EditorState } from "@codemirror/state";
import type { HeadingBlock } from "./block-model";
import { headingBlockField } from "./block-field";

function buildBlockLineDecorations(
  state: EditorState,
  blocks: HeadingBlock[],
): DecorationSet {
  const entries: { from: number; deco: Decoration }[] = [];

  for (const block of blocks) {
    let pos = block.from;
    while (pos < block.to && pos <= state.doc.length) {
      const line = state.doc.lineAt(pos);
      entries.push({
        from: line.from,
        deco: Decoration.line({
          attributes: { "data-block-otid": String(block.otid) },
        }),
      });
      if (line.to >= state.doc.length) break;
      pos = line.to + 1;
    }
  }

  entries.sort((a, b) => a.from - b.from);

  const builder = new RangeSetBuilder<Decoration>();
  for (const { from, deco } of entries) {
    builder.add(from, from, deco);
  }
  return builder.finish();
}

export const blockDecorationField = StateField.define<DecorationSet>({
  create(state) {
    const blocks = state.field(headingBlockField, false) ?? [];
    return buildBlockLineDecorations(state, blocks);
  },
  update(_decos, tr) {
    if (tr.docChanged) {
      const blocks = tr.state.field(headingBlockField, false) ?? [];
      return buildBlockLineDecorations(tr.state, blocks);
    }
    return _decos.map(tr.changes);
  },
  provide: (f) => EditorView.decorations.from(f),
});
