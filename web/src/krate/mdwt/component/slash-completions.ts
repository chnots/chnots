import type {
  Completion,
  CompletionContext,
  CompletionResult,
} from "@codemirror/autocomplete";
import { format } from "date-fns";

const SLASH_COMMANDS: Completion[] = [
  {
    label: "/time",
    displayLabel: "/time",
    detail: "Insert current date & time",
    apply: (_view, _completion, from, to) => {
      _view.dispatch({
        changes: { from, to, insert: format(new Date(), "yyyy-MM-dd HH:mm") },
      });
    },
    type: "keyword",
    boost: 1,
  },
  {
    label: "/date",
    displayLabel: "/date",
    detail: "Insert current date",
    apply: (_view, _completion, from, to) => {
      _view.dispatch({
        changes: { from, to, insert: format(new Date(), "yyyy-MM-dd") },
      });
    },
    type: "keyword",
    boost: 1,
  },
];

export const slashCommandCompletions = (
  context: CompletionContext,
): CompletionResult | null => {
  const word = context.matchBefore(/\/[a-zA-Z]*$/);
  if (!word || (word.from === word.to && !context.explicit)) return null;
  if (word.text === "/") {
    return { from: word.from, options: SLASH_COMMANDS, filter: false };
  }
  const query = word.text.toLowerCase();
  const filtered = SLASH_COMMANDS.filter((c) =>
    c.label.toLowerCase().startsWith(query),
  );
  if (filtered.length === 0) return null;
  return { from: word.from, options: filtered, filter: false };
};
