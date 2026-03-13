import type { TID } from "@/lib/id_util";
import type { DbText } from "@/lib/types";

export type Toent = object;

export type TodoEvent = "TODO" | "DONE" | "WAIT" | "CANCEL" | "DOING";

export type ToentTimeEvent = string;

export type TodoStateEnum = "TODO" | "DONE" | "DOING" | "WAIT" | "CANCEL";
export type TodoPriorityEnum = "A" | "B" | "C" | "D" | "E";

export type ToentTimeEventInst = object;

export type ToentInst = {
  otid: TID;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  alert_tid?: TID;
  start_tid?: TID;
  end_tid?: TID;
  finished_count: number;
  timezone?: number;
  closed?: boolean;
  note?: DbText;
  tid: TID;
};

export type ToentEventDefi = {
  events: ToentTimeEvent[];
};

export type ToentDefi = {
  otid: TID;
  event_defi: ToentEventDefi;
  start_time?: TID;
  start_timezone?: number;
  end_time?: TID;
  end_timezone?: number;
  tid: TID;
};

export type ToentScheduleItem = {
  inst: ToentInst;
  title: string;
};

export type MdwtToent = {
  mdwt_otid: TID;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  closed?: boolean;
  note?: DbText;
  tid: TID;
};
