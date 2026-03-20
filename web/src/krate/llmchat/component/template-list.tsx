import { BadgePlus, MessageCircle, Pencil, Trash } from "lucide-react";
import KSVG from "@/common/component/svg";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/common/component/ui/context-menu";
import type { LLMChatTemplate } from "@/krate/llmchat/po";
import { useLLMChatStore } from "@/krate/llmchat/store";

const handleKeyDown = (e: React.KeyboardEvent, callback: () => void) => {
  if (e.key === "Enter" || e.key === " ") {
    e.preventDefault();
    callback();
  }
};

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
    <div className="flex flex-wrap gap-2 p-3 max-w-4xl">
      {[...templates.values()].map((item, index) => (
        <ContextMenu key={item.otid}>
          <ContextMenuTrigger>
            <div
              className="group flex items-start gap-2 px-2.5 py-2 rounded-lg border border-border bg-card hover:bg-accent/50 cursor-pointer transition-all duration-200 animate-in"
              style={{ animationDelay: `${index * 30}ms` }}
              role="button"
              tabIndex={0}
              onClick={() => {
                onSelectTemplate(item);
              }}
              onKeyDown={(e) => {
                handleKeyDown(e, () => onSelectTemplate(item));
              }}
            >
              <div className="shrink-0 w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center">
                {item.svg_logo ? (
                  <KSVG src={item.svg_logo} className="w-5 h-5" />
                ) : (
                  <MessageCircle className="w-5 h-5 text-primary" />
                )}
              </div>
              <div className="flex flex-col min-w-0">
                <span className="text-sm font-medium truncate">
                  {item.name}
                </span>
                {item.prompt && (
                  <span className="text-xs text-muted-foreground truncate max-w-40">
                    {item.prompt.replace(/\n/g, " ").slice(0, 30)}
                    {item.prompt.length > 30 ? "..." : ""}
                  </span>
                )}
              </div>
            </div>
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
      <div
        className="flex items-center gap-2 px-3 py-2 rounded-lg border border-dashed border-border hover:border-primary/50 cursor-pointer transition-colors text-muted-foreground hover:text-primary min-w-48"
        role="button"
        tabIndex={0}
        onClick={() => {
          onNew();
        }}
        onKeyDown={(e) => {
          handleKeyDown(e, onNew);
        }}
      >
        <BadgePlus className="w-4 h-4" />
        <span className="text-sm">New Template</span>
      </div>
    </div>
  );
};

export default LLMChatTemplateList;
