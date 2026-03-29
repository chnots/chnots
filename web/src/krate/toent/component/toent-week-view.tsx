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
  getWeekdayShort,
} from "./toent-page-shared";

const cardVariants: Variants = {
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
              <AnimatePresence mode="popLayout">
                {dayItems.length === 0 ? (
                  <motion.p
                    key={`empty-${dayKey}`}
                    className="text-xs text-muted-foreground"
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                  >
                    No events
                  </motion.p>
                ) : (
                  dayItems.map((item) => {
                    return (
                      <motion.div
                        key={item.inst.tid}
                        layout
                        variants={cardVariants}
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
          </div>
        );
      })}
    </div>
  );
}
