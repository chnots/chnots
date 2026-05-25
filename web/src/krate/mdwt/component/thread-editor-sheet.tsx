import { X } from "lucide-react";
import type { TID } from "@/lib/id_util";
import { Button } from "@/common/component/ui/button";
import { ChnotKind } from "@/krate/chnot/po";
import ExcalidrawChnot from "@/krate/chnot/component/rich-chnot/excalidraw";
import KFileChnot from "@/krate/chnot/component/rich-chnot/kfile";
import LLMChatChnot from "@/krate/chnot/component/rich-chnot/llmchat";
import MdwtChnot from "@/krate/chnot/component/rich-chnot/mdwt";
import MindMapChnot from "@/krate/chnot/component/rich-chnot/mindmap";
import TableChnot from "@/krate/chnot/component/rich-chnot/table";
import type { PostSaveArg } from "@/krate/chnot/component/rich-chnot/types";

const KIND_LABELS: Record<ChnotKind, string> = {
  [ChnotKind.MDWT]: "Markdown",
  [ChnotKind.ExcalidrawV1]: "Excalidraw",
  [ChnotKind.KFileV1]: "File",
  [ChnotKind.KTab]: "Table",
  [ChnotKind.LLMChat]: "LLM Chat",
  [ChnotKind.MindMapV1]: "Mind Map",
  [ChnotKind.ThreadV1]: "Thread",
};

const ThreadEditorPanel = ({
  item,
  onClose,
  onSaved,
}: {
  item: { otid: TID; kind: ChnotKind };
  onClose: () => void;
  onSaved: () => void;
}) => {
  const handlePostSave = async (_arg: PostSaveArg) => {
    onSaved();
  };

  const renderEditor = () => {
    const commonProps = {
      otid: item.otid,
      readonly: false,
      fullscreen: false,
      onPostSave: handlePostSave,
      disableHeaderActions: true,
    };

    switch (item.kind) {
      case ChnotKind.MDWT:
        return <MdwtChnot {...commonProps} fillParentHeight />;
      case ChnotKind.LLMChat:
        return <LLMChatChnot {...commonProps} />;
      case ChnotKind.ExcalidrawV1:
        return <ExcalidrawChnot {...commonProps} showEditWhenEmpty />;
      case ChnotKind.KTab:
        return <TableChnot {...commonProps} />;
      case ChnotKind.KFileV1:
        return <KFileChnot {...commonProps} />;
      case ChnotKind.MindMapV1:
        return <MindMapChnot {...commonProps} showEditWhenEmpty />;
      default:
        return <div className="p-4">Unsupported kind: {item.kind}</div>;
    }
  };

  return (
    <div className="w-1/2 h-full flex flex-col border-l shrink-0">
      <div className="flex items-center justify-between px-3 py-1 border-b">
        <span className="text-sm font-medium">
          {KIND_LABELS[item.kind] ?? item.kind}
        </span>
        <Button variant="ghost" size="icon" onClick={onClose}>
          <X className="size-4" />
        </Button>
      </div>
      <div className="flex-1 min-h-0">
        {renderEditor()}
      </div>
    </div>
  );
};

export default ThreadEditorPanel;
