import dayjs from "dayjs";
import { Badge } from "@/common/component/ui/badge";
import type { TodoStateEnum } from "@/krate/toent/po";
import type { TID } from "@/lib/id_util";
import { isUpdatingTid, ToentItemCard } from "./toent-item-card";
import type { NormalizedScheduleItem } from "./toent-page-shared";
import {
  dateToDayKey,
  formatLunarDate,
  getWeekdayShort,
} from "./toent-page-shared";

export function ToentWeekView({
  weekDates,
  groupedByDay,
  onOpen,
  onStateChange,
  statusUpdatingTid,
}: {
  weekDates: Date[];
  groupedByDay: Record<string, NormalizedScheduleItem[]>;
  onOpen: (item: NormalizedScheduleItem) => void;
  onStateChange: (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => void;
  statusUpdatingTid?: TID;
}) {
  return (
    <div className="grid min-w-[760px] grid-cols-7 gap-2 overflow-x-auto">
      {weekDates.map((date) => {
        const dayKey = dateToDayKey(date);
        const dayItems = groupedByDay[dayKey] ?? [];
        const isToday =
          date.getFullYear() === new Date().getFullYear() &&
          date.getMonth() === new Date().getMonth() &&
          date.getDate() === new Date().getDate();

        return (
          <div key={dayKey} className="rounded-lg border p-2">
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
                      timeFormat="HH:mm"
                    />
                  );
                })
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
