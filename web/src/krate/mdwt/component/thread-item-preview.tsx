import type { ChnotKind } from "@/krate/chnot/po";
import ExcalidrawPreview from "@/krate/graph/excalidraw/component/excalidraw-preview";
import type { ExcalidrawChnotState } from "@/krate/graph/excalidraw/service";
import MindElixirPreview from "@/krate/graph/mind-elixir/preview";
import type { MindElixirData } from "@/krate/graph/mind-elixir";
import type { KTabMeta } from "@/krate/ktab/po";
import type { TID } from "@/lib/id_util";

import KTabInlineWidget from "./ktab-inline-widget";
import LLMChatInlineWidget from "./llmchat-inline-preview";
import type { LLMChatPreviewData } from "./llmchat-inline-preview";
import { MarkdownViewer } from "./markdown-viewer";
import type { ThreadWidgetItem } from "./thread-widget-extension";

const ThreadItemPreview = ({
  item,
  onClick,
}: {
  item: ThreadWidgetItem;
  onClick: (otid: TID, kind: ChnotKind) => void;
}) => {
  if (item.kind === "ktabv1") {
    if (!item.kindData) {
      return (
        <div className="cm-thread-item-preview rounded border p-3 my-2 mx-1">
          {item.mdwtContent && <MarkdownViewer content={item.mdwtContent} />}
        </div>
      );
    }
    return (
      <KTabInlineWidget
        otid={item.otid}
        kindData={item.kindData as KTabMeta}
        onItemClick={onClick}
      />
    );
  }

  if (item.kind === "llm_chat") {
    if (!item.kindData) {
      return (
        <div className="cm-thread-item-preview rounded border p-3 my-2 mx-1">
          {item.mdwtContent && <MarkdownViewer content={item.mdwtContent} />}
        </div>
      );
    }
    return (
      <LLMChatInlineWidget
        otid={item.otid}
        kindData={item.kindData as LLMChatPreviewData}
        onItemClick={onClick}
      />
    );
  }

  const renderKindPreview = () => {
    if (!item.kindData) return null;

    switch (item.kind) {
      case "exdrv1":
        return (
          <ExcalidrawPreview
            state={item.kindData as ExcalidrawChnotState}
            className="w-full"
          />
        );
      case "mindmapv1":
        return <MindElixirPreview data={item.kindData as MindElixirData} />;
      default:
        return null;
    }
  };

  const kindPreview = renderKindPreview();

  return (
    <div
      className="cm-thread-item-preview cursor-pointer hover:bg-accent/50 rounded border p-3 my-2 mx-1"
      onClick={() => onClick(item.otid, item.kind)}
    >
      {item.mdwtContent && <MarkdownViewer content={item.mdwtContent} />}
      {kindPreview}
    </div>
  );
};

export default ThreadItemPreview;
