import type {
  Completion,
  CompletionContext,
  CompletionResult,
} from "@codemirror/autocomplete";
import type { EditorView } from "@codemirror/view";
import type { TID } from "@/lib/id_util";
import type { ChnotKind } from "@/krate/chnot/po";
import { chnotSearch } from "@/krate/chnot/service";
import { genTID } from "@/lib/id_util";

const CREATE_OPTIONS: {
  kind: ChnotKind;
  label: string;
  icon: string;
}[] = [
  { kind: "mdwt" as ChnotKind, label: "Markdown", icon: "\u{1F4DD}" },
  { kind: "exdrv1" as ChnotKind, label: "Excalidraw", icon: "\u{1F3A8}" },
  { kind: "ktabv1" as ChnotKind, label: "Table", icon: "\u{1F4CA}" },
  { kind: "resov1" as ChnotKind, label: "File", icon: "\u{1F4C1}" },
  { kind: "llm_chat" as ChnotKind, label: "LLM Chat", icon: "\u{1F916}" },
  { kind: "mindmapv1" as ChnotKind, label: "Mind Map", icon: "\u{1F9E0}" },
];

const HEADING_LINE_RE = /^\s{0,3}(#{1,6}) (.*)$/;

export type HeadingCompletionConfig = {
  parentOtid: TID;
  getExcludeOtids: () => TID[];
  onCreateChnot: (otid: TID, kind: ChnotKind) => void;
  onRefExisting: (otid: TID, kind: ChnotKind) => void;
};

export function headingChnotCompletion(
  getConfig: () => HeadingCompletionConfig,
) {
  return async (
    context: CompletionContext,
  ): Promise<CompletionResult | null> => {
    const config = getConfig();
    const line = context.state.doc.lineAt(context.pos);
    const match = HEADING_LINE_RE.exec(line.text);
    if (!match) return null;

    const headingMarker = match[1];
    const textAfterHeading = match[2];

    // Check if heading already has an otid
    if (/^\[\[\d{13,16}\]\]/.test(textAfterHeading)) return null;

    const queryText = textAfterHeading.trim();
    const from = line.from + headingMarker.length + 1; // position after "## "

    // Build "Create New" options
    const createOptions: Completion[] = CREATE_OPTIONS.map(
      ({ kind, label, icon }) => ({
        label: `Create new ${label}`,
        displayLabel: `${icon} ${label}`,
        type: "class",
        section: { name: "Create New" },
        apply: (view: EditorView, _c: Completion, _from: number, _to: number) => {
          const newOtid = genTID();
          const insert = `[[${newOtid}]] `;
          view.dispatch({
            changes: { from, to: context.pos, insert },
            selection: { anchor: from + insert.length },
          });
          void config.onCreateChnot(newOtid, kind);
        },
      }),
    );

    // Build "Reference Existing" section
    let existingOptions: Completion[] = [];
    if (queryText.length > 0) {
      try {
        const excludeOtids = new Set([
          config.parentOtid,
          ...config.getExcludeOtids(),
        ]);
        const rsp = await chnotSearch({
          query: queryText,
          kinds: [],
          start_index: 0,
          page_size: 10,
        });
        existingOptions = rsp.data
          .filter((d) => !excludeOtids.has(d.meta.otid))
          .map((d) => ({
            label: d.title ?? String(d.meta.otid),
            displayLabel: `${d.meta.kind} — ${d.title ?? String(d.meta.otid)}`,
            detail: d.meta.kind,
            type: "variable",
            section: { name: "Reference Existing" },
            apply: (view: EditorView, _c: Completion, _from: number, _to: number) => {
              const insert = `[[${d.meta.otid}]] `;
              view.dispatch({
                changes: { from, to: context.pos, insert },
                selection: { anchor: from + insert.length },
              });
              config.onRefExisting(d.meta.otid, d.meta.kind);
            },
          }));
      } catch {
        // Search failed, skip
      }
    }

    if (existingOptions.length === 0) {
      existingOptions = [
        {
          label: "Type to search existing chnots...",
          displayLabel: "\u{1F50D} Type to search existing chnots...",
          type: "text",
          section: { name: "Reference Existing" },
          apply: "",
        },
      ];
    }

    return {
      from,
      options: [...createOptions, ...existingOptions],
      filter: false,
    };
  };
}
