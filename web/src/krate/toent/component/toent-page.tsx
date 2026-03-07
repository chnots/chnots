import dayjs from "dayjs";
import {
  Brain,
  Calendar,
  CalendarRange,
  CheckCircle2,
  ChevronLeft,
  ChevronRight,
  Circle,
  Hash,
  Inbox,
  Search,
  Settings,
  Sun,
} from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { NavLink } from "react-router-dom";
import { Badge } from "@/common/component/ui/badge";
import { Button } from "@/common/component/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/common/component/ui/select";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@/common/component/ui/sheet";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInput,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarSeparator,
  SidebarTrigger,
} from "@/common/component/ui/sidebar";
import { Switch } from "@/common/component/ui/switch";
import { Toggle } from "@/common/component/ui/toggle";
import { SaveState } from "@/common/types";
import MdwtChnot from "@/krate/chnot/component/rich-chnot/mdwt";
import type { MdwtTagSearchType } from "@/krate/chnot/dto";
import { ChnotKind } from "@/krate/chnot/po";
import { chnotMetaCommit } from "@/krate/chnot/service";
import { KSpaceSelect } from "@/krate/kspace/component/kspace-select";
import { useKSpaceStore } from "@/krate/kspace/store";
import { chnotTagNameList } from "@/krate/mdwt/service";
import type {
  TodoPriorityEnum,
  TodoStateEnum,
  ToentScheduleItem,
} from "@/krate/toent/po";
import {
  toentInstCount,
  toentSearch,
  toentTodoStateCommit,
} from "@/krate/toent/service";
import { parseNaiveDateTime } from "@/lib/date-utils";
import type { TID } from "@/lib/id_util";
import { RoutePaths } from "@/router";

type ToentViewMode = "list" | "week" | "month";
type SidebarMode = "month" | "week" | "today" | "all";
const TODO_STATES: TodoStateEnum[] = [
  "TODO",
  "DOING",
  "WAIT",
  "DONE",
  "CANCEL",
];
const TODO_STATE_LABEL: Record<TodoStateEnum, string> = {
  TODO: "TODO",
  DOING: "DOING",
  WAIT: "WAIT",
  DONE: "DONE",
  CANCEL: "CANCEL",
};
const TODO_STATE_COLOR: Record<
  TodoStateEnum,
  { badge: string; dot: string; select: string }
> = {
  TODO: {
    badge: "border-blue-200 bg-blue-50 text-blue-700",
    dot: "bg-blue-500",
    select: "border-blue-200 bg-blue-50/70 text-blue-700",
  },
  DOING: {
    badge: "border-amber-200 bg-amber-50 text-amber-700",
    dot: "bg-amber-500",
    select: "border-amber-200 bg-amber-50/70 text-amber-700",
  },
  WAIT: {
    badge: "border-violet-200 bg-violet-50 text-violet-700",
    dot: "bg-violet-500",
    select: "border-violet-200 bg-violet-50/70 text-violet-700",
  },
  DONE: {
    badge: "border-emerald-200 bg-emerald-50 text-emerald-700",
    dot: "bg-emerald-500",
    select: "border-emerald-200 bg-emerald-50/70 text-emerald-700",
  },
  CANCEL: {
    badge: "border-rose-200 bg-rose-50 text-rose-700",
    dot: "bg-rose-500",
    select: "border-rose-200 bg-rose-50/70 text-rose-700",
  },
};

const FILTER_META: Record<SidebarMode, { title: string; description: string }> =
  {
    today: { title: "Today", description: "Only reminders for today" },
    all: { title: "All items", description: "All reminders in current range" },
    month: { title: "Month", description: "View reminders by month" },
    week: { title: "Week", description: "View reminders by week" },
  };

type NormalizedScheduleItem = ToentScheduleItem & {
  parsedTime: Date;
  dayKey: string;
};

type ToentEditorTarget = {
  otid: TID;
  title: string;
  dateText: string;
};

