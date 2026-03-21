import type { TID } from "@/lib/id_util";
import type { DbText } from "@/lib/types";
import type { TodoStateEnum, TodoPriorityEnum, TimeEvent } from "./toent-model";

export type ToentInst = {
  otid: TID;
  chnot_otid: TID;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  alert_tid?: TID;
  start_tid?: TID;
  end_tid?: TID;
  finished_count: number;
  is_lunar: boolean;
  timezone?: number;
  closed?: boolean;
  tid: TID;
  note?: DbText;
};

export type ToentDefi = {
  otid: TID;
  event_defi?: TimeEventField;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  start_tid?: TID;
  start_timezone?: number;
  end_tid?: TID;
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
export type TimeEventField = {
  time_events: TimeEvent[];
};
