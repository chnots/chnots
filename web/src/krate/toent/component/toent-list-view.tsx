import dayjs from "dayjs";
import type { TID } from "@/lib/id_util";
import { isUpdatingTid, ToentItemCard } from "./toent-item-card";
import type { NormalizedScheduleItem } from "./toent-page-shared";
import { formatLunarDate, getWeekdayShort } from "./toent-page-shared";
import type { TodoStateEnum } from "../toent-model";

export function ToentListView({
  listDayGroups,
  listItems,
  groupByDay,
  onOpen,
  onStateChange,
  statusUpdatingTid,
}: {
  listDayGroups: Array<{ dayKey: string; items: NormalizedScheduleItem[] }>;
  listItems: NormalizedScheduleItem[];
  groupByDay: boolean;
  onOpen: (item: NormalizedScheduleItem) => void;
  onStateChange: (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => void;
  statusUpdatingTid?: TID;
}) {
  if (!groupByDay) {
    return (
      <div className="space-y-2">
        {listItems.map((item) => {
          return (
            <ToentItemCard
              key={item.inst.tid}
              item={item}
              onOpen={onOpen}
              onStateChange={onStateChange}
              stateUpdating={isUpdatingTid(statusUpdatingTid, item.inst.tid)}
            />
          );
        })}
      </div>
    );
  }

  return (
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
                <ToentItemCard
                  key={item.inst.tid}
                  item={item}
                  onOpen={onOpen}
                  onStateChange={onStateChange}
                  stateUpdating={isUpdatingTid(
                    statusUpdatingTid,
                    item.inst.tid,
                  )}
                />
              );
            })}
          </div>
        );
      })}
    </div>
  );
}
