import { Annotation, type Transaction } from "@codemirror/state";
import { type PluginValue, ViewPlugin, type ViewUpdate } from "@codemirror/view";
import { syntaxTree } from "@codemirror/language";
import type { EditorState } from "@codemirror/state";

const HEADING_NODE_RE = /^ATXHeading([1-6])$/;
const HEADING_OTID_RE = /^(\s{0,3}#{1,6}\s+)\[\[(\d{13,16})\]\]\s*(.*)/;
const HEADING_PLAIN_RE = /^(\s{0,3}#{1,6}\s+)(.*)/;

interface MissingOtid {
  insertPos: number;
}

const selfAnnotation = Annotation.define<boolean>();

function findHeadingsWithoutOtid(state: EditorState): MissingOtid[] {
  const missing: MissingOtid[] = [];
  syntaxTree(state).iterate({
    enter(node) {
      const m = node.name.match(HEADING_NODE_RE);
      if (!m) return;
      const text = state.sliceDoc(node.from, node.to);
      if (HEADING_OTID_RE.test(text)) return;
      const plain = text.match(HEADING_PLAIN_RE);
      if (!plain) return;
      missing.push({
        insertPos: node.from + plain[1].length,
      });
    },
  });
  return missing;
}

export function otidInjector(genTID: () => number) {
  return ViewPlugin.fromClass(
    class implements PluginValue {
      update(update: ViewUpdate) {
        if (!update.docChanged) return;
        if (isSelfTransaction(update.transactions)) return;

        const headings = findHeadingsWithoutOtid(update.state);
        if (headings.length === 0) return;

        const changes = [...headings].reverse().map((h) => ({
          from: h.insertPos,
          insert: `[[${genTID()}]] `,
        }));

        const view = update.view;
        queueMicrotask(() => {
          try {
            view.dispatch({
              changes,
              annotations: selfAnnotation.of(true),
            });
          } catch {
            // view may have been destroyed
          }
        });
      }
    },
  );
}

function isSelfTransaction(transactions: readonly Transaction[]): boolean {
  return transactions.some(
    (tr) => tr.annotation(selfAnnotation) !== undefined,
  );
}
