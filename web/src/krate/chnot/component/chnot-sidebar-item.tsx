import React, { ForwardedRef } from "react";
import { chnotShortDate } from "@/lib/date-utils";
import Icon from "@/common/component/icon";
import { useChnotStore } from "@/krate/chnot/store/store";
import { Chnot } from "@/krate/chnot/store/dto";
import { chnotUpdate } from "@/krate/chnot/store/service";
import { ChnotKind } from "@/krate/chnot/store/po";
import {
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarMenuAction,
  useSidebar,
} from "@/common/component/ui/sidebar";
import { MoreHorizontal, Trash2 } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/common/component/ui/dropdown-menu";
import { cn } from "@/lib/utils";
import { KSpaceIcon } from "@/krate/kspace/component/kspace-select";

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
  (
    { chnot, showKSpace }: { chnot: Chnot; showKSpace: boolean },
    ref: ForwardedRef<HTMLLIElement>
  ) => {
    const { setCurrentChnotMetaId, getCurrentChnot, validateChnotCache } =
      useChnotStore();

    const onClick = (_: React.MouseEvent) => {
      setCurrentChnotMetaId(chnot.meta.tid);
    };

    const currentChnot = getCurrentChnot();

    const { isMobile } = useSidebar();

    const isSelected = currentChnot?.record.tid === chnot.record.tid;

    const title = chnot.record.content.startsWith("# ")
      ? chnot.record.content.split("\n")[0].substring(2)
      : chnot.record.content.substring(0, 500);

    const onDelete = async () => {
      await chnotUpdate({
        meta_tid: chnot.meta.tid,
        archive: true,
        update_time: false,
      });
      validateChnotCache([chnot.meta.tid]);
    };

    return (
      <SidebarMenuItem key={chnot.record.tid}>
        <a
          href={"#" + chnot.meta.tid}
          key={chnot.meta.tid}
          onClick={onClick}
          className={cn(
            "group flex items-start gap-2 p-2 rounded-md transition-colors duration-150",
            "hover:shadow-xs border",
            isSelected ? "bg-background" : "bg-transparent border-transparent"
          )}
          tabIndex={0}
          aria-label={`Navigate to ${title}`}
        >
          <div
            className={cn(
              "flex-shrink-0 p-1.5 rounded",
              "text-muted-foreground group-hover:text-sidebar-accent-foreground",
              isSelected
                ? "text-sidebar-accent-foreground"
                : "text-muted-foreground"
            )}
          >
            {chnot.meta.kind === ChnotKind.MarkdownWithToent ? (
              <Icon.Text className="h-4 w-4" />
            ) : chnot.meta.kind === ChnotKind.ExcalidrawV1 ? (
              <Icon.Flower className="h-4 w-4" />
            ) : chnot.meta.kind === ChnotKind.KFileV1 ? (
              <Icon.File className="h-4 w-4" />
            ) : chnot.meta.kind === ChnotKind.KTab ? (
              <Icon.Table className="h-4 w-4" />
            ) : chnot.meta.kind === ChnotKind.LLMChat ? (
              <Icon.Bot className="h-4 w-4" />
            ) : (
              <Icon.TextCursor className="h-4 w-4" />
            )}
          </div>

          <div className="flex-1 min-w-0 space-y-0.5">
            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
              {showKSpace && (
                <KSpaceIcon
                  name={chnot.meta.kspace}
                  className="h-3.5 w-3.5 text-muted-foreground/60"
                />
              )}
              <time
                dateTime={new Date(chnot.meta.tid).toISOString()}
                className="text-[0.7rem]"
              >
                {chnotShortDate(new Date(chnot.meta.tid))}
              </time>
            </div>

            <h3
              className={cn(
                "text-xs font-medium line-clamp-2 leading-tight",
                "text-foreground group-hover:text-sidebar-accent-foreground",
                isSelected
                  ? "text-sidebar-accent-foreground"
                  : "text-foreground"
              )}
              title={title}
            >
              {title}
            </h3>
          </div>
        </a>
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
