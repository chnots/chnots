import dayjs from "dayjs";
import type { Variants } from "framer-motion";
import { AnimatePresence, motion } from "framer-motion";
import type { TID } from "@/lib/id_util";
import type { TodoStateEnum } from "../toent-model";
import { isUpdatingTid, ToentItemCard } from "./toent-item-card";
import type { NormalizedScheduleItem } from "./toent-page-shared";
import { formatLunarDate, getWeekdayShort } from "./toent-page-shared";

const itemVariants: Variants = {
  initial: { opacity: 0, y: 8, scale: 0.98 },
  animate: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: { type: "spring", stiffness: 400, damping: 30 },
  },
  exit: {
    opacity: 0,
    scale: 0.95,
    transition: { duration: 0.15 },
  },
};

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
        <AnimatePresence mode="popLayout">
          {listItems.map((item) => {
            return (
              <motion.div
                key={item.inst.tid}
                layout
                variants={itemVariants}
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
                />
              </motion.div>
            );
          })}
        </AnimatePresence>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <AnimatePresence mode="popLayout">
        {listDayGroups.map((group) => {
          return (
            <motion.div
              key={group.dayKey}
              layout
              variants={itemVariants}
              initial="initial"
              animate="animate"
              exit="exit"
              className="space-y-2"
            >
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
            </motion.div>
          );
        })}
      </AnimatePresence>
    </div>
  );
}
