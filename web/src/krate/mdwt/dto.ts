import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";
import type { MdwtTagSearchType } from "../chnot/dto";
import type { TodoEvent } from "../toent/toent-model";
import type { MdwtRecord } from "./po";

export type MdwtRecordsReq = {
  mdwt_otids: TID[];
};
export type MdwtRecordsRsp = {
  mdwt_map: Record<TID, MdwtRecord>;
};

export type MdwtTagListReq = {
  query?: string;
  tags?: MdwtTagSearchType;
  remove_params?: boolean;
  start_index: number;
  page_size: number;
};
export type MdwtTagUpdateReq = {
  content: DbText;
  mdwt_otid: TID;
};

export type MdwtTagListRsp<T> = {
  data: T[];
  start_index: number;
};

export type MdwtCommitReq = {
  mdwt: MdwtCommitReqData;
};

export type MdwtCommitReqData = {
  otid: TID;
  content: DbText;
};
export type MdwtCommitRsp = {
  todo_event?: TodoEvent;
  title: string;
};
