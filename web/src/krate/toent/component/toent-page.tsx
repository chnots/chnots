import dayjs from "dayjs";
import { AnimatePresence, motion } from "framer-motion";
import {
  Brain,
  Calendar,
  CalendarRange,
  Hash,
  Inbox,
  Search,
  Settings,
  Sun,
} from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { NavLink } from "react-router-dom";
import { toast } from "sonner";
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@/common/component/ui/alert";
import { Button } from "@/common/component/ui/button";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInput,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarSeparator,
  SidebarTrigger,
} from "@/common/component/ui/sidebar";
import { Switch } from "@/common/component/ui/switch";
import { Toggle } from "@/common/component/ui/toggle";
import type { MdwtTagSearchType } from "@/krate/chnot/dto";
import { KSpaceSelect } from "@/krate/kspace/component/kspace-select";
import { useKSpaceStore } from "@/krate/kspace/store";
import { chnotTagNameList } from "@/krate/mdwt/service";
import type { ToentScheduleItem } from "@/krate/toent/po";
import {
  toentInstCommit,
  toentInstCount,
  toentSearch,
  toentTodoStateCommit,
} from "@/krate/toent/service";
import { fadeInUp, fastTransition } from "@/lib/animations";
import { tidToDate } from "@/lib/date-utils";
import { genTID, type TID } from "@/lib/id_util";
import { RoutePaths } from "@/router";
import type { TodoStateEnum } from "../toent-model";
import { ToentEditorSheet } from "./toent-editor-sheet";
import { ToentListView } from "./toent-list-view";
import { ToentMonthView } from "./toent-month-view";
import {
  addDays,
  addMonths,
  DEFAULT_ENABLED_TODO_STATES,
  dateToDayKey,
  FILTER_META,
  getDayKeysBetween,
  getItemState,
  getItemTitle,
  getQueryRange,
  getWeekStartMonday,
  type NormalizedScheduleItem,
  type SidebarMode,
  startOfDay,
  startOfMonth,
  TODO_STATE_COLOR,
  TODO_STATE_LABEL,
  TODO_STATES,
  type ToentEditorTarget,
  type ToentViewMode,
  toentInstOtid,
} from "./toent-page-shared";
import { ToentFilterSummaryBar, ToentViewToolbar } from "./toent-page-toolbar";
import { ToentWeekView } from "./toent-week-view";

function queryDateToTidRange(startDate: string, endDate: string) {
  if (!startDate && !endDate) {
    return {
      startTid: dayjs("1900-01-01 00:00:00").valueOf() * 1000,
      endTid: dayjs("2200-01-01 00:00:00").valueOf() * 1000,
    };
  }

  if (!startDate) {
    return {
      startTid: dayjs("1900-01-01 00:00:00").valueOf() * 1000,
      endTid: dayjs(`${endDate} 23:59:59.999`).valueOf() * 1000,
    };
  }

  if (!endDate) {
    return {
      startTid: dayjs(`${startDate} 00:00:00`).valueOf() * 1000,
      endTid: dayjs("2200-01-01 00:00:00").valueOf() * 1000,
    };
  }

  return {
    startTid: dayjs(`${startDate} 00:00:00`).valueOf() * 1000,
    endTid: dayjs(`${endDate} 23:59:59.999`).valueOf() * 1000,
  };
}

