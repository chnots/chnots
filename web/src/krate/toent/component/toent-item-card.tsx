import dayjs from "dayjs";
import { Badge } from "@/common/component/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/common/component/ui/select";
import type { TodoPriorityEnum, TodoStateEnum } from "@/krate/toent/po";
import type { TID } from "@/lib/id_util";
import {
  getItemState,
  getItemTitle,
  isSpanItem,
  type NormalizedScheduleItem,
  TODO_STATE_COLOR,
} from "./toent-page-shared";

export function StateInlineSelect({
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

export function PriorityBadge({ priority }: { priority: TodoPriorityEnum }) {
  const variant =
    priority === "A" ? "default" : priority === "B" ? "secondary" : "outline";
  return <Badge variant={variant}>P{priority}</Badge>;
}

export function ToentItemCard({
  item,
  onOpen,
  onStateChange,
  stateUpdating,
  compact = false,
  timeFormat,
}: {
  item: NormalizedScheduleItem;
  onOpen: (item: NormalizedScheduleItem) => void;
  onStateChange: (
    item: NormalizedScheduleItem,
    nextState: TodoStateEnum,
  ) => void;
  stateUpdating: boolean;
  compact?: boolean;
  timeFormat?: string;
}) {
  const state = getItemState(item);
  const priority = item.inst.todo_priority;
  const title = getItemTitle(item);
  const done = state === "DONE";
  const timeText = isSpanItem(item)
    ? `${dayjs(item.parsedTime).format("YYYY-MM-DD HH:mm")} - ${dayjs(item.parsedEndTime).format("YYYY-MM-DD HH:mm")}`
    : dayjs(item.parsedTime).format(timeFormat ?? "YYYY-MM-DD HH:mm:ss");

  return (
    <div
      className={
        compact
          ? "rounded-md border p-2"
          : "flex w-full flex-col gap-2 rounded-lg border p-3 transition-colors hover:bg-muted/40 sm:flex-row sm:items-center sm:justify-between"
      }
    >
      <div
        className={
          compact ? "flex items-start gap-2" : "flex items-start gap-2.5"
        }
      >
        <StateInlineSelect
          state={state}
          disabled={stateUpdating}
          onChange={(nextState) => onStateChange(item, nextState)}
        />
        <div className="min-w-0">
          <div className="mb-1 flex items-center gap-2">
            {priority ? <PriorityBadge priority={priority} /> : null}
          </div>
          <button
            type="button"
            onClick={() => onOpen(item)}
            className="min-w-0 text-left"
          >
            <p
              className={
                done
                  ? compact
                    ? "truncate text-xs font-medium text-slate-500 line-through"
                    : "font-medium text-slate-500 line-through"
                  : compact
                    ? "truncate text-xs font-medium"
                    : "font-medium"
              }
            >
              {title}
            </p>
            <p
              className={
                compact
                  ? "text-xs text-muted-foreground"
                  : "text-sm text-muted-foreground"
              }
            >
              {timeText}
            </p>
          </button>
        </div>
      </div>
    </div>
  );
}

export function isUpdatingTid(
  statusUpdatingTid: TID | undefined,
  tid: TID,
): boolean {
  return statusUpdatingTid === tid;
}
