import { BadgePlus, MessageCircle, Pencil, Trash } from "lucide-react";
import KSVG from "@/common/component/svg";
import { Button } from "@/common/component/ui/button";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/common/component/ui/context-menu";
import type { LLMChatTemplate } from "@/krate/llmchat/po";
import { useLLMChatStore } from "@/krate/llmchat/store";

const LLMChatTemplateList = ({
  onSelectTemplate,
  onNew,
  onEditTemplate,
  onDeleteTemplate,
}: {
  onSelectTemplate: (template: LLMChatTemplate) => void;
  onNew: () => void;
  onEditTemplate?: (template: LLMChatTemplate) => void;
  onDeleteTemplate?: (template: LLMChatTemplate) => void;
}) => {
  const { templates } = useLLMChatStore();

  return (
    <div className="flex flex-wrap space-x-4 text-sm p-3 max-w-4xl space-y-2">
      {[...templates.values()].map((item) => (
        <ContextMenu key={item.otid}>
          <ContextMenuTrigger>
            <Button
              className={
                "hover:cursor-pointer space-x-1 items-center flex py-1"
              }
              onClick={() => {
                onSelectTemplate(item);
              }}
            >
              {item.svg_logo ? (
                <KSVG src={item.svg_logo} className="w-4 h-4" />
              ) : (
                <MessageCircle className="w-4 h-4" />
              )}
              <span>{item.name}</span>
            </Button>
          </ContextMenuTrigger>
          <ContextMenuContent>
            <ContextMenuItem
              onClick={() => {
                onEditTemplate?.(item);
              }}
            >
              <Pencil className="mr-2 h-4 w-4" />
              Edit
            </ContextMenuItem>
            <ContextMenuItem
              variant="destructive"
              onClick={() => {
                onDeleteTemplate?.(item);
              }}
            >
              <Trash className="mr-2 h-4 w-4" />
              Delete
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      ))}
      <Button
        key={"add-new"}
        className={"hover:cursor-pointer space-x-1 items-center flex py-1"}
        onClick={() => {
          onNew();
        }}
      >
        <BadgePlus />
      </Button>
    </div>
  );
};

export default LLMChatTemplateList;