function ToentPage() {
  const [activeMode, setActiveMode] = useState<SidebarMode>("today");
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
  >(DEFAULT_ENABLED_TODO_STATES);
  const [cursorDate, setCursorDate] = useState(() => startOfDay(new Date()));
  const [scheduleItems, setScheduleItems] = useState<ToentScheduleItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | undefined>(undefined);
  const [editorTarget, setEditorTarget] = useState<ToentEditorTarget>();
  const [statusUpdatingTid, setStatusUpdatingTid] = useState<TID>();
  const [selectedMonthDayKey, setSelectedMonthDayKey] = useState<string>();
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
  const includeNoTimeTodo = activeMode === "all";

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
      const modeRanges = modes.map((mode) => {
        const { startDate, endDate } = getQueryRange(mode, cursorDate);
        const { startTid, endTid } = queryDateToTidRange(startDate, endDate);
        return {
          key: mode,
          start_tid: startTid,
          end_tid: endTid,
          include_no_time_todo: mode === "all",
        };
      });

      const activeRange = modeRanges.find((range) => range.key === activeMode);
      if (!activeRange) {
        return;
      }

      try {
        const rsp = await toentInstCount({
          start_tid: activeRange.start_tid,
          end_tid: activeRange.end_tid,
          include_completed: includeCompleted,
          include_uncompleted: includeUncompleted,
          include_no_time_todo: activeMode === "all",
          ranges: modeRanges,
        });

        if (!cancelled) {
          setModeCounts((prev) => ({
            ...prev,
            month: rsp.totals.month ?? prev.month,
            week: rsp.totals.week ?? prev.week,
            today: rsp.totals.today ?? prev.today,
            all: rsp.totals.all ?? prev.all,
            [activeMode]: rsp.total,
          }));
        }
      } catch {
        if (!cancelled) {
          setModeCounts((prev) => ({
            ...prev,
            month: 0,
            week: 0,
            today: 0,
            all: 0,
          }));
        }
      }
    };

    loadAllModeCounts();
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

  const viewMode: ToentViewMode =
    activeMode === "week" || activeMode === "month" ? activeMode : "list";

  useEffect(() => {
    if (activeMode === "today") {
      setCursorDate(startOfDay(new Date()));
    }
  }, [activeMode]);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      setLoading(true);
      const { startDate, endDate } = getQueryRange(activeMode, cursorDate);
      const { startTid, endTid } = queryDateToTidRange(startDate, endDate);
      const pageSize = 200;

      try {
        const mergedItems: ToentScheduleItem[] = [];
        let startIndex = 0;
        let hasNext = true;
        let guard = 0;

        while (hasNext && guard < 20) {
          guard += 1;
          const rsp = await toentSearch({
            start_tid: startTid,
            end_tid: endTid,
            include_completed: includeCompleted,
            include_uncompleted: includeUncompleted,
            include_no_time_todo: includeNoTimeTodo,
            query: searchTerm.trim() || undefined,
            tags,
            start_index: startIndex,
            page_size: pageSize,
          });
          mergedItems.push(
            ...rsp.items.flatMap((entry) =>
              entry.inst.map((inst) => ({
                inst: {
                  ...inst,
                  otid: toentInstOtid(inst.chnot_otid, inst.start_tid),
                },
                title: entry.title,
              })),
            ),
          );
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
    includeNoTimeTodo,
    mkspaces,
    tags,
    searchTerm,
    refreshTick,
    currentKSpace,
  ]);

  const normalizedItems = useMemo<NormalizedScheduleItem[]>(() => {
    return scheduleItems
      .map((item) => {
        const parsedTime = tidToDate(item.inst.start_tid ?? item.inst.tid);
        if (!parsedTime) {
          return undefined;
        }

        const parsedEndTime =
          tidToDate(item.inst.end_tid ?? item.inst.start_tid) ?? parsedTime;
        const fixedEndTime =
          parsedEndTime.getTime() >= parsedTime.getTime()
            ? parsedEndTime
            : parsedTime;
        const dayKeys = item.inst.start_tid
          ? getDayKeysBetween(parsedTime, fixedEndTime)
          : [];

        return {
          ...item,
          parsedTime,
          parsedEndTime: fixedEndTime,
          dayKey: dateToDayKey(parsedTime),
          dayKeys,
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
      const state = getItemState(item);
      initial[state] += 1;
    }
    return initial;
  }, [normalizedItems]);

  const stateFilteredItems = useMemo(() => {
    return normalizedItems.filter((item) => {
      const state = getItemState(item);
      return enabledStates[state];
    });
  }, [normalizedItems, enabledStates]);

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

  const searchedItems = useMemo(() => {
    if (activeMode === "all") {
      return stateFilteredItems;
    }

    if (activeMode === "today") {
      return stateFilteredItems;
    }

    const dayKeys =
      activeMode === "week"
        ? weekDates.map((date) => dateToDayKey(date))
        : monthDates.map((date) => dateToDayKey(date));

    const dayKeySet = new Set(dayKeys);
    return stateFilteredItems.filter((item) =>
      item.dayKeys.some((dayKey) => dayKeySet.has(dayKey)),
    );
  }, [activeMode, monthDates, stateFilteredItems, weekDates]);

  const groupedByDay = useMemo(() => {
    const group: Record<string, NormalizedScheduleItem[]> = {};
    for (const item of searchedItems) {
      for (const dayKey of item.dayKeys) {
        group[dayKey] ??= [];
        group[dayKey].push(item);
      }
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

  useEffect(() => {
    if (viewMode !== "month") {
      setSelectedMonthDayKey(undefined);
      return;
    }

    const defaultKey = dateToDayKey(cursorDate);
    const monthDayKeys = new Set(monthDates.map((d) => dateToDayKey(d)));
    if (!selectedMonthDayKey || !monthDayKeys.has(selectedMonthDayKey)) {
      setSelectedMonthDayKey(defaultKey);
    }
  }, [viewMode, cursorDate, monthDates, selectedMonthDayKey]);

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
    const hasTime = Boolean(item.inst.start_tid);
    setEditorTarget({
      chnotOtid: item.inst.chnot_otid,
      instOtid: toentInstOtid(item.inst.chnot_otid, item.inst.start_tid),
      title: getItemTitle(item),
      dateText: hasTime
        ? item.parsedEndTime.getTime() !== item.parsedTime.getTime()
          ? `${dayjs(item.parsedTime).format("YYYY-MM-DD HH:mm:ss")} - ${dayjs(item.parsedEndTime).format("YYYY-MM-DD HH:mm:ss")}`
          : dayjs(item.parsedTime).format("YYYY-MM-DD HH:mm:ss")
        : "No schedule time",
      state: getItemState(item),
      priority: item.inst.todo_priority,
    });
  };

  const updateItemTodoState = async (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => {
    const currentState = getItemState(item);
    if (statusUpdatingTid !== undefined || currentState === nextState) {
      return;
    }
    setStatusUpdatingTid(item.inst.tid);
    try {
      if (item.inst.start_tid) {
        await toentInstCommit({
          inst: {
            ...item.inst,
            todo_state: nextState,
            tid: genTID(),
          },
        });
      } else {
        await toentTodoStateCommit({
          otid: item.inst.otid,
          todo_state: nextState,
        });
      }
      setLoadError(undefined);
      setRefreshTick((prev) => prev + 1);
      toast.success(`State updated: ${currentState} -> ${nextState}`);
    } catch {
      toast.error("Failed to update state. Please try again later.");
    } finally {
      setStatusUpdatingTid(undefined);
    }
  };

  const hasSearchOrTagFilter =
    searchTerm.trim().length > 0 ||
    (tags?.id === "Inset" && tags.data.length > 0);
  const hasStateFilter = TODO_STATES.some(
    (state) => enabledStates[state] !== DEFAULT_ENABLED_TODO_STATES[state],
  );

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
              {activeMeta.description} · {searchedItems.length} items
            </p>
          </div>
        </header>

        <div className="flex-1 overflow-auto p-4 sm:p-6">
          <div className="space-y-4">
            <ToentFilterSummaryBar
              total={searchedItems.length}
              searchTerm={searchTerm}
              tags={tags}
              enabledStates={enabledStates}
              stateCounts={stateCounts}
            />

            <ToentViewToolbar
              viewMode={viewMode}
              title={title}
              onPrev={() => {
                setCursorDate((prev) => {
                  return viewMode === "month"
                    ? addMonths(prev, -1)
                    : addDays(prev, -7);
                });
              }}
              onNext={() => {
                setCursorDate((prev) => {
                  return viewMode === "month"
                    ? addMonths(prev, 1)
                    : addDays(prev, 7);
                });
              }}
              onToday={() => setCursorDate(startOfDay(new Date()))}
            />

            {loadError ? (
              <Alert variant="destructive">
                <AlertTitle>Load failed</AlertTitle>
                <AlertDescription className="flex items-center justify-between gap-3">
                  <span>{loadError}</span>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => setRefreshTick((v) => v + 1)}
                  >
                    Retry
                  </Button>
                </AlertDescription>
              </Alert>
            ) : !loading && searchedItems.length === 0 ? (
              <Alert>
                <AlertTitle>No items found</AlertTitle>
                <AlertDescription>
                  {hasSearchOrTagFilter || hasStateFilter
                    ? "Current filters returned no results. Try clearing search, tags, or status filters."
                    : "No schedule items in this range yet."}
                </AlertDescription>
              </Alert>
            ) : null}

            {!loading && !loadError && searchedItems.length > 0 ? (
              <AnimatePresence mode="wait">
                <motion.div
                  key={viewMode}
                  initial="initial"
                  animate="animate"
                  exit="exit"
                  variants={fadeInUp}
                  transition={fastTransition}
                >
                  {viewMode === "list" ? (
                    <ToentListView
                      listDayGroups={listDayGroups}
                      listItems={searchedItems}
                      groupByDay={
                        activeMode !== "all" && activeMode !== "today"
                      }
                      onOpen={openItemEditor}
                      onStateChange={updateItemTodoState}
                      statusUpdatingTid={statusUpdatingTid}
                    />
                  ) : null}

                  {viewMode === "week" ? (
                    <ToentWeekView
                      weekDates={weekDates}
                      groupedByDay={groupedByDay}
                      onOpen={openItemEditor}
                      onStateChange={updateItemTodoState}
                      statusUpdatingTid={statusUpdatingTid}
                    />
                  ) : null}

                  {viewMode === "month" ? (
                    <ToentMonthView
                      monthDates={monthDates}
                      currentMonth={monthStart.getMonth()}
                      groupedByDay={groupedByDay}
                      selectedDayKey={selectedMonthDayKey}
                      onSelectDay={setSelectedMonthDayKey}
                      onOpen={openItemEditor}
                      onStateChange={updateItemTodoState}
                      statusUpdatingTid={statusUpdatingTid}
                    />
                  ) : null}
                </motion.div>
              </AnimatePresence>
            ) : null}
          </div>
        </div>

        <ToentEditorSheet
          editorTarget={editorTarget}
          currentKSpace={currentKSpace}
          ensuredMetaOtids={ensuredMetaOtidsRef.current}
          onClose={() => {
            setEditorTarget(undefined);
          }}
        />
      </main>
    </SidebarProvider>
  );
}

export default ToentPage;