function ToentPage() {
  const [activeMode, setActiveMode] = useState<SidebarMode>("month");
  const [modeCounts, setModeCounts] = useState<Record<SidebarMode, number>>({
    month: 0,
    week: 0,
    today: 0,
    all: 0,
  });
  const [searchTerm, setSearchTerm] = useState("");
  const [tags, setTags] = useState<MdwtTagSearchType>();
  const [tagOptions, setTagOptions] = useState<string[]>([]);
  const [enabledStates, setEnabledStates] = useState<
    Record<TodoStateEnum, boolean>
  >({
    TODO: true,
    DOING: true,
    WAIT: true,
    DONE: true,
    CANCEL: true,
  });
  const [cursorDate, setCursorDate] = useState(() => startOfDay(new Date()));
  const [scheduleItems, setScheduleItems] = useState<ToentScheduleItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | undefined>(undefined);
  const [editorTarget, setEditorTarget] = useState<ToentEditorTarget>();
  const [statusUpdatingTid, setStatusUpdatingTid] = useState<TID>();
  const [refreshTick, setRefreshTick] = useState(0);
  const ensuredMetaOtidsRef = useRef<Set<TID>>(new Set());

  const { currentKSpace, mkspaces, selectKSpace } = useKSpaceStore((store) => {
    return {
      currentKSpace: store.currentKSpace,
      mkspaces: store.mkspaces,
      selectKSpace: store.selectKSpace,
    };
  });

  const includeCompleted = enabledStates.DONE;
  const includeUncompleted =
    enabledStates.TODO ||
    enabledStates.DOING ||
    enabledStates.WAIT ||
    enabledStates.CANCEL;

  useEffect(() => {
    let cancelled = false;

    const loadTagOptions = async () => {
      if (!tags) {
        setTagOptions([]);
        return;
      }

      try {
        const rsp = await chnotTagNameList({
          start_index: 0,
          page_size: 9999,
          tags,
          query: searchTerm,
        });
        if (!cancelled) {
          setTagOptions(rsp.data);
        }
      } catch {
        if (!cancelled) {
          setTagOptions([]);
        }
      }
    };

    loadTagOptions();
    return () => {
      cancelled = true;
    };
  }, [mkspaces, searchTerm, tags, currentKSpace]);

  useEffect(() => {
    let cancelled = false;
    const loadAllModeCounts = async () => {
      const modes: SidebarMode[] = ["month", "week", "today", "all"];
      const entries = await Promise.all(
        modes.map(async (mode) => {
          const { startDate, endDate } = getQueryRange(mode, cursorDate);
          try {
            const rsp = await toentInstCount({
              start_date: startDate,
              end_date: endDate,
              include_completed: includeCompleted,
              include_uncompleted: includeUncompleted,
            });
            return [mode, rsp.total] as const;
          } catch {
            return [mode, 0] as const;
          }
        }),
      );

      if (!cancelled) {
        setModeCounts((prev) => ({ ...prev, ...Object.fromEntries(entries) }));
      }
    };

    loadAllModeCounts();
    return () => {
      cancelled = true;
    };
  }, [
    cursorDate,
    includeCompleted,
    includeUncompleted,
    mkspaces,
    refreshTick,
    currentKSpace,
  ]);

  const viewMode: ToentViewMode =
    activeMode === "week" || activeMode === "month" ? activeMode : "list";

  useEffect(() => {
    if (activeMode === "today") {
      setCursorDate(startOfDay(new Date()));
    }
  }, [activeMode]);

  useEffect(() => {
    let cancelled = false;
    const loadCount = async () => {
      const { startDate, endDate } = getQueryRange(activeMode, cursorDate);
      try {
        const rsp = await toentInstCount({
          start_date: startDate,
          end_date: endDate,
          include_completed: includeCompleted,
          include_uncompleted: includeUncompleted,
        });

        if (!cancelled) {
          setModeCounts((prev) => ({ ...prev, [activeMode]: rsp.total }));
        }
      } catch {
        if (!cancelled) {
          setModeCounts((prev) => ({ ...prev, [activeMode]: 0 }));
        }
      }
    };

    loadCount();
    return () => {
      cancelled = true;
    };
  }, [
    activeMode,
    cursorDate,
    includeCompleted,
    includeUncompleted,
    mkspaces,
    refreshTick,
    currentKSpace,
  ]);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      setLoading(true);
      const { startDate, endDate } = getQueryRange(activeMode, cursorDate);
      const pageSize = 200;

      try {
        const mergedItems: ToentScheduleItem[] = [];
        let startIndex = 0;
        let hasNext = true;
        let guard = 0;

        while (hasNext && guard < 20) {
          guard += 1;
          const rsp = await toentSearch({
            start_date: startDate,
            end_date: endDate,
            include_completed: includeCompleted,
            include_uncompleted: includeUncompleted,
            query: searchTerm.trim() || undefined,
            tags,
            start_index: startIndex,
            page_size: pageSize,
          });
          mergedItems.push(...rsp.items);
          hasNext = rsp.has_next;
          startIndex = rsp.next_start;
        }

        if (cancelled) {
          return;
        }
        setScheduleItems(mergedItems);
        setLoadError(undefined);
      } catch {
        if (cancelled) {
          return;
        }
        setScheduleItems([]);
        setLoadError("Failed to load. Please try again later.");
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    load();
    return () => {
      cancelled = true;
    };
  }, [
    activeMode,
    cursorDate,
    includeCompleted,
    includeUncompleted,
    mkspaces,
    tags,
    searchTerm,
    refreshTick,
    currentKSpace,
  ]);

  const normalizedItems = useMemo<NormalizedScheduleItem[]>(() => {
    return scheduleItems
      .map((item) => {
        const parsedTime = parseNaiveDateTime(item.inst.naive_time);
        if (!parsedTime) {
          return undefined;
        }

        return {
          ...item,
          parsedTime,
          dayKey: dateToDayKey(parsedTime),
        };
      })
      .filter((item): item is NormalizedScheduleItem => item !== undefined)
      .sort((a, b) => a.parsedTime.getTime() - b.parsedTime.getTime());
  }, [scheduleItems]);

  const stateCounts = useMemo(() => {
    const initial: Record<TodoStateEnum, number> = {
      TODO: 0,
      DOING: 0,
      WAIT: 0,
      DONE: 0,
      CANCEL: 0,
    };
    for (const item of normalizedItems) {
      const state = item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
      initial[state] += 1;
    }
    return initial;
  }, [normalizedItems]);

  const stateFilteredItems = useMemo(() => {
    return normalizedItems.filter((item) => {
      const state = item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
      return enabledStates[state];
    });
  }, [normalizedItems, enabledStates]);

  const searchedItems = stateFilteredItems;

  const groupedByDay = useMemo(() => {
    const group: Record<string, NormalizedScheduleItem[]> = {};
    for (const item of searchedItems) {
      group[item.dayKey] ??= [];
      group[item.dayKey].push(item);
    }
    return group;
  }, [searchedItems]);

  const listDayGroups = useMemo(() => {
    const groups: Array<{ dayKey: string; items: NormalizedScheduleItem[] }> =
      [];
    for (const [dayKey, items] of Object.entries(groupedByDay)) {
      groups.push({ dayKey, items });
    }
    return groups.sort((a, b) => a.dayKey.localeCompare(b.dayKey));
  }, [groupedByDay]);

  const weekStart = getWeekStartMonday(cursorDate);
  const weekDates = useMemo(() => {
    return Array.from({ length: 7 }, (_, index) => addDays(weekStart, index));
  }, [weekStart]);

  const monthStart = startOfMonth(cursorDate);
  const monthGridStart = getWeekStartMonday(monthStart);
  const monthDates = useMemo(() => {
    return Array.from({ length: 42 }, (_, index) =>
      addDays(monthGridStart, index),
    );
  }, [monthGridStart]);

  const title =
    viewMode === "month"
      ? dayjs(cursorDate).format("YYYY/MM")
      : viewMode === "week"
        ? `${dayjs(weekDates[0]).format("MM/DD")} - ${dayjs(weekDates[6]).format("MM/DD")}`
        : "Upcoming schedule";

  const sidebarItems = useMemo(
    () => [
      {
        key: "today" as const,
        label: "Today",
        icon: Sun,
        count: modeCounts.today,
      },
      {
        key: "all" as const,
        label: "All items",
        icon: Inbox,
        count: modeCounts.all,
      },
      {
        key: "week" as const,
        label: "Week",
        icon: CalendarRange,
        count: modeCounts.week,
      },
      {
        key: "month" as const,
        label: "Month",
        icon: Calendar,
        count: modeCounts.month,
      },
    ],
    [modeCounts],
  );

  const activeMeta = FILTER_META[activeMode];

  const openItemEditor = (item: NormalizedScheduleItem) => {
    setEditorTarget({
      otid: item.inst.otid,
      title: item.title ?? `Toent #${item.inst.otid}`,
      dateText: dayjs(item.parsedTime).format("YYYY-MM-DD HH:mm:ss"),
    });
  };

  const updateItemTodoState = async (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => {
    const currentState =
      item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
    if (statusUpdatingTid !== undefined || currentState === nextState) {
      return;
    }
    setStatusUpdatingTid(item.inst.tid);
    try {
      await toentTodoStateCommit({
        otid: item.inst.otid,
        todo_state: nextState,
      });
      setLoadError(undefined);
      setRefreshTick((prev) => prev + 1);
    } catch {
      setLoadError("Failed to update state. Please try again later.");
    } finally {
      setStatusUpdatingTid(undefined);
    }
  };

  return (
    <SidebarProvider>
      <Sidebar variant="inset">
        <SidebarHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-1">
              <KSpaceSelect
                onSelect={(kspace) => {
                  selectKSpace(kspace);
                }}
                currentKSpace={currentKSpace}
                showMKspaces={true}
              />
              <Toggle
                size="sm"
                onClick={() => {
                  if (tags) {
                    setTags(undefined);
                  } else {
                    setTags({ id: "Inset", data: [] });
                  }
                }}
              >
                <Hash className="size-4" />
              </Toggle>
            </div>
            <div className="flex items-center gap-2">
              <NavLink to={RoutePaths.Chnots} id={"chnots"}>
                <Brain className="h-4 w-4" />
              </NavLink>
              <NavLink to={RoutePaths.Settings} id={"settings"}>
                <Settings className="h-4 w-4" />
              </NavLink>
            </div>
          </div>
          <div className="flex flex-wrap items-center gap-1 px-1">
            {tags?.id === "Inset"
              ? tags.data.map((tag) => {
                  return (
                    <Button
                      key={tag}
                      size="sm"
                      variant="outline"
                      onClick={() => {
                        setTags({
                          id: "Inset",
                          data: tags.data.filter((item) => item !== tag),
                        });
                      }}
                    >
                      {tag}
                    </Button>
                  );
                })
              : null}
          </div>
          <div className="relative">
            <Search className="pointer-events-none absolute left-2 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
            <SidebarInput
              value={searchTerm}
              onChange={(event) => setSearchTerm(event.target.value)}
              placeholder="Search mdwt"
              className="pl-8"
            />
          </div>
        </SidebarHeader>
        <SidebarSeparator />
        <SidebarContent>
          {tags ? (
            <SidebarGroup>
              <SidebarGroupLabel>Tags</SidebarGroupLabel>
              <div className="space-y-1 px-2">
                {tagOptions.map((tagName) => {
                  return (
                    <Button
                      key={tagName}
                      size="sm"
                      variant="ghost"
                      className="w-full justify-start"
                      onClick={() => {
                        setTags({
                          id: "Inset",
                          data: [...new Set([...(tags.data ?? []), tagName])],
                        });
                      }}
                    >
                      #{tagName}
                    </Button>
                  );
                })}
              </div>
            </SidebarGroup>
          ) : null}
          <SidebarGroup>
            <SidebarGroupLabel>View</SidebarGroupLabel>
            <SidebarMenu>
              {sidebarItems.map((entry) => {
                const Icon = entry.icon;
                return (
                  <SidebarMenuItem key={entry.key}>
                    <SidebarMenuButton
                      isActive={activeMode === entry.key}
                      onClick={() => setActiveMode(entry.key)}
                    >
                      <Icon />
                      <span>{entry.label}</span>
                      <span className="ml-auto text-xs text-muted-foreground">
                        {entry.count}
                      </span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                );
              })}
            </SidebarMenu>
          </SidebarGroup>

          <SidebarGroup>
            <SidebarGroupLabel>Status filter</SidebarGroupLabel>
            <div className="space-y-2 px-2 text-sm">
              {TODO_STATES.map((state) => {
                return (
                  <div
                    key={state}
                    className="flex items-center justify-between"
                  >
                    <span className="inline-flex items-center gap-1.5">
                      <span
                        className={`size-2 rounded-full ${TODO_STATE_COLOR[state].dot}`}
                      />
                      {TODO_STATE_LABEL[state]}
                    </span>
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-muted-foreground">
                        {stateCounts[state]}
                      </span>
                      <Switch
                        checked={enabledStates[state]}
                        onCheckedChange={(checked) => {
                          setEnabledStates((prev) => ({
                            ...prev,
                            [state]: checked,
                          }));
                        }}
                      />
                    </div>
                  </div>
                );
              })}
            </div>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>

      <main className="h-svh w-full border-l bg-background">
        <header className="flex h-14 items-center gap-2 border-b px-4">
          <SidebarTrigger />
          <div className="flex-1">
            <p className="text-sm font-medium">{activeMeta.title}</p>
            <p className="text-xs text-muted-foreground">
              {loading
                ? "Loading schedule..."
                : loadError
                  ? loadError
                  : `${activeMeta.description} · ${searchedItems.length} items`}
            </p>
          </div>
        </header>

        <div className="flex-1 overflow-auto p-4 sm:p-6">
          <div className="space-y-4">
            {viewMode !== "list" ? (
              <div className="flex flex-wrap items-center justify-between gap-2 rounded-xl border bg-muted/40 p-2">
                <div className="flex items-center gap-2">
                  <Button
                    size="icon"
                    variant="ghost"
                    onClick={() => {
                      setCursorDate((prev) => {
                        return viewMode === "month"
                          ? addMonths(prev, -1)
                          : addDays(prev, -7);
                      });
                    }}
                  >
                    <ChevronLeft className="size-4" />
                  </Button>
                  <Button
                    size="icon"
                    variant="ghost"
                    onClick={() => {
                      setCursorDate((prev) => {
                        return viewMode === "month"
                          ? addMonths(prev, 1)
                          : addDays(prev, 7);
                      });
                    }}
                  >
                    <ChevronRight className="size-4" />
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => setCursorDate(startOfDay(new Date()))}
                  >
                    Today
                  </Button>
                </div>
                <p className="text-sm font-medium text-muted-foreground">
                  {title}
                </p>
              </div>
            ) : null}

            {viewMode === "list" ? (
              <div className="space-y-2">
                {listDayGroups.map((group) => {
                  return (
                    <div key={group.dayKey} className="space-y-2">
                      <p className="px-1 text-xs font-medium tracking-wide text-muted-foreground">
                        {dayjs(group.dayKey).format("MM/DD")}{" "}
                        {getWeekdayShort(dayjs(group.dayKey).toDate())} · 农历
                        {formatLunarDate(dayjs(group.dayKey).toDate())}
                      </p>
                      {group.items.map((item) => {
                        return (
                          <ListItem
                            key={item.inst.tid}
                            item={item}
                            onOpen={openItemEditor}
                            onStateChange={updateItemTodoState}
                            stateUpdating={statusUpdatingTid === item.inst.tid}
                          />
                        );
                      })}
                    </div>
                  );
                })}
              </div>
            ) : null}

            {viewMode === "week" ? (
              <div className="grid min-w-[760px] grid-cols-7 gap-2 overflow-x-auto">
                {weekDates.map((date) => {
                  const dayKey = dateToDayKey(date);
                  const dayItems = groupedByDay[dayKey] ?? [];
                  return (
                    <WeekColumn
                      key={dayKey}
                      date={date}
                      dayItems={dayItems}
                      isToday={isSameDay(date, new Date())}
                      onOpen={openItemEditor}
                      onStateChange={updateItemTodoState}
                      statusUpdatingTid={statusUpdatingTid}
                    />
                  );
                })}
              </div>
            ) : null}

            {viewMode === "month" ? (
              <div className="grid grid-cols-7 gap-2">
                {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].map(
                  (label) => {
                    return (
                      <p
                        key={label}
                        className="text-center text-sm text-muted-foreground"
                      >
                        {label}
                      </p>
                    );
                  },
                )}
                {monthDates.map((date) => {
                  const dayKey = dateToDayKey(date);
                  const dayItems = groupedByDay[dayKey] ?? [];
                  return (
                    <MonthCell
                      key={dayKey}
                      date={date}
                      currentMonth={monthStart.getMonth()}
                      dayItems={dayItems}
                      onOpen={openItemEditor}
                    />
                  );
                })}
              </div>
            ) : null}
          </div>
        </div>

        <Sheet
          open={editorTarget !== undefined}
          onOpenChange={(open) => {
            if (!open) {
              setEditorTarget(undefined);
            }
          }}
        >
          <SheetContent side="right" className="w-full p-0 sm:max-w-3xl">
            {editorTarget && (
              <div className="flex h-full flex-col">
                <SheetHeader className="border-b pb-3">
                  <SheetTitle>{editorTarget.title}</SheetTitle>
                  <SheetDescription>{editorTarget.dateText}</SheetDescription>
                </SheetHeader>
                <div className="min-h-0 flex-1 overflow-auto p-2">
                  <div className="h-full min-h-[55vh]">
                    <MdwtChnot
                      otid={editorTarget.otid}
                      readonly={false}
                      fullscreen={false}
                      onPostSave={async (arg) => {
                        if (arg.saveState !== SaveState.Saved) {
                          return;
                        }

                        if (ensuredMetaOtidsRef.current.has(arg.otid)) {
                          return;
                        }

                        await chnotMetaCommit({
                          metas: [
                            {
                              otid: arg.otid,
                              kind: ChnotKind.MDWT,
                              kspace: currentKSpace,
                            },
                          ],
                        });
                        ensuredMetaOtidsRef.current.add(arg.otid);
                      }}
                    />
                  </div>
                </div>
              </div>
            )}
          </SheetContent>
        </Sheet>
      </main>
    </SidebarProvider>
  );
}

