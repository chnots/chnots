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

export type ChnotThreadTagSearchType = {
  Inset: string[];
};

export type ChnotThread = {
  head_content?: string;
  todo_event?: TodoEvent;
  meta: ChnotThreadMeta;
};
export type ChnotOverwriteThreadMetaReq = {
  meta_otid: TID;
  kspace?: Varchar<40>;
  pinned?: boolean;
  archive?: boolean;
};
export type ChnotOverwriteThreadMetaRsp = {
  meta: ChnotThreadMeta;
};
export type ChnotOverwriteMdwtReqData = {
  otid: TID;
  content: DbText;
};
export type ChnotOverwriteMetaReqData = {
  otid: TID;
  korder: number;
  kind: ChnotKind;
  kind_id: Varchar<200>;
};
export type ChnotThreadArchiveReq = {
  thread_otid: TID;
  logic: boolean;
};
export type ChnotThreadArchiveRsp = object;
export type ChnotThreadQueryReq = {
  query?: string;
  thread_otid?: TID;
  tags?: ChnotThreadTagSearchType;
  kinds: ChnotKind[];
  with_omitted?: boolean;
  with_archive?: boolean;
  start_index: number;
  page_size: number;
};
export type ChnotThreadMetaReq = {
  thread_otid: TID;
};
export type ChnotThreadMetaRsp = {
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
export type ChnotThreadTagQueryReq = {
  query?: string;
  tags?: ChnotThreadTagSearchType;
  remove_params?: boolean;
  start_index: number;
  page_size: number;
};
export type ChnotThreadTagUpdateReq = {
  content: DbText;
  thread_otid: TID;
  kspace: Varchar<40>;
};

export type ChnotThreadTagQueryRsp<T> = {
  data: T[];
  start_index: number;
};

export type ChnotOverwriteMdwtReq = {
  mdwt: ChnotOverwriteMdwtReqData;
};
export type ChnotOverwriteMetaReq = {
  thread_otid: TID;
  metas: ChnotOverwriteMetaReqData[];
};
export type ChnotOverwriteMetaRsp = {
  metas: ChnotMeta[];
};
export type ChnotOverwriteMdwtRsp = {
  todo_event?: TodoEvent;
};

export type ChnotThreadQueryRsp = {
  data: ChnotThread[];
  has_next: boolean;
  next_start: number;
};
