import type { TID } from "@/lib/id_util";
import type { DbText } from "@/lib/types";

export type Toent = object;


export type TodoEvent = "TODO" | "DONE" | "WAIT" | "CANCEL" | "DOING";

export type ToentTimeEvent = object;

export type TodoStateEnum = "TODO" | "DONE" | "DOING" | "WAIT" | "CANCEL";
export type TodoPriorityEnum = "A" | "B" | "C" | "D" | "E";

export type ToentTimeEventInst = object;

export type ToentTodo = {
  otid: TID;
  todo_priority?: TodoPriorityEnum;
  todo_state?: TodoStateEnum;
  todo_closed: boolean;
  tid: TID;
};

export type ToentEventDefiItem = {
  raw: string;
  standard?: string;
  timezone?: string;
};

export type ToentEventDefi = {
  events: ToentEventDefiItem[];
};

export type ToentEvent = {
  otid: TID;
  event_defi: ToentEventDefi;
  tid: TID;
};

export type ToentInst = {
  otid: TID;
  timezone?: string;
  naive_time: string;
  target_status?: TodoStateEnum;
  note?: string;
  alert_tid?: TID;
  target_tid: TID;
  tid: TID;
};

export type ToentScheduleItem = {
  inst: ToentInst;
  todo?: ToentTodo;
  event?: ToentEvent;
  title?: DbText;
};

export type MdwtToent = {
  mdwt_otid: TID;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  todo_closed: boolean;
  note?: DbText;
  tid: TID;
};
