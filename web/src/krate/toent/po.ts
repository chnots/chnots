import type { TID } from "@/lib/id_util";
import type { DbText } from "@/lib/types";

export type Toent = object;

export type ToentInst = object;

export type TodoEvent = "TODO" | "DONE" | "WAIT" | "CANCEL" | "DOING";

export type ToentTimeEvent = object;

export type TodoStateEnum = "TODO" | "DONE" | "DOING" | "WAIT" | "CANCEL";
export type TodoPriorityEnum = "A" | "B" | "C" | "D" | "E";

export type ToentTimeEventInst = object;

export type MdwtToent = {
  mdwt_otid: TID;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  todo_closed: boolean;
  note?: DbText;
  tid: TID;
};
