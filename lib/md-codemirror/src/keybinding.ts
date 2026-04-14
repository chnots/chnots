import { historyKeymap, standardKeymap } from "@codemirror/commands";
import { getIndentUnit, indentString } from "@codemirror/language";
import { EditorSelection, Prec } from "@codemirror/state";
import {
  type Command,
  type EditorView,
  type KeyBinding,
  keymap,
} from "@codemirror/view";

import {
  insertLineAfter,
  intersectsSyntaxNode,
  isCursorAtBeginning,
  renumberSelectedLists,
  toggleBold,
  toggleHighlight,
  toggleInlineCode,
  toggleItalic,
  toggleSelectedLinesStartWith,
  toggleStrikethrough,
} from "./index";

export const increaseIndent: Command = (view: EditorView): boolean => {
  const matchEmpty = true;
  const matchNothing = /$ ^/;
  const indentUnit = indentString(view.state, getIndentUnit(view.state));

  const changes = toggleSelectedLinesStartWith(
    view.state,
    matchNothing,
    indentUnit,
    matchEmpty,
  );
  view.dispatch(changes);

  view.dispatch(renumberSelectedLists(view.state));

  return true;
};

export const insertOrIncreaseIndent: Command = (view: EditorView): boolean => {
  const selection = view.state.selection;
  const mainSelection = selection.main;
  if (selection.ranges.length !== 1 || !mainSelection.empty) {
    return increaseIndent(view);
  }

  if (intersectsSyntaxNode(view.state, mainSelection, "ListItem")) {
    return increaseIndent(view);
  }

  const indentUnit = indentString(view.state, getIndentUnit(view.state));
  view.dispatch(
    view.state.changeByRange((selection) => {
      return {
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
    new RegExp(`^(?:[\\t]|[ ]{1,${getIndentUnit(view.state)}})`),
    "",
    matchEmpty,
  );

  view.dispatch(changes);

  view.dispatch(renumberSelectedLists(view.state));

  return true;
};

const dispatchFormatToggle = (
  view: EditorView,
  toggleFn: (
    state: import("@codemirror/state").EditorState,
  ) => import("@codemirror/state").TransactionSpec,
): boolean => {
  view.dispatch(toggleFn(view.state));
  return true;
};

export const generateKeybinding = (
  onCtrlEnter?: (view: EditorView) => boolean,
) => {
  const keyCommand = (
    key: string,
    run: Command,
    _alwaysActive?: boolean,
  ): KeyBinding => {
    return {
      key,
      run: (editor) => {
        return run(editor);
      },
    };
  };

  const keymapConfig = Prec.high(
    keymap.of([
      keyCommand(
        "Tab",
        (view: EditorView) => {
          return insertOrIncreaseIndent(view);
        },
        true,
      ),
      keyCommand(
        "Shift-Tab",
        (view) => {
          if (isCursorAtBeginning(view.state)) {
            return false;
          }

          return decreaseIndent(view);
        },
        true,
      ),
      keyCommand(
        "Mod-Enter",
        (view: EditorView) => {
          if (onCtrlEnter?.(view)) {
            return true;
          }
          insertLineAfter(view);
          return true;
        },
        true,
      ),
      keyCommand(
        "Mod-b",
        (view: EditorView) => dispatchFormatToggle(view, toggleBold),
        true,
      ),
      keyCommand(
        "Mod-i",
        (view: EditorView) => dispatchFormatToggle(view, toggleItalic),
        true,
      ),
      keyCommand(
        "Mod-Shift-s",
        (view: EditorView) => dispatchFormatToggle(view, toggleStrikethrough),
        true,
      ),
      keyCommand(
        "Mod-e",
        (view: EditorView) => dispatchFormatToggle(view, toggleInlineCode),
        true,
      ),
      keyCommand(
        "Mod-Shift-h",
        (view: EditorView) => dispatchFormatToggle(view, toggleHighlight),
        true,
      ),

      ...standardKeymap,
      ...historyKeymap,
    ]),
  );

  return keymapConfig;
};
