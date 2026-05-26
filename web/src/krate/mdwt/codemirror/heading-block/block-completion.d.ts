import type { CompletionContext, CompletionResult } from "@codemirror/autocomplete";
export declare function tidCompletion(config: {
    genTID: () => number;
}): (context: CompletionContext) => CompletionResult | null;
