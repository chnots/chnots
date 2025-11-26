import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";
import type { TodoEvent } from "../toent/po";
import type { ChnotKind, ChnotMeta, ChnotThreadMeta } from "./po";

export type MdwtTagSearchType = {
  Inset: string[];
};

export type ChnotSearchRspThread = {
  meta: ChnotThreadMeta;
  title?: string;
};
export type ChnotThreadMetaCommitReq = {
  meta_otid: TID;
  kspace?: Varchar<40>;
  pinned?: boolean;
  archive?: boolean;
};
export type ChnotThreadMetaCommitRsp = {
  meta: ChnotThreadMeta;
};
export type MdwtCommitReqData = {
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
export type ChnotSearchReq = {
  query?: string;
  tags?: MdwtTagSearchType;
  kinds: ChnotKind[];
  with_archive?: boolean;
  start_index: number;
  page_size: number;
};
export type ChnotThreadMetaFetchReq = {
  thread_otid: TID;
};
export type ChnotThreadMetaFetchRsp = {
  thread_meta?: ChnotThreadMeta;
  chnot_meta_sorted: ChnotMeta[];
};

export type ChnotThreadOrderCommitReq = {
  thread_otid: TID;
  orders: ChnotThreadOrderCommitReqData[];
};
export type ChnotThreadOrderCommitRsp = object;
export type MdwtCommitRsp = {
  todo_event?: TodoEvent;
};

export type ChnotMetaCommitReqData = {
  otid: TID;
  kind: ChnotKind;
  kspace: Varchar<40>;
  archive?: boolean;
  pin_it?: boolean;
};
export type ChnotMetaCommitReq = {
  metas: ChnotMetaCommitReqData[];
};
export type ChnotMetaCommitRsp = {
  metas: ChnotMeta[];
};

export type ChnotMetaListReq = {
  otids: TID[];
};
export type ChnotMetaListRsp = {
  metas: ChnotMeta[];
};

export type ChnotSearchRspSingle = {
  title?: string;
  meta: ChnotMeta;
};
