import { TID } from "@/lib/id_util";
import {
  MdwtRecord,
  ChnotThreadMeta,
  ChnotKind,
  ChnotToent,
  ChnotMeta,
} from "./po";
import { DbText, Varchar } from "@/lib/types";
import { TodoEvent } from "../toent/po";

export type ChnotTagSearchType = {
  Inset: string[];
};

export type ChnotThread = {
  head_content?: DbText;
  todo_event?: TodoEvent;
  meta: ChnotThreadMeta;
};
export type ChnotThreadMetaFetchCommitReq = {
  meta_otid: TID;
  kspace?: Varchar<40>;
  pinned?: boolean;
  archive?: boolean;
};
export type ChnotThreadMetaFetchCommitRsp = {
  meta: ChnotThreadMeta;
};
export type ChnotMdwtCommitReqData = {
  otid: TID;
  content: DbText;
};
export type ChnotThreadOrderCommitReqData = {
  otid: TID;
};
export type ChnotThreadArchiveReq = {
  thread_otid: TID;
  logic: boolean;
};
export type ChnotThreadArchiveRsp = object;
export type ChnotThreadListReq = {
  query?: string;
  thread_otid?: TID;
  tags?: ChnotTagSearchType;
  kinds: ChnotKind[];
  with_omitted?: boolean;
  with_archive?: boolean;
  start_index: number;
  page_size: number;
};
export type ChnotThreadMetaFetchReq = {
  thread_otid: TID;
};
export type ChnotThreadMetaFetchRsp = {
  thread_meta: ChnotThreadMeta;
  chnot_meta_sorted: ChnotMeta[];
};
export type MdwtRecordsReq = {
  mdwt_otids: TID[];
};
export type MdwtRecordsRsp = {
  mdwt_map: Record<TID, MdwtRecord>;
};
export type Toents = {
  toent_inst_map: Record<TID, ChnotToent[]>;
};
export type ChnotTagListReq = {
  query?: string;
  tags?: ChnotTagSearchType;
  remove_params?: boolean;
  start_index: number;
  page_size: number;
};
export type ChnotTagUpdateReq = {
  content: DbText;
  thread_otid: TID;
  kspace: Varchar<40>;
};

export type ChnotTagListRsp<T> = {
  data: T[];
  start_index: number;
};

export type ChnotMdwtCommitReq = {
  mdwt: ChnotMdwtCommitReqData;
};
export type ChnotThreadOrderCommitReq = {
  thread_otid: TID;
  orders: ChnotThreadOrderCommitReqData[];
};
export type ChnotThreadOrderCommitRsp = object;
export type ChnotMdwtCommitRsp = {
  todo_event?: TodoEvent;
};

export type ChnotThreadListRsp = {
  data: ChnotThread[];
  has_next: boolean;
  next_start: number;
};

export type ChnotMetaCommitReqData = {
  otid: TID;
  kind: ChnotKind;
  kind_id: Varchar<200>;
  kspace: Varchar<200>;
};
export type ChnotMetaCommitReq = {
  metas: ChnotMetaCommitReqData[];
};
export type ChnotMetaCommitRsp = {
  metas: ChnotMeta[];
};
