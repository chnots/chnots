import { TID } from "@/lib/id_util";
import { ChnotThreadMeta, ChnotKind, ChnotMeta } from "./po";
import { DbText, Varchar } from "@/lib/types";
import { MdwtToent, TodoEvent } from "../toent/po";

export type MdwtTagSearchType = {
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
export type MdwtCommitReqData = {
  otid: TID;
  content: DbText;
};
export type chnotThreadOrderCommitReqData = {
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
  tags?: MdwtTagSearchType;
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

export type chnotThreadOrderCommitReq = {
  thread_otid: TID;
  orders: chnotThreadOrderCommitReqData[];
};
export type chnotThreadOrderCommitRsp = object;
export type MdwtCommitRsp = {
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
