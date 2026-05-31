import {
  type EditorState,
  type Extension,
  StateEffect,
  StateField,
} from "@codemirror/state";
import {
  Decoration,
  type DecorationSet,
  EditorView,
  WidgetType,
} from "@codemirror/view";
import { createRoot } from "react-dom/client";

import { ChnotKind } from "@/krate/chnot/po";
import type { TID } from "@/lib/id_util";

import ThreadItemPreview from "./thread-item-preview";

export type ThreadWidgetItem = {
  otid: TID;
  kind: ChnotKind;
  headingLevel: number;
  titleLine: string;
  mdwtContent: string;
  kindData?: unknown;
};

export type ThreadWidgetData = {
  items: ThreadWidgetItem[];
  onItemClick: (otid: TID, kind: ChnotKind) => void;
};

const EMPTY_DATA: ThreadWidgetData = {
  items: [],
  onItemClick: () => {},
};

class ThreadItemWidget extends WidgetType {
  constructor(
    private item: ThreadWidgetItem,
    private onClick: (otid: TID, kind: ChnotKind) => void,
  ) {
    super();
  }

  eq(other: ThreadItemWidget): boolean {
    return (
      other.item.otid === this.item.otid &&
      other.item.mdwtContent === this.item.mdwtContent
    );
  }

  toDOM(): HTMLElement {
    const container = document.createElement("div");
    container.className = "cm-thread-item";
    const root = createRoot(container);
    root.render(
      <ThreadItemPreview item={this.item} onClick={this.onClick} />,
    );
    (container as any)._reactRoot = root;
    return container;
  }

  destroy(dom: HTMLElement) {
    const root = (dom as any)._reactRoot;
    root?.unmount();
  }

  ignoreEvent(event: Event): boolean {
    if (
      (event instanceof KeyboardEvent || event instanceof MouseEvent) &&
      this.item.kind === "ktabv1"
    ) {
      return true;
    }
    return false;
  }
}

export const threadDataField = StateField.define<ThreadWidgetData>({
  create: () => EMPTY_DATA,
  update: (val, tr) => {
    for (const effect of tr.effects) {
      if (effect.is(setThreadDataEffect)) {
        return effect.value;
      }
    }
    return val;
  },
});

const setThreadDataEffect = StateEffect.define<ThreadWidgetData>();

const threadDecorations = StateField.define<DecorationSet>({
  create(state) {
    return buildThreadDecorations(state);
  },
  update(decos, tr) {
    for (const effect of tr.effects) {
      if (effect.is(setThreadDataEffect)) {
        return buildThreadDecorations(tr.state);
      }
    }
    return decos.map(tr.changes);
  },
  provide: (f) => EditorView.decorations.from(f),
});

function findOtidSectionEnd(state: EditorState, headingLine: number): number {
  const lineCount = state.doc.lines;
  for (let i = headingLine + 1; i <= lineCount; i++) {
    const line = state.doc.line(i);
    if (/^\s{0,3}#{1,6}\s/.test(line.text)) {
      return state.doc.line(i - 1).to;
    }
    if (line.text.trim() === "") {
      return state.doc.line(i - 1).to;
    }
  }
  return state.doc.line(lineCount).to;
}

function buildThreadDecorations(state: EditorState): DecorationSet {
  const data = state.field(threadDataField, false) as
    | ThreadWidgetData
    | undefined;
  if (!data?.items.length) return Decoration.none;

  const lineCount = state.doc.lines;
  const endPos = state.doc.length;
  const widgets: import("@codemirror/state").Range<Decoration>[] = [];
  const inlineItems: ThreadWidgetItem[] = [];

  for (const item of data.items) {
    if (item.kind === ChnotKind.MDWT) continue;

    const otidStr = String(item.otid);
    let found = false;
    for (let i = 1; i <= lineCount; i++) {
      const line = state.doc.line(i);
      if (line.text.includes(`[[${otidStr}]]`)) {
        const pos = findOtidSectionEnd(state, i);
        widgets.push(
          Decoration.widget({
            widget: new ThreadItemWidget(item, data.onItemClick),
            block: true,
            side: 1,
          }).range(pos),
        );
        found = true;
        break;
      }
    }
    if (!found) inlineItems.push(item);
  }

  // Place inline items before the trailing empty line so there's always a
  // navigable line after each widget. The trailing empty line is guaranteed
  // by dispatchThreadData.
  const inlinePos = lineCount >= 2
    ? state.doc.line(lineCount - 1).to
    : endPos;
  for (let i = 0; i < inlineItems.length; i++) {
    widgets.push(
      Decoration.widget({
        widget: new ThreadItemWidget(inlineItems[i], data.onItemClick),
        block: true,
        side: 1,
      }).range(inlinePos),
    );
  }

  return widgets.length
    ? Decoration.set(widgets, true)
    : Decoration.none;
}

export function threadWidgetExtension(): Extension {
  return [threadDataField, threadDecorations];
}

export function dispatchThreadData(
  view: EditorView,
  data: ThreadWidgetData,
) {
  const doc = view.state.doc;
  if (data.items.length > 0) {
    // Ensure at least one trailing empty line so block widgets aren't placed
    // at the absolute end of the document, which would block cursor navigation.
    const trailing = doc.sliceString(Math.max(0, doc.length - 2));
    let insert: string | undefined;
    if (!trailing.endsWith("\n")) {
      insert = "\n\n";
    } else if (trailing !== "\n\n") {
      insert = "\n";
    }
    if (insert) {
      view.dispatch({
        effects: [setThreadDataEffect.of(data)],
        changes: { from: doc.length, insert },
      });
      return;
    }
  }
  view.dispatch({
    effects: [setThreadDataEffect.of(data)],
  });
}
