import type {
  CompletionContext,
  CompletionResult,
} from "@codemirror/autocomplete";

export function tidCompletion(config: { genTID: () => number }) {
  return (context: CompletionContext): CompletionResult | null => {
    const line = context.state.doc.lineAt(context.pos);
    if (!/^\s{0,3}#{1,6}\s/.test(line.text)) return null;

    const word = context.matchBefore(/\/tid$/);
    if (!word || (word.from === word.to && !context.explicit)) return null;

    return {
      from: word.from,
      options: [
        {
          label: "/tid",
          displayLabel: "/tid",
          detail: "Insert block OTID",
          apply: `[[${config.genTID()}]] `,
          type: "keyword",
        },
      ],
      filter: false,
    };
  };
}
