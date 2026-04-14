import { syntaxTree } from '@codemirror/language';
import { type EditorState, Prec } from '@codemirror/state';
import { type Command, type EditorView, keymap } from '@codemirror/view';

function findTableCoveringLine(
  state: EditorState,
  lineNumber: number,
): { from: number; to: number } | null {
  let result: { from: number; to: number } | null = null;
  syntaxTree(state).iterate({
    enter: (node) => {
      if (node.name === 'Table') {
        const startLine = state.doc.lineAt(node.from).number;
        const endLine = state.doc.lineAt(node.to).number;
        if (startLine <= lineNumber && lineNumber <= endLine) {
          result = { from: node.from, to: node.to };
        }
      }
    },
  });
  return result;
}

const arrowDown: Command = (view: EditorView): boolean => {
  const { state } = view;
  const pos = state.selection.main.head;
  const curLineNum = state.doc.lineAt(pos).number;

  if (findTableCoveringLine(state, curLineNum)) return false;

  const nextLineNum = curLineNum + 1;
  if (nextLineNum > state.doc.lines) return false;

  const table = findTableCoveringLine(state, nextLineNum);
  if (!table) return false;

  const curLine = state.doc.lineAt(pos);
  const targetLine = state.doc.lineAt(table.from);
  const colOffset = pos - curLine.from;
  const targetPos = Math.max(
    targetLine.from,
    Math.min(targetLine.from + colOffset, targetLine.to - 1),
  );
  view.dispatch({ selection: { anchor: targetPos } });
  return true;
};

const arrowUp: Command = (view: EditorView): boolean => {
  const { state } = view;
  const pos = state.selection.main.head;
  const curLineNum = state.doc.lineAt(pos).number;

  if (findTableCoveringLine(state, curLineNum)) return false;

  const prevLineNum = curLineNum - 1;
  if (prevLineNum < 1) return false;

  const table = findTableCoveringLine(state, prevLineNum);
  if (!table) return false;

  const curLine = state.doc.lineAt(pos);
  const lastLinePos = table.to > table.from ? table.to - 1 : table.from;
  const targetLine = state.doc.lineAt(lastLinePos);
  const colOffset = pos - curLine.from;
  const targetPos = Math.max(
    targetLine.from,
    Math.min(targetLine.from + colOffset, targetLine.to - 1),
  );
  view.dispatch({ selection: { anchor: targetPos } });
  return true;
};

const shiftArrowDown: Command = (view: EditorView): boolean => {
  const { state } = view;
  const sel = state.selection.main;
  const curLineNum = state.doc.lineAt(sel.head).number;

  if (findTableCoveringLine(state, curLineNum)) return false;

  const nextLineNum = curLineNum + 1;
  if (nextLineNum > state.doc.lines) return false;

  const table = findTableCoveringLine(state, nextLineNum);
  if (!table) return false;

  const curLine = state.doc.lineAt(sel.head);
  const targetLine = state.doc.lineAt(table.from);
  const colOffset = sel.head - curLine.from;
  const targetPos = Math.max(
    targetLine.from,
    Math.min(targetLine.from + colOffset, targetLine.to - 1),
  );
  view.dispatch({ selection: { anchor: sel.anchor, head: targetPos } });
  return true;
};

const shiftArrowUp: Command = (view: EditorView): boolean => {
  const { state } = view;
  const sel = state.selection.main;
  const curLineNum = state.doc.lineAt(sel.head).number;

  if (findTableCoveringLine(state, curLineNum)) return false;

  const prevLineNum = curLineNum - 1;
  if (prevLineNum < 1) return false;

  const table = findTableCoveringLine(state, prevLineNum);
  if (!table) return false;

  const curLine = state.doc.lineAt(sel.head);
  const lastLinePos = table.to > table.from ? table.to - 1 : table.from;
  const targetLine = state.doc.lineAt(lastLinePos);
  const colOffset = sel.head - curLine.from;
  const targetPos = Math.max(
    targetLine.from,
    Math.min(targetLine.from + colOffset, targetLine.to - 1),
  );
  view.dispatch({ selection: { anchor: sel.anchor, head: targetPos } });
  return true;
};

export const tableNavigation = () =>
  Prec.highest(
    keymap.of([
      { key: 'ArrowDown', run: arrowDown, shift: shiftArrowDown },
      { key: 'ArrowUp', run: arrowUp, shift: shiftArrowUp },
    ]),
  );
