import dayjs from "dayjs";
import { AnimatePresence, motion, type Variants } from "framer-motion";
import { Badge } from "@/common/component/ui/badge";
import type { TID } from "@/lib/id_util";
import type { TodoStateEnum } from "../toent-model";
import { isUpdatingTid, ToentItemCard } from "./toent-item-card";
import type { NormalizedScheduleItem } from "./toent-page-shared";
import {
  dateToDayKey,
  formatLunarDate,
  getItemState,
  TODO_STATE_COLOR,
} from "./toent-page-shared";

const detailItemVariants: Variants = {
  initial: { opacity: 0, y: 6, scale: 0.97 },
  animate: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: { type: "spring", stiffness: 400, damping: 30 },
  },
  exit: {
    opacity: 0,
    scale: 0.95,
    transition: { duration: 0.12 },
  },
};

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
                  return (
                    <div
                      key={item.inst.tid}
                      className="min-w-0 rounded-md bg-muted px-2 py-1 text-xs"
                    >
                      <span className="inline-flex items-center gap-1">
                        <span
                          className={`size-1.5 rounded-full ${TODO_STATE_COLOR[state].dot}`}
                        />
                        {priority ? (
                          <span className="text-muted-foreground">
                            P{priority}
                          </span>
                        ) : null}
                      </span>
                      <span className="ml-1 break-all">
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

      <AnimatePresence mode="wait">
        {selectedDayKey ? (
          <motion.div
            key={selectedDayKey}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -5 }}
            transition={{ duration: 0.2, ease: [0.25, 0.1, 0.25, 1] }}
            className="rounded-xl border bg-muted/30 p-3"
          >
            <div className="mb-2 flex items-center justify-between">
              <p className="text-sm font-medium">
                {dayjs(selectedDayKey).format("YYYY/MM/DD")} · 农历
                {formatLunarDate(dayjs(selectedDayKey).toDate())}
              </p>
              <Badge variant="outline">{selectedDayItems.length} items</Badge>
            </div>
            <div className="space-y-2">
              <AnimatePresence mode="popLayout">
                {selectedDayItems.length === 0 ? (
                  <motion.p
                    key="empty"
                    className="text-sm text-muted-foreground"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                  >
                    No events for this day.
                  </motion.p>
                ) : (
                  selectedDayItems.map((item) => {
                    return (
                      <motion.div
                        key={item.inst.tid}
                        layout
                        variants={detailItemVariants}
                        initial="initial"
                        animate="animate"
                        exit="exit"
                      >
                        <ToentItemCard
                          item={item}
                          onOpen={onOpen}
                          onStateChange={onStateChange}
                          stateUpdating={isUpdatingTid(
                            statusUpdatingTid,
                            item.inst.tid,
                          )}
                          compact={true}
                          showTime={false}
                        />
                      </motion.div>
                    );
                  })
                )}
              </AnimatePresence>
            </div>
          </motion.div>
        ) : null}
      </AnimatePresence>
    </div>
  );
}
