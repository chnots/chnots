import { createElement } from "react";
import type { ChnotKind } from "@/krate/chnot/po";
import ExcalidrawPreview from "@/krate/graph/excalidraw/component/excalidraw-preview";
import type { ExcalidrawChnotState } from "@/krate/graph/excalidraw/service";
import MindElixirPreview from "@/krate/graph/mind-elixir/preview";
import type { MindElixirData } from "@/krate/graph/mind-elixir";
import type { TID } from "@/lib/id_util";

import { MarkdownViewer } from "./markdown-viewer";
import type { ThreadWidgetItem } from "./thread-widget-extension";

const HEADING_TAGS = ["h1", "h2", "h3", "h4", "h5", "h6"] as const;

const ThreadItemPreview = ({
  item,
  onClick,
}: {
  item: ThreadWidgetItem;
  onClick: (otid: TID, kind: ChnotKind) => void;
}) => {
  const level = Math.min(Math.max(item.headingLevel, 1), 6);
  const Tag = HEADING_TAGS[level - 1];

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

  return (
    <div
      className="cm-thread-item-preview cursor-pointer hover:bg-accent/50 rounded p-2 border-b last:border-b-0"
      onClick={() => onClick(item.otid, item.kind)}
    >
      {createElement(Tag, { className: "text-lg font-semibold" },
        `[[${item.otid}]] ${item.titleLine}`,
      )}
      {item.mdwtContent && <MarkdownViewer content={item.mdwtContent} />}
      {renderKindPreview()}
    </div>
  );
};

export default ThreadItemPreview;
