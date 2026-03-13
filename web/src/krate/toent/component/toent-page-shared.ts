import dayjs from "dayjs";
import type {
  TodoPriorityEnum,
  TodoStateEnum,
  ToentScheduleItem,
} from "@/krate/toent/po";
import type { TID } from "@/lib/id_util";

export type ToentViewMode = "list" | "week" | "month";
export type SidebarMode = "month" | "week" | "today" | "all";

export const TODO_STATES: TodoStateEnum[] = [
  "TODO",
  "DOING",
  "WAIT",
  "DONE",
  "CANCEL",
];

export const TODO_STATE_LABEL: Record<TodoStateEnum, string> = {
  TODO: "TODO",
  DOING: "DOING",
  WAIT: "WAIT",
  DONE: "DONE",
  CANCEL: "CANCEL",
};

export const TODO_STATE_COLOR: Record<
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

export const FILTER_META: Record<
  SidebarMode,
  { title: string; description: string }
> = {
  today: { title: "Today", description: "Only reminders for today" },
  all: { title: "All items", description: "All reminders in current range" },
  month: { title: "Month", description: "View reminders by month" },
  week: { title: "Week", description: "View reminders by week" },
};

export type NormalizedScheduleItem = ToentScheduleItem & {
  parsedTime: Date;
  parsedEndTime: Date;
  dayKey: string;
  dayKeys: string[];
};

export type ToentEditorTarget = {
  otid: TID;
  title: string;
  dateText: string;
  state: TodoStateEnum;
  priority?: TodoPriorityEnum;
};

export function getItemState(item: NormalizedScheduleItem): TodoStateEnum {
  return item.inst.todo_state ?? "TODO";
}

export function getItemTitle(item: NormalizedScheduleItem): string {
  return item.title ?? `Toent #${item.inst.otid}`;
}

export function dateToDayKey(date: Date): string {
  return dayjs(date).format("YYYY-MM-DD");
}

export function getDayKeysBetween(start: Date, end: Date): string[] {
  const result: string[] = [];
  const cursor = startOfDay(start);
  const endDay = startOfDay(end);
  while (cursor.getTime() <= endDay.getTime()) {
    result.push(dateToDayKey(cursor));
    cursor.setDate(cursor.getDate() + 1);
  }
  return result;
}

export function isSpanItem(item: NormalizedScheduleItem): boolean {
  return item.dayKeys.length > 1;
}

export function formatLunarDate(date: Date): string {
  try {
    return new Intl.DateTimeFormat("zh-CN-u-ca-chinese", {
      month: "short",
      day: "numeric",
    }).format(date);
  } catch {
    return "--";
  }
}

export function getWeekdayShort(date: Date): string {
  const labels = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  return labels[date.getDay()] ?? "";
}

export function getQueryRange(
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

export function startOfDay(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

export function startOfMonth(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), 1);
}

export function addDays(date: Date, amount: number): Date {
  const next = new Date(date);
  next.setDate(next.getDate() + amount);
  return next;
}

export function addMonths(date: Date, amount: number): Date {
  const next = new Date(date);
  next.setMonth(next.getMonth() + amount);
  return next;
}

export function getWeekStartMonday(date: Date): Date {
  const base = startOfDay(date);
  const day = base.getDay();
  const offset = day === 0 ? 6 : day - 1;
  return addDays(base, -offset);
}

export function isSameDay(left: Date, right: Date): boolean {
  return (
    left.getFullYear() === right.getFullYear() &&
    left.getMonth() === right.getMonth() &&
    left.getDate() === right.getDate()
  );
}
