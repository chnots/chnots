import React, { ForwardedRef } from "react";
import { chnotShortDate } from "@/lib/date-utils";
import Icon from "@/common/component/icon";
import { StateChnotLike } from "@/krate/chnot/store";
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
import { chnotThreadMetaOverwrite } from "../service";
import { genTID, TID } from "@/lib/id_util";

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

ChnotSidebarTagItem.displayName = "ChnotSidebarTagItem";

const ChnotSidebarItem = React.forwardRef(
  (
    {
      item,
      showKSpace,
      curOtid,
      getCurrent,
      overwrite,
      setCurrOtid,
      unvalidate,
    }: {
      item: StateChnotLike;
      showKSpace: boolean;
      curOtid?: TID;
      getCurrent(): StateChnotLike | undefined;
      overwrite(chnot: StateChnotLike): void;
      setCurrOtid(cutOtid?: TID): void;
      unvalidate(toRemoves: TID[]): void;
    },
    ref: ForwardedRef<HTMLLIElement>,
  ) => {
    const { isMobile } = useSidebar();

    const isSelected = curOtid === item.meta.otid;

    const title = item.title?.startsWith("# ")
      ? item.title.split("\n")[0].substring(2)
      : (item.title?.substring(0, 500) ?? "<unknown>");

    const onArchive = async () => {
      await chnotThreadMetaOverwrite({
        meta_otid: item.meta.otid,
        archive: true,
      });
      unvalidate([item.meta.otid]);
    };

    const onTogglePin = async () => {
      const pin = item.meta.pin_tid ? false : true;
      await chnotThreadMetaOverwrite({
        meta_otid: item.meta.otid,
        pinned: pin,
      });
      item.meta.pin_tid = pin ? genTID() : undefined;
      overwrite(item);
    };

    return (
      <SidebarMenuItem key={item.meta.otid}>
        <a
          href={"#" + item.meta.otid}
          key={item.meta.otid}
          onClick={() => {
            setCurrOtid(item.meta.otid);
          }}
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
              dateTime={new Date(item.meta.otid / 1e3).toISOString()}
              className="text-[0.7rem] break-keep"
            >
              {chnotShortDate(new Date(item.meta.otid / 1e3))}
            </time>
            {showKSpace && (
              <KSpaceIcon
                name={item.meta.kspace}
                className="h-3.5 w-3.5 text-muted-foreground/60"
              />
            )}
            {item.meta.pin_tid && (
              <Icon.Pin className="h-3.5 w-3.5 text-red-900" />
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
                    kspace={item.meta.kspace}
                    onSelect={(e) => {
                      chnotThreadMetaOverwrite({
                        meta_otid: item.meta.otid,
                        kspace: e,
                      }).then(() => {
                        unvalidate([item.meta.otid]);
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
