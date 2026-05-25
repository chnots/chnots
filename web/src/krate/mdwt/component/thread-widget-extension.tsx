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

import type { ChnotKind } from "@/krate/chnot/po";
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

  ignoreEvent() {
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

function buildThreadDecorations(state: EditorState): DecorationSet {
  const data = state.field(threadDataField, false) as
    | ThreadWidgetData
    | undefined;
  if (!data?.items.length) return Decoration.none;

  const endPos = state.doc.length;
  const widgets = data.items.map((item, i) =>
    Decoration.widget({
      widget: new ThreadItemWidget(item, data.onItemClick),
      block: true,
      side: 1,
    }).range(endPos + i),
  );

  return Decoration.set(widgets, true);
}

export function threadWidgetExtension(): Extension {
  return [threadDataField, threadDecorations];
}

export function dispatchThreadData(
  view: EditorView,
  data: ThreadWidgetData,
) {
  view.dispatch({
    effects: [setThreadDataEffect.of(data)],
  });
}
