import type { TID } from "@/lib/id_util";

export type TodoStateEnum = "TODO" | "DONE" | "DOING" | "WAIT" | "CANCEL";
export type TodoPriorityEnum = "A" | "B" | "C" | "D" | "E";

export type TodoEvent =
  | `${TodoStateEnum}`
  | `${TodoStateEnum} !${TodoPriorityEnum}`;

export type NoneOrI32 = number | null;

export type Dymd = {
  year: NoneOrI32;
  month: NoneOrI32;
  day: NoneOrI32;
};

export type Dhms = {
  hour: NoneOrI32;
  minute: NoneOrI32;
  second: NoneOrI32;
};

export type DymdHms = {
  date: Dymd;
  time: Dhms;
};

export type WesTime = {
  local_minus_utc: number | null;
  timestamp: DymdHms;
};

export type ChnTime = {
  leap_month: boolean;
  timestamp: DymdHms;
};

export type TimeEnum =
  | {
      Wes: WesTime;
    }
  | {
      Chn: ChnTime;
    };

export type RepeatType = "RepeatTodo" | "RepeatEvent";

export type TimeInterval = {
  base: DymdHms;
  week: NoneOrI32;
};

export type Times = {
  count: number;
};

export type EndCondition =
  | {
      Times: Times;
    }
  | {
      Interval: TimeInterval;
    }
  | {
      Time: TimeEnum;
    };

export type TimeEvent = {
  start: TimeEnum | null;
  interval: [TimeInterval, RepeatType] | null;
  interval_end: TimeInterval | null;
  alert: TimeInterval | null;
  end: EndCondition | null;
};

export type ToentTimeEvent = TimeEvent;

export type ToentTimeEventInst = {
  alert_tid: TID | null;
  start_tid: TID | null;
  end_tid: TID | null;
  is_lunar: boolean;
  timezone: number | null;
};
