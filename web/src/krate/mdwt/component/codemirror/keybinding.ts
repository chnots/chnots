import { historyKeymap, standardKeymap } from '@codemirror/commands';
import { getIndentUnit, indentString } from '@codemirror/language';
import { EditorSelection, Prec } from '@codemirror/state';
import { type Command, type EditorView, type KeyBinding, keymap } from '@codemirror/view';
import {
  insertLineAfter,
  intersectsSyntaxNode,
  isCursorAtBeginning,
  renumberSelectedLists,
  toggleSelectedLinesStartWith,
} from 'jolpin-codemirror';

// Prepends the given editor's indentUnit to all lines of the current selection
// and re-numbers modified ordered lists (if any).
export const increaseIndent: Command = (view: EditorView): boolean => {
  const matchEmpty = true;
  const matchNothing = /$ ^/;
  const indentUnit = indentString(view.state, getIndentUnit(view.state));

  const changes = toggleSelectedLinesStartWith(
    view.state,
    // Delete nothing
    matchNothing,
    // ...and thus always add indentUnit.
    indentUnit,
    matchEmpty,
  );
  view.dispatch(changes);

  // Fix any lists
  view.dispatch(renumberSelectedLists(view.state));

  return true;
};

// Like `increaseIndent`, but may insert tabs, rather than
// indenting, in some instances.
export const insertOrIncreaseIndent: Command = (view: EditorView): boolean => {
  const selection = view.state.selection;
  const mainSelection = selection.main;
  if (selection.ranges.length !== 1 || !mainSelection.empty) {
    return increaseIndent(view);
  }

  if (intersectsSyntaxNode(view.state, mainSelection, 'ListItem')) {
    return increaseIndent(view);
  }

  const indentUnit = indentString(view.state, getIndentUnit(view.state));
  view.dispatch(
    view.state.changeByRange((selection) => {
      return {
        // Move the selection to after the inserted text
        range: EditorSelection.cursor(selection.from + indentUnit.length),
        changes: {
          from: selection.from,
          insert: indentUnit,
        },
      };
    }),
  );

  return true;
};

export const decreaseIndent: Command = (view: EditorView): boolean => {
  const matchEmpty = true;
  const changes = toggleSelectedLinesStartWith(
    view.state,
    // Assume indentation is either a tab or in units
    // of n spaces.
    new RegExp(`^(?:[\\t]|[ ]{1,${getIndentUnit(view.state)}})`),
    // Don't add new text
    '',
    matchEmpty,
  );

  view.dispatch(changes);

  // Fix any lists
  view.dispatch(renumberSelectedLists(view.state));

  return true;
};

export const generateKeybinding = () => {
  const keyCommand = (key: string, run: Command, alwaysActive?: boolean): KeyBinding => {
    return {
      key,
      run: (editor) => {
        // if (settings.ignoreModifiers && !alwaysActive) return false;

        return run(editor);
      },
    };
  };

  const keymapConfig = Prec.high(
    keymap.of([
      keyCommand(
        'Tab',
        (view: EditorView) => {
          return insertOrIncreaseIndent(view);
        },
        true,
      ),
      keyCommand(
        'Shift-Tab',
        (view) => {
          // When at the beginning of the editor, allow shift-tab to act
          // normally.
          if (isCursorAtBeginning(view.state)) {
            return false;
          }

          return decreaseIndent(view);
        },
        true,
      ),
      keyCommand(
        'Mod-Enter',
        (_: EditorView) => {
          insertLineAfter(_);
          return true;
        },
        true,
      ),

      ...standardKeymap,
      ...historyKeymap,
    ]),
  );

  return keymapConfig;
};
