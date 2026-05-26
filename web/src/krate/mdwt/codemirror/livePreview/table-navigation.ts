import { Prec } from '@codemirror/state';
import { type Command, type EditorView, keymap } from '@codemirror/view';

/**
 * Move cursor vertically by one document line, preserving column offset.
 * Uses document-line navigation to avoid CodeMirror's built-in behavior
 * of skipping over block widgets (Decoration.replace with block:true).
 */
function moveVertically(
  view: EditorView,
  dir: 1 | -1,
  select: boolean,
): boolean {
  const { state } = view;
  const sel = state.selection.main;
  const curLine = state.doc.lineAt(sel.head);
  const targetLineNum = curLine.number + dir;
  if (targetLineNum < 1 || targetLineNum > state.doc.lines) return false;

  const targetLine = state.doc.line(targetLineNum);
  const colOffset = sel.head - curLine.from;
  const targetPos = targetLine.from + Math.min(colOffset, targetLine.text.length);

  view.dispatch({
    selection: select
      ? { anchor: sel.anchor, head: targetPos }
      : { anchor: targetPos },
    scrollIntoView: true,
  });
  return true;
}

const arrowDown: Command = (view) => moveVertically(view, 1, false);
const arrowUp: Command = (view) => moveVertically(view, -1, false);
const shiftArrowDown: Command = (view) => moveVertically(view, 1, true);
const shiftArrowUp: Command = (view) => moveVertically(view, -1, true);

export const tableNavigation = () =>
  Prec.highest(
    keymap.of([
      { key: 'ArrowDown', run: arrowDown, shift: shiftArrowDown },
      { key: 'ArrowUp', run: arrowUp, shift: shiftArrowUp },
    ]),
  );
