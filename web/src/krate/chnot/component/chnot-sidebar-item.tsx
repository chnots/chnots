import React, { ForwardedRef } from "react";
import { chnotShortDate } from "@/lib/date-utils";
import Icon from "@/common/component/icon";
import { useChnotStore } from "@/krate/chnot/store/store";
import { Chnot } from "@/krate/chnot/store/dto";
import { chnotUpdate } from "@/krate/chnot/store/service";
import { ChnotKind } from "@/krate/chnot/store/db";
import {
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarMenuAction,
  useSidebar,
} from "@/common/component/ui/sidebar";
import { MoreHorizontal, Trash2 } from "lucide-react";
import clsx from "clsx";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/common/component/ui/dropdown-menu";
import { cn } from "@/lib/utils";

const ChnotSidebarTagItem = React.forwardRef(
  (
    {
      tag,
      focused,
      onClick,
    }: { tag: string; focused?: boolean; onClick: () => void },
    ref: ForwardedRef<HTMLLIElement>
  ) => {
    return (
      <SidebarMenuItem onClick={onClick} ref={ref}>
        <SidebarMenuButton
          size="lg"
          asChild
          onClick={onClick}
          className={cn(focused ? "border" : "border border-transparent")}
        >
          <div>
            <div className="flex flex-row text-xs m-2 space-x-2">
              <Icon.Hash className="h-4 w-4 min-w-4 text-green-600" />
              <div className="relative text-xs line-clamp-1 break-all">
                {tag.replace(RegExp("#"), "")}
              </div>
            </div>
          </div>
        </SidebarMenuButton>
      </SidebarMenuItem>
    );
  }
);

ChnotSidebarTagItem.displayName = "ChnotTagListItem";

const ChnotSidebarItem = React.forwardRef(
  ({ chnot }: { chnot: Chnot }, ref: ForwardedRef<HTMLLIElement>) => {
    const { setCurrentChnotMetaId, getCurrentChnot, validateChnotCache } =
      useChnotStore();

    const onClick = (_: React.MouseEvent) => {
      setCurrentChnotMetaId(chnot.meta.id);
    };

    const currentChnot = getCurrentChnot();

    const { isMobile } = useSidebar();

    const isSelected = currentChnot?.record.id === chnot.record.id;

    const title = chnot.record.content.startsWith("# ")
      ? chnot.record.content.split("\n")[0]
      : chnot.record.content.substring(0, 500);

    const onDelete = async () => {
      await chnotUpdate({
        meta_id: chnot.meta.id,
        archive: true,
        update_time: false,
      });
      validateChnotCache([chnot.meta.id]);
    };

    return (
      <SidebarMenuItem key={chnot.record.id}>
        <SidebarMenuButton
          size="lg"
          asChild
          onClick={onClick}
          className={clsx(isSelected ? "border" : "border border-transparent")}
        >
          <div>
            <div className="flex flex-row text-xs m-2 space-x-2">
              {chnot.meta.kind === ChnotKind.MarkdownWithToent && (
                <Icon.TextCursor className="h-4 w-4 min-w-4 text-blue-600" />
              )}
              {chnot.meta.kind === ChnotKind.ExcalidrawV1 && (
                <Icon.Pen className="h-4 w-4 min-w-4 text-red-600" />
              )}
              {chnot.meta.kind === ChnotKind.KFileV1 && (
                <Icon.File className="h-4 w-4 min-w-4 text-red-600" />
              )}
              {chnot.meta.kind === ChnotKind.KTab && (
                <Icon.Table className="h-4 w-4 min-w-4 text-purple-600" />
              )}
              {chnot.meta.kind === ChnotKind.LLMChat && (
                <Icon.Bot className="h-4 w-4 min-w-4 text-gray-600" />
              )}
              <div
                className="text-gray-600"
                title={chnot.meta.insert_time.toISOString()}
              >
                {chnotShortDate(chnot.meta.insert_time)}
              </div>
              <div className="relative line-clamp-2 break-all" title={title}>
                {title}
              </div>
            </div>
          </div>
        </SidebarMenuButton>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuAction showOnHover>
              <MoreHorizontal />
              <span className="sr-only">More</span>
            </SidebarMenuAction>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-48 rounded-lg"
            side={isMobile ? "bottom" : "right"}
            align={isMobile ? "end" : "start"}
          >
            <DropdownMenuItem onClick={onDelete}>
              <Trash2 className="text-muted-foreground" />
              <span>Delete</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    );
  }
);

ChnotSidebarItem.displayName = "ChnotListItem";

export { ChnotSidebarItem, ChnotSidebarTagItem };
