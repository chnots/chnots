import React, { ForwardedRef } from "react";
import { chnotShortDate } from "@/lib/date-utils";
import Icon from "@/common/component/icon";
import { useChnotStore } from "@/krate/chnot/store";
import { ChnotThread } from "@/krate/chnot/dto";
import { chnotOverwriteMeta } from "@/krate/chnot/service";
import { ChnotKind } from "@/krate/chnot/po";
import {
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarMenuAction,
  useSidebar,
} from "@/common/component/ui/sidebar";
import { MoreHorizontal } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/common/component/ui/dropdown-menu";
import { cn } from "@/lib/utils";
import {
  KSpaceIcon,
  KSpaceSelectDropDownGroup,
} from "@/krate/kspace/component/kspace-select";
import { ChnotKindIcon } from "./chnot-kind-icon";
import { useShallow } from "zustand/react/shallow";
import TodoLabel from "@/krate/toent/component/todo-label";

const ChnotSidebarTagItem = React.forwardRef(
  (
    {
      tag,
      focused,
      onClick,
    }: { tag: string; focused?: boolean; onClick: () => void },
    ref: ForwardedRef<HTMLLIElement>,
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
  },
);

ChnotSidebarTagItem.displayName = "ChnotThreadTagListItem";

const ChnotSidebarItem = React.forwardRef(
  (
    { chnot, showKSpace }: { chnot: ChnotThread; showKSpace: boolean },
    ref: ForwardedRef<HTMLLIElement>,
  ) => {
    const {
      setCurrentChnotMetaId,
      getCurrentChnot,
      validateChnotCache,
      overwriteChnotCache,
    } = useChnotStore(
      useShallow((store) => {
        return {
          overwriteChnotCache: store.overwriteChnotCache,
          setCurrentChnotMetaId: store.setCurrentChnotMetaId,
          getCurrentChnot: store.getCurrentChnot,
          validateChnotCache: store.validateChnotCache,
        };
      }),
    );

    const onClick = (_: React.MouseEvent) => {
      setCurrentChnotMetaId(chnot.meta.otid);
    };

    const currentChnot = getCurrentChnot();

    const { isMobile } = useSidebar();

    const isSelected = currentChnot?.head_record.tid === chnot.head_record.tid;

    const title = chnot.head_record.content.startsWith("# ")
      ? chnot.head_record.content.split("\n")[0].substring(2)
      : chnot.head_record.content.substring(0, 500);

    const onArchive = async () => {
      await chnotOverwriteMeta({
        meta_otid: chnot.meta.otid,
        archive: true,
      });
      validateChnotCache([chnot.meta.otid]);
    };

    const onTogglePin = async () => {
      const pin = chnot.meta.pin_time ? false : true;
      await chnotOverwriteMeta({
        meta_otid: chnot.meta.otid,
        pinned: pin,
      });
      chnot.meta.pin_time = pin ? new Date() : undefined;
      overwriteChnotCache(chnot);
    };

    return (
      <SidebarMenuItem key={chnot.head_record.tid}>
        <a
          href={"#" + chnot.meta.otid}
          key={chnot.meta.otid}
          onClick={onClick}
          className={cn(
            "group flex items-start gap-2 p-2 rounded-md transition-colors duration-150",
            "hover:shadow-xs border",
            isSelected ? "bg-background" : "bg-transparent border-transparent",
          )}
          tabIndex={0}
          aria-label={`Navigate to ${title}`}
        >
          <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
            <time
              dateTime={new Date(chnot.meta.otid / 1e3).toISOString()}
              className="text-[0.7rem]"
            >
              {chnotShortDate(new Date(chnot.meta.otid / 1e3))}
            </time>
            {showKSpace && (
              <KSpaceIcon
                name={chnot.meta.kspace}
                className="h-3.5 w-3.5 text-muted-foreground/60"
              />
            )}
            {chnot.meta.pin_time && (
              <Icon.Pin className="h-3.5 w-3.5 text-red-900" />
            )}
            {chnot.head_record.todo_event && (
              <TodoLabel todoEvent={chnot.head_record.todo_event} />
            )}
          </div>

          <h3
            className={cn(
              "text-xs font-medium line-clamp-2 leading-tight break-all",
              "text-foreground group-hover:text-sidebar-accent-foreground",
              isSelected ? "text-sidebar-accent-foreground" : "text-foreground",
            )}
            title={title}
          >
            {title}
          </h3>
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
            <DropdownMenuItem onClick={onTogglePin}>
              <Icon.Pin className="text-muted-foreground" />
              <span>Pin</span>
            </DropdownMenuItem>
            <DropdownMenuItem onClick={onArchive}>
              <Icon.Trash2 className="text-muted-foreground" />
              <span>Archive</span>
            </DropdownMenuItem>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger className="space-x-2">
                <Icon.Warehouse className="text-muted-foreground w-4 h-4" />
                <span>Workspace</span>
              </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <KSpaceSelectDropDownGroup
                    kspace={chnot.meta.kspace}
                    onSelect={(e) => {
                      chnotOverwriteMeta({
                        meta_otid: chnot.meta.otid,
                        kspace: e,
                      }).then(() => {
                        validateChnotCache([chnot.meta.otid]);
                      });
                    }}
                  />
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>{" "}
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    );
  },
);

ChnotSidebarItem.displayName = "ChnotListItem";

export { ChnotSidebarItem, ChnotSidebarTagItem };
