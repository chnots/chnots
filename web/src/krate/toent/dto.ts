import type { TID } from "@/lib/id_util";
import type { MdwtTagSearchType } from "../chnot/dto";
import type { MdwtToent, TodoStateEnum, ToentScheduleItem } from "./po";

type PossibleScore = number;

export type ToentGuessReq = {
  input: string;
};

export type GuessElem<T> = {
  toent: T;
  score: PossibleScore;
};
export type ToentGuessRsp<T> = {
  toents: T[];
};

export type Toents = {
  toent_inst_map: Record<TID, MdwtToent[]>;
};

export type ToentInstListReq = {
  start_date: string;
  end_date: string;
  include_completed: boolean;
  include_uncompleted: boolean;
  start_index: number;
  page_size: number;
};

export type ToentInstCountReq = {
  start_date: string;
  end_date: string;
  include_completed: boolean;
  include_uncompleted: boolean;
};

export type ToentSearchReq = {
  start_date: string;
  end_date: string;
  include_completed: boolean;
  include_uncompleted: boolean;
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
};

export type ToentTodoStateCommitReq = {
  otid: TID;
  todo_state: TodoStateEnum;
};

export type ToentTodoStateCommitRsp = {
  otid: TID;
  todo_state: TodoStateEnum;
};