function ListItem({
  item,
  onOpen,
  onStateChange,
  stateUpdating,
}: {
  item: NormalizedScheduleItem;
  onOpen: (item: NormalizedScheduleItem) => void;
  onStateChange: (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => void;
  stateUpdating: boolean;
}) {
  const eventState = item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
  const priority = item.todo?.todo_priority;
  const title = item.title ?? `Toent #${item.inst.otid}`;
  const done = eventState === "DONE";

  return (
    <div className="flex w-full flex-col gap-2 rounded-lg border p-3 transition-colors hover:bg-muted/40 sm:flex-row sm:items-center sm:justify-between">
      <div className="flex items-start gap-2.5">
        <StateInlineSelect
          state={eventState}
          disabled={stateUpdating}
          onChange={(nextState) => onStateChange(item, nextState)}
        />
        <div className="flex items-center gap-2">
          {priority ? <PriorityBadge priority={priority} /> : null}
        </div>
        <button
          type="button"
          onClick={() => onOpen(item)}
          className="flex min-w-0 items-start gap-2.5 text-left"
        >
          <div>
            <p
              className={
                done ? "font-medium text-slate-500 line-through" : "font-medium"
              }
            >
              {title}
            </p>
            <p className="text-sm text-muted-foreground">
              {dayjs(item.parsedTime).format("YYYY-MM-DD HH:mm:ss")}
            </p>
          </div>
        </button>
      </div>
    </div>
  );
}

function WeekColumn({
  date,
  dayItems,
  isToday,
  onOpen,
  onStateChange,
  statusUpdatingTid,
}: {
  date: Date;
  dayItems: NormalizedScheduleItem[];
  isToday: boolean;
  onOpen: (item: NormalizedScheduleItem) => void;
  onStateChange: (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => void;
  statusUpdatingTid?: TID;
}) {
  return (
    <div className="rounded-lg border p-2">
      <div className="mb-2 flex items-center justify-between">
        <div>
          <p className="text-xs text-muted-foreground">
            {getWeekdayShort(date)}
          </p>
          <p className="text-[11px] text-muted-foreground/80">
            农历{formatLunarDate(date)}
          </p>
        </div>
        <Badge variant={isToday ? "default" : "outline"}>
          {dayjs(date).format("DD")}
        </Badge>
      </div>
      <div className="space-y-2">
        {dayItems.length === 0 ? (
          <p className="text-xs text-muted-foreground">No events</p>
        ) : (
          dayItems.map((item) => {
            const state =
              item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
            return (
              <div key={item.inst.tid} className="rounded-md border p-2">
                <div className="flex items-start gap-2">
                  <StateInlineSelect
                    state={state}
                    disabled={statusUpdatingTid === item.inst.tid}
                    onChange={(nextState) => onStateChange(item, nextState)}
                  />
                  <div className="min-w-0">
                    <button
                      type="button"
                      onClick={() => onOpen(item)}
                      className="truncate text-left text-xs font-medium hover:underline"
                    >
                      {item.title ?? `#${item.inst.otid}`}
                    </button>
                    <p className="text-xs text-muted-foreground">
                      {dayjs(item.parsedTime).format("HH:mm")}
                    </p>
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}

function MonthCell({
  date,
  dayItems,
  currentMonth,
  onOpen,
}: {
  date: Date;
  dayItems: NormalizedScheduleItem[];
  currentMonth: number;
  onOpen: (item: NormalizedScheduleItem) => void;
}) {
  const visible = dayItems.slice(0, 3);
  const overflow = dayItems.length - visible.length;
  const isCurrentMonth = date.getMonth() === currentMonth;
  const isToday = isSameDay(date, new Date());

  return (
    <div className="group relative min-h-28 rounded-lg border p-2">
      <div className="mb-2 flex justify-between">
        <div>
          <span
            className={
              isCurrentMonth
                ? "text-sm"
                : "text-sm text-muted-foreground opacity-60"
            }
          >
            {dayjs(date).format("D")}
          </span>
          <p className="text-[11px] text-muted-foreground/80">
            {formatLunarDate(date)}
          </p>
        </div>
        {isToday ? <Badge variant="secondary">Today</Badge> : null}
      </div>
      <div className="space-y-1">
        {visible.map((item) => {
          return (
            <div
              key={item.inst.tid}
              className="rounded-md bg-muted px-2 py-1 text-xs"
            >
              <span className="inline-flex items-center gap-1">
                <span
                  className={`size-1.5 rounded-full ${TODO_STATE_COLOR[item.inst.target_status ?? item.todo?.todo_state ?? "TODO"].dot}`}
                />
                {dayjs(item.parsedTime).format("HH:mm")}
              </span>
              <button
                type="button"
                onClick={() => onOpen(item)}
                className="ml-1 truncate hover:underline"
              >
                {item.title ?? `#${item.inst.otid}`}
              </button>
            </div>
          );
        })}
        {overflow > 0 ? (
          <p className="text-xs text-muted-foreground">+{overflow}</p>
        ) : null}
      </div>

      {dayItems.length > 0 ? (
        <div className="mt-2 flex items-center gap-1 px-0.5">
          {dayItems.slice(0, 4).map((item) => {
            const state =
              item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
            return (
              <span
                key={`${item.inst.tid}-dot`}
                className={`size-1.5 rounded-full ${TODO_STATE_COLOR[state].dot}`}
              />
            );
          })}
        </div>
      ) : null}

      {dayItems.length > 0 ? (
        <div className="pointer-events-none absolute left-1/2 top-0 z-10 hidden w-48 -translate-x-1/2 -translate-y-[104%] rounded-lg border bg-white p-2 text-xs shadow-md group-hover:block">
          <p className="mb-1 font-medium text-slate-700">
            {dayjs(date).format("MM/DD")} · 农历{formatLunarDate(date)}
          </p>
          {dayItems.slice(0, 3).map((item) => {
            return (
              <p
                key={`${item.inst.tid}-preview`}
                className="truncate text-slate-500"
              >
                {dayjs(item.parsedTime).format("HH:mm")}{" "}
                {item.title ?? `#${item.inst.otid}`}
              </p>
            );
          })}
          {dayItems.length > 3 ? (
            <p className="mt-1 text-slate-400">+{dayItems.length - 3} more</p>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

function StateInlineSelect({
  state,
  disabled,
  onChange,
}: {
  state: TodoStateEnum;
  disabled?: boolean;
  onChange: (state: TodoStateEnum) => void;
}) {
  return (
    <Select
      value={state}
      onValueChange={(value) => onChange(value as TodoStateEnum)}
      disabled={disabled}
    >
      <SelectTrigger
        className={`h-7 w-[98px] px-2 text-xs font-medium ${TODO_STATE_COLOR[state].select}`}
      >
        <SelectValue placeholder="State" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="TODO" className="text-blue-700">
          TODO
        </SelectItem>
        <SelectItem value="DOING" className="text-amber-700">
          DOING
        </SelectItem>
        <SelectItem value="WAIT" className="text-violet-700">
          WAIT
        </SelectItem>
        <SelectItem value="DONE" className="text-emerald-700">
          DONE
        </SelectItem>
        <SelectItem value="CANCEL" className="text-rose-700">
          CANCEL
        </SelectItem>
      </SelectContent>
    </Select>
  );
}

function PriorityBadge({ priority }: { priority: TodoPriorityEnum }) {
  const variant =
    priority === "A" ? "default" : priority === "B" ? "secondary" : "outline";
  return <Badge variant={variant}>P{priority}</Badge>;
}

function dateToDayKey(date: Date): string {
  return dayjs(date).format("YYYY-MM-DD");
}

function formatLunarDate(date: Date): string {
  try {
    return new Intl.DateTimeFormat("zh-CN-u-ca-chinese", {
      month: "short",
      day: "numeric",
    }).format(date);
  } catch {
    return "--";
  }
}

function getWeekdayShort(date: Date): string {
  const labels = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  return labels[date.getDay()] ?? "";
}

function getQueryRange(
  activeMode: SidebarMode,
  cursorDate: Date,
): {
  startDate: string;
  endDate: string;
} {
  if (activeMode === "today") {
    const today = dayjs(startOfDay(new Date())).format("YYYY-MM-DD");
    return { startDate: today, endDate: today };
  }

  if (activeMode === "week") {
    const weekStart = getWeekStartMonday(cursorDate);
    const weekEnd = addDays(weekStart, 6);
    return {
      startDate: dayjs(weekStart).format("YYYY-MM-DD"),
      endDate: dayjs(weekEnd).format("YYYY-MM-DD"),
    };
  }

  if (activeMode === "month") {
    const monthStart = startOfMonth(cursorDate);
    const monthGridStart = getWeekStartMonday(monthStart);
    const monthGridEnd = addDays(monthGridStart, 41);
    return {
      startDate: dayjs(monthGridStart).format("YYYY-MM-DD"),
      endDate: dayjs(monthGridEnd).format("YYYY-MM-DD"),
    };
  }

  const base = startOfDay(cursorDate);
  return {
    startDate: dayjs(addDays(base, -45)).format("YYYY-MM-DD"),
    endDate: dayjs(addDays(base, 90)).format("YYYY-MM-DD"),
  };
}

function startOfDay(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

function startOfMonth(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), 1);
}

function addDays(date: Date, amount: number): Date {
  const next = new Date(date);
  next.setDate(next.getDate() + amount);
  return next;
}

function addMonths(date: Date, amount: number): Date {
  const next = new Date(date);
  next.setMonth(next.getMonth() + amount);
  return next;
}

function getWeekStartMonday(date: Date): Date {
  const base = startOfDay(date);
  const day = base.getDay();
  const offset = day === 0 ? 6 : day - 1;
  return addDays(base, -offset);
}

function isSameDay(left: Date, right: Date): boolean {
  return (
    left.getFullYear() === right.getFullYear() &&
    left.getMonth() === right.getMonth() &&
    left.getDate() === right.getDate()
  );
}

export default ToentPage;
