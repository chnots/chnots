import type { TID } from "@/lib/id_util";
import type { MdwtToent, ToentScheduleItem } from "./po";

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
  start_index: number;
  page_size: number;
};

export type ToentInstListRsp = {
  items: ToentScheduleItem[];
  has_next: boolean;
  next_start: number;
};
