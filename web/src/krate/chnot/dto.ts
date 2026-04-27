import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";
import type { TodoEvent } from "../toent/toent-model";
import type { ChnotKind, ChnotMeta } from "./po";

export type MdwtTagSearchType = {
  id: "Inset";
  data: string[];
};

export type MdwtCommitReqData = {
  otid: TID;
  content: DbText;
};
export type ChnotThreadOrderCommitReqData = {
  otid: TID;
  closed: boolean;
};
export type ChnotSearchReq = {
  query?: string;
  tags?: MdwtTagSearchType;
  kinds: ChnotKind[];
  start_index: number;
  page_size: number;
};
export type ChnotThreadMetaFetchReq = {
  otid: TID;
  include_hist?: boolean;
};
export type ChnotThreadMetaFetchRsp = {
  chnot_meta_sorted: ChnotThreadMetaFetchRspData[];
};

export type ChnotThreadOrderCommitReq = {
  thread_otid: TID;
  orders: ChnotThreadOrderCommitReqData[];
  remove_others?: boolean;
};
export type ChnotThreadOrderCommitRsp = object;
export type MdwtBlockRspData = {
  otid: TID;
  title: string;
};
export type MdwtCommitRsp = {
  todo_event?: TodoEvent;
  title: string;
  blocks: MdwtBlockRspData[];
  content?: DbText;
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

export type ChnotThreadMetaFetchRspData = {
  meta: ChnotMeta;
  closed: boolean;
};

export type ChnotSearchRspData = {
  title?: string;
  meta: ChnotMeta;
};

export type ChnotThreadOrderArchiveRsp = object;
export type ChnotThreadOrderArchiveReq = {
  thread_otid: TID;
  otids: TID[];
};
