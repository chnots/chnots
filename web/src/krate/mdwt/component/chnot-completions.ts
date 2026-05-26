import type {
  Completion,
  CompletionContext,
  CompletionResult,
} from "@codemirror/autocomplete";
import { chnotSearch } from "@/krate/chnot/service";
import { toentTodoEventGuess } from "@/krate/toent/service";
import { chnotTagNameList } from "../service";
import { slashCommandCompletions } from "./slash-completions";

export const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const slashResult = slashCommandCompletions(context);
  if (slashResult) return slashResult;

  const word = context.matchBefore(/#[^# ]*|^#* \[|^[ ]*- \[|\[\[/);
  let options: Completion[];
  if (!word || (word?.from === word?.to && !context.explicit)) {
    return null;
  } else if (word.text.startsWith("#")) {
    options = (
      await chnotTagNameList({
        query: word.text,
        start_index: 0,
        page_size: 20,
      })
    ).data.map((name) => {
      return { label: name, type: "hashtag" };
    });
  } else if (word.text.startsWith("[[")) {
    options = (
      await chnotSearch({
        query: word.text.substring(3),
        start_index: 0,
        page_size: 10,
        kinds: [],
      })
    ).data.map((chnot) => {
      return {
        label: chnot.title ?? "",
        apply: `[[${chnot.meta.otid}]]`,
        type: "backlink",
      };
    });
  } else if (word.text.includes("# [") || word.text.includes("- [")) {
    options = (
      await toentTodoEventGuess({ input: word.text.replace(/.*\[/, "") })
    ).toents.map((toent) => {
      return { label: `{${toent}}`, type: "toent" };
    });
  } else {
    return null;
  }

  options.sort((e1, e2) => e1.label.length - e2.label.length);

  return {
    from: word.from,
    options: options,
    filter: false,
  };
};
