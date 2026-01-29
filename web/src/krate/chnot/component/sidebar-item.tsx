import { MoreHorizontal } from "lucide-react";
import React, { type ForwardedRef, memo } from "react";
import Icon from "@/common/component/icon";
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
import {
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/common/component/ui/sidebar";
import type { StateChnotLike } from "@/krate/chnot/store";
import {
  KSpaceIcon,
  KSpaceSelectDropDownGroup,
} from "@/krate/kspace/component/kspace-select";
import { chnotShortDate } from "@/lib/date-utils";
import type { TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import type { ChnotKind } from "../po";
import { ChnotKindIcon } from "./kind-icon";

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
                {tag.replace(/#/, "")}
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
      isCurrent,
      kind,
      setCurOtid,
      unvalidate,
      onArchive,
      onTogglePin,
      onChangeKspace,
    }: {
      item: StateChnotLike;
      showKSpace: boolean;
      isCurrent?: boolean;
      kind?: ChnotKind;
      onArchive(otid: TID): void;
      onTogglePin(otid: TID): void;
      setCurOtid(cutOtid?: TID): void;
      unvalidate(toRemoves: TID[]): void;
      onChangeKspace(otid: TID, kspace: string): Promise<void>;
    },
    _ref: ForwardedRef<HTMLLIElement>,
  ) => {
    const { isMobile } = useSidebar();

    const title = item.title?.startsWith("# ")
      ? item.title.split("\n")[0].substring(2)
      : (item.title?.substring(0, 500) ?? "<unknown>");

    return (
      <SidebarMenuItem key={item.meta.otid}>
        <a
          href={`#${item.meta.otid}`}
          key={item.meta.otid}
          onClick={() => {
            setCurOtid(item.meta.otid);
          }}
          className={cn(
            "group flex items-start gap-2 p-2 rounded-md transition-colors duration-150",
            "hover:shadow-xs border",
            isCurrent ? "bg-background" : "bg-transparent border-transparent",
          )}
          tabIndex={0}
          aria-label={`Navigate to ${title}`}
        >
          <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
            <time
              dateTime={new Date(item.meta.otid / 1e3).toISOString()}
              className="text-[0.7rem] whitespace-nowrap"
            >
              {chnotShortDate(new Date(item.meta.otid / 1e3))}
            </time>
            {kind && <ChnotKindIcon kind={kind} className="h-3.5 w-3.5" />}
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
              isCurrent ? "text-sidebar-accent-foreground" : "text-foreground",
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
            <DropdownMenuItem onClick={() => onTogglePin(item.meta.otid)}>
              <Icon.Pin className="text-muted-foreground" />
              <span>Pin</span>
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onArchive(item.meta.otid)}>
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
                      onChangeKspace(item.meta.otid, e).then(() => {
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

const ChnotSidebarItemMemo = memo(ChnotSidebarItem);

export { ChnotSidebarItemMemo, ChnotSidebarTagItem };
