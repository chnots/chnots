import type { TID } from "@/lib/id_util";
import type { MdwtTagSearchType } from "../chnot/dto";
import type {
  MdwtToent,
  TimeEventField,
  ToentInst,
  ToentScheduleItem,
} from "./po";
import type { TodoStateEnum, TodoEvent } from "./toent-model";

type PossibleScore = number;

export type ToentGuessReq = {
  input: string;
};

export type GuessElem<T> = {
  timestamp: T;
  score: PossibleScore;
};
export type ToentGuessRsp<T> = {
  toents: T[];
};

export type Toents = {
  toent_inst_map: Record<TID, MdwtToent[]>;
};

export type ToentInstListReq = {
  start_tid: number;
  end_tid: number;
  include_completed: boolean;
  include_uncompleted: boolean;
  start_index: number;
  page_size: number;
};

export type ToentInstCountReq = {
  start_tid: TID;
  end_tid: TID;
  include_completed: boolean;
  include_uncompleted: boolean;
  include_no_time_todo: boolean;
  ranges: ToentInstCountRangeReq[];
};

export type ToentInstCountRangeReq = {
  key: string;
  start_tid: TID;
  end_tid: TID;
  include_no_time_todo: boolean;
};

export type ToentSearchReq = {
  start_tid: TID;
  end_tid: TID;
  include_completed: boolean;
  include_uncompleted: boolean;
  chnot_otids?: TID[];
  include_no_time_todo: boolean;
  query?: string;
  tags?: MdwtTagSearchType;
  start_index: number;
  page_size: number;
};

export type ToentInstListRsp = {
  items: ToentScheduleItem[];
  has_next: boolean;
  next_start: number;
};

export type ToentInstCountRsp = {
  total: number;
  totals: Record<string, number>;
};

export type ToentTodoStateCommitReq = {
  otid: TID;
  todo_state: TodoStateEnum;
};

export type ToentTodoStateCommitRsp = {
  otid: TID;
  todo_state: TodoStateEnum;
};

export type ToentSearchRsp = {
  items: ToentSearchRspData[];
  has_next: boolean;
  next_start: number;
};
export type ToentSearchRspData = {
  inst: ToentInst[];
  defi?: TimeEventField;
  title: string;
};

export type ToentInstCommitReq = {
  inst: ToentInst;
};
export type ToentInstCommitRsp = {
  updated_insts: ToentInst[];
};

export type ToentDefiCommitReq = {
  otid: TID;
  todo_event?: TodoEvent;
  time_event_field?: TimeEventField;
};
export type ToentDefiCommitRsp = object;
