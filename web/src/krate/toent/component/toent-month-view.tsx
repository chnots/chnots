import dayjs from "dayjs";
import { Badge } from "@/common/component/ui/badge";
import type { TodoStateEnum } from "@/krate/toent/po";
import type { TID } from "@/lib/id_util";
import { isUpdatingTid, ToentItemCard } from "./toent-item-card";
import type { NormalizedScheduleItem } from "./toent-page-shared";
import {
  dateToDayKey,
  formatLunarDate,
  getItemState,
  isSpanItem,
  TODO_STATE_COLOR,
} from "./toent-page-shared";

export function ToentMonthView({
  monthDates,
  currentMonth,
  groupedByDay,
  selectedDayKey,
  onSelectDay,
  onOpen,
  onStateChange,
  statusUpdatingTid,
}: {
  monthDates: Date[];
  currentMonth: number;
  groupedByDay: Record<string, NormalizedScheduleItem[]>;
  selectedDayKey?: string;
  onSelectDay: (dayKey: string) => void;
  onOpen: (item: NormalizedScheduleItem) => void;
  onStateChange: (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => void;
  statusUpdatingTid?: TID;
}) {
  const selectedDayItems = selectedDayKey
    ? (groupedByDay[selectedDayKey] ?? [])
    : [];

  return (
    <div className="space-y-3">
      <div className="grid grid-cols-7 gap-2">
        {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].map((label) => {
          return (
            <p
              key={label}
              className="text-center text-sm text-muted-foreground"
            >
              {label}
            </p>
          );
        })}
        {monthDates.map((date) => {
          const dayKey = dateToDayKey(date);
          const dayItems = groupedByDay[dayKey] ?? [];
          const visible = dayItems.slice(0, 3);
          const overflow = dayItems.length - visible.length;
          const isCurrentMonth = date.getMonth() === currentMonth;
          const isToday =
            date.getFullYear() === new Date().getFullYear() &&
            date.getMonth() === new Date().getMonth() &&
            date.getDate() === new Date().getDate();
          const isSelected = selectedDayKey === dayKey;

          return (
            <button
              key={dayKey}
              type="button"
              onClick={() => onSelectDay(dayKey)}
              className={`min-h-28 rounded-lg border p-2 text-left transition-colors hover:bg-muted/40 ${
                isSelected ? "border-primary bg-primary/5" : ""
              }`}
            >
              <div className="mb-2 flex justify-between gap-1">
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
                  const state = getItemState(item);
                  const priority = item.inst.todo_priority;
                  const timeText = isSpanItem(item)
                    ? `${dayjs(item.parsedTime).format("MM/DD HH:mm")}-${dayjs(item.parsedEndTime).format("MM/DD HH:mm")}`
                    : dayjs(item.parsedTime).format("HH:mm");
                  return (
                    <div
                      key={item.inst.tid}
                      className="rounded-md bg-muted px-2 py-1 text-xs"
                    >
                      <span className="inline-flex items-center gap-1">
                        <span
                          className={`size-1.5 rounded-full ${TODO_STATE_COLOR[state].dot}`}
                        />
                        {timeText}
                      </span>
                      {priority ? (
                        <span className="ml-1 text-muted-foreground">
                          P{priority}
                        </span>
                      ) : null}
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
            </button>
          );
        })}
      </div>

      {selectedDayKey ? (
        <div className="rounded-xl border bg-muted/30 p-3">
          <div className="mb-2 flex items-center justify-between">
            <p className="text-sm font-medium">
              {dayjs(selectedDayKey).format("YYYY/MM/DD")} · 农历
              {formatLunarDate(dayjs(selectedDayKey).toDate())}
            </p>
            <Badge variant="outline">{selectedDayItems.length} items</Badge>
          </div>
          <div className="space-y-2">
            {selectedDayItems.length === 0 ? (
              <p className="text-sm text-muted-foreground">
                No events for this day.
              </p>
            ) : (
              selectedDayItems.map((item) => {
                return (
                  <ToentItemCard
                    key={item.inst.tid}
                    item={item}
                    onOpen={onOpen}
                    onStateChange={onStateChange}
                    stateUpdating={isUpdatingTid(
                      statusUpdatingTid,
                      item.inst.tid,
                    )}
                    compact={true}
                  />
                );
              })
            )}
          </div>
        </div>
      ) : null}
    </div>
  );
}
