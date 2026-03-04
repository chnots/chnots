import dayjs from "dayjs";
import {
  CalendarDays,
  ChevronLeft,
  ChevronRight,
  ListChecks,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { Badge } from "@/common/component/ui/badge";
import { Button } from "@/common/component/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/common/component/ui/card";
import type {
  TodoPriorityEnum,
  TodoStateEnum,
  ToentScheduleItem,
} from "@/krate/toent/po";
import { toentInstList } from "@/krate/toent/service";
import { parseNaiveDateTime } from "@/lib/date-utils";

type ToentViewMode = "list" | "week" | "month";

type NormalizedScheduleItem = ToentScheduleItem & {
  parsedTime: Date;
  dayKey: string;
};

function ToentPage() {
  const [viewMode, setViewMode] = useState<ToentViewMode>("week");
  const [cursorDate, setCursorDate] = useState(() => startOfDay(new Date()));
  const [scheduleItems, setScheduleItems] = useState<ToentScheduleItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | undefined>(undefined);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      setLoading(true);
      const startDate = dayjs(addDays(startOfDay(cursorDate), -45)).format(
        "YYYY-MM-DD",
      );
      const endDate = dayjs(addDays(startOfDay(cursorDate), 90)).format(
        "YYYY-MM-DD",
      );
      const pageSize = 200;

      try {
        const mergedItems: ToentScheduleItem[] = [];
        let startIndex = 0;
        let hasNext = true;
        let guard = 0;

        while (hasNext && guard < 20) {
          guard += 1;
          const rsp = await toentInstList({
            start_date: startDate,
            end_date: endDate,
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
        setLoadError("加载失败，请稍后重试。");
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
  }, [cursorDate]);

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

  const groupedByDay = useMemo(() => {
    const group: Record<string, NormalizedScheduleItem[]> = {};
    for (const item of normalizedItems) {
      group[item.dayKey] ??= [];
      group[item.dayKey].push(item);
    }
    return group;
  }, [normalizedItems]);

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
      ? dayjs(cursorDate).format("YYYY年MM月")
      : viewMode === "week"
        ? `${dayjs(weekDates[0]).format("MM/DD")} - ${dayjs(weekDates[6]).format("MM/DD")}`
        : "未来日程";

  return (
    <section className="w-full min-h-[100svh] p-4 sm:p-6 lg:p-8">
      <div className="mx-auto w-full max-w-6xl space-y-4">
        <Card className="gap-4">
          <CardHeader className="gap-3 sm:flex sm:flex-row sm:items-center sm:justify-between">
            <div>
              <CardTitle className="flex items-center gap-2 text-lg sm:text-xl">
                <CalendarDays className="size-5" />
                Toent 日程
              </CardTitle>
              <CardDescription>
                MVP 视图已支持列表/周/月；周起始日固定为周一。
              </CardDescription>
            </div>
            <div className="flex items-center gap-2">
              <Button
                size="sm"
                variant={viewMode === "list" ? "default" : "outline"}
                onClick={() => setViewMode("list")}
              >
                <ListChecks className="size-4" />
                列表
              </Button>
              <Button
                size="sm"
                variant={viewMode === "week" ? "default" : "outline"}
                onClick={() => setViewMode("week")}
              >
                周
              </Button>
              <Button
                size="sm"
                variant={viewMode === "month" ? "default" : "outline"}
                onClick={() => setViewMode("month")}
              >
                月
              </Button>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            {viewMode !== "list" ? (
              <div className="flex flex-wrap items-center justify-between gap-2 rounded-lg border p-2">
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
                    今天
                  </Button>
                </div>
                <p className="text-sm font-medium">{title}</p>
              </div>
            ) : null}

            <p className="text-xs text-muted-foreground">
              {loading
                ? "正在加载日程..."
                : loadError
                  ? loadError
                  : `共 ${normalizedItems.length} 条实例`}
            </p>

            {!loading && !loadError && normalizedItems.length === 0 ? (
              <div className="rounded-lg border border-dashed p-6 text-sm text-muted-foreground">
                当前时间窗口暂无实例。
              </div>
            ) : null}

            {viewMode === "list" ? (
              <div className="space-y-2">
                {normalizedItems.map((item) => {
                  return <ListItem key={item.inst.tid} item={item} />;
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
                    />
                  );
                })}
              </div>
            ) : null}

            {viewMode === "month" ? (
              <div className="grid grid-cols-7 gap-2">
                {["一", "二", "三", "四", "五", "六", "日"].map((label) => {
                  return (
                    <p
                      key={label}
                      className="text-center text-sm text-muted-foreground"
                    >
                      周{label}
                    </p>
                  );
                })}
                {monthDates.map((date) => {
                  const dayKey = dateToDayKey(date);
                  const dayItems = groupedByDay[dayKey] ?? [];
                  return (
                    <MonthCell
                      key={dayKey}
                      date={date}
                      currentMonth={monthStart.getMonth()}
                      dayItems={dayItems}
                    />
                  );
                })}
              </div>
            ) : null}
          </CardContent>
        </Card>
      </div>
    </section>
  );
}

function ListItem({ item }: { item: NormalizedScheduleItem }) {
  const eventState = item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
  const priority = item.todo?.todo_priority;
  const title = item.title ?? `Toent #${item.inst.otid}`;

  return (
    <div className="flex flex-col gap-2 rounded-lg border p-3 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <p className="font-medium">{title}</p>
        <p className="text-sm text-muted-foreground">
          {dayjs(item.parsedTime).format("YYYY-MM-DD HH:mm:ss")}
        </p>
      </div>
      <div className="flex items-center gap-2">
        <StateBadge state={eventState} />
        {priority ? <PriorityBadge priority={priority} /> : null}
      </div>
    </div>
  );
}

function WeekColumn({
  date,
  dayItems,
  isToday,
}: {
  date: Date;
  dayItems: NormalizedScheduleItem[];
  isToday: boolean;
}) {
  return (
    <div className="rounded-lg border p-2">
      <div className="mb-2 flex items-center justify-between">
        <p className="text-xs text-muted-foreground">
          {dayjs(date).format("ddd")}
        </p>
        <Badge variant={isToday ? "default" : "outline"}>
          {dayjs(date).format("DD")}
        </Badge>
      </div>
      <div className="space-y-2">
        {dayItems.length === 0 ? (
          <p className="text-xs text-muted-foreground">无安排</p>
        ) : (
          dayItems.map((item) => {
            const state =
              item.inst.target_status ?? item.todo?.todo_state ?? "TODO";
            return (
              <div key={item.inst.tid} className="rounded-md border p-2">
                <p className="truncate text-xs font-medium">
                  {item.title ?? `#${item.inst.otid}`}
                </p>
                <p className="text-xs text-muted-foreground">
                  {dayjs(item.parsedTime).format("HH:mm")}
                </p>
                <div className="mt-1">
                  <StateBadge state={state} />
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
}: {
  date: Date;
  dayItems: NormalizedScheduleItem[];
  currentMonth: number;
}) {
  const visible = dayItems.slice(0, 3);
  const overflow = dayItems.length - visible.length;
  const isCurrentMonth = date.getMonth() === currentMonth;
  const isToday = isSameDay(date, new Date());

  return (
    <div className="min-h-28 rounded-lg border p-2">
      <div className="mb-2 flex justify-between">
        <span
          className={
            isCurrentMonth
              ? "text-sm"
              : "text-sm text-muted-foreground opacity-60"
          }
        >
          {dayjs(date).format("D")}
        </span>
        {isToday ? <Badge variant="secondary">今天</Badge> : null}
      </div>
      <div className="space-y-1">
        {visible.map((item) => {
          return (
            <div
              key={item.inst.tid}
              className="rounded-md bg-muted px-2 py-1 text-xs"
            >
              <span>{dayjs(item.parsedTime).format("HH:mm")}</span>
              <span className="ml-1 truncate">
                {item.title ?? `#${item.inst.otid}`}
              </span>
            </div>
          );
        })}
        {overflow > 0 ? (
          <p className="text-xs text-muted-foreground">+{overflow}</p>
        ) : null}
      </div>
    </div>
  );
}

function StateBadge({ state }: { state: TodoStateEnum }) {
  if (state === "DONE") {
    return <Badge variant="secondary">DONE</Badge>;
  }
  if (state === "CANCEL") {
    return <Badge variant="destructive">CANCEL</Badge>;
  }
  if (state === "DOING") {
    return <Badge>DOING</Badge>;
  }
  if (state === "WAIT") {
    return <Badge variant="outline">WAIT</Badge>;
  }
  return <Badge variant="outline">TODO</Badge>;
}

function PriorityBadge({ priority }: { priority: TodoPriorityEnum }) {
  const variant =
    priority === "A" ? "default" : priority === "B" ? "secondary" : "outline";
  return <Badge variant={variant}>P{priority}</Badge>;
}

function dateToDayKey(date: Date): string {
  return dayjs(date).format("YYYY-MM-DD");
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
