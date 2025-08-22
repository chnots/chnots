import { TID } from "@/lib/id_util";
import {
  MdwtRecord,
  ChnotMetadata,
  ChnotTag,
  ChnotKind,
  ChnotBlockToent,
  ChnotBlockMeta,
} from "./po";
import { DbText, Varchar } from "@/lib/types";
import { TodoEvent } from "../toent/po";

export type ChnotTagSearchType = {
  Inset: string[];
};

export type Chnot = {
  head_record: MdwtRecord;
  meta: ChnotMetadata;
};
export type ChnotOverwriteMetaReq = {
  meta_otid: TID;
  kspace?: Varchar<40>;
  pinned?: boolean;
  archive?: boolean;
};
export type ChnotOverwriteMetaRsp = object;
export type ChnotOverwriteRecordReqMdwt = {
  block_otid: TID;
  content: DbText;
};
export type ChnotOverwriteBlockReqMeta = {
  block_otid: TID;
  korder: number;
  kind: ChnotKind;
  kind_id: Varchar<200>;
};
export type ChnotOverwriteBlockReq = {
  meta_otid: TID;
  mdwts: ChnotOverwriteRecordReqMdwt[];
  metas: ChnotOverwriteBlockReqMeta[];
};
export type ChnotOverwriteRecordRsp = {
  todo_event?: TodoEvent;
};
export type ChnotArchiveReq = {
  meta_otid: TID;
  logic: boolean;
};
export type ChnotArchiveRsp = object;
export type ChnotQueryReq = {
  query?: string;
  meta_otid?: TID;
  tags?: ChnotTagSearchType;
  kinds: ChnotKind[];
  with_omitted?: boolean;
  with_archive?: boolean;
  start_index: number;
  page_size: number;
};
export type ChnotMetaReq = {
  chnot_meta_otid: TID;
};
export type ChnotMetaRsp = {
  chnot_meta: ChnotMetadata;
  block_meta_sorted: ChnotBlockMeta[];
};
export type MdwtBlocksReq = {
  mdwt_otids: TID[];
};
export type MdwtBlocksRsp = {
  mdwt_map: Record<TID, MdwtRecord>;
};
export type Toents = {
  toent_inst_map: Record<TID, ChnotBlockToent[]>;
};
export type ChnotTagQueryReq = {
  query?: string;
  tags?: ChnotTagSearchType;
  remove_params?: boolean;
  start_index: number;
  page_size: number;
};
export type ChnotTagUpdateReq = {
  content: DbText;
  meta_otid: TID;
  kspace: Varchar<40>;
};

export type ChnotQueryRsp<T> = {
  data: T[];
  has_next: boolean;
  next_start: number;
};
export type ChnotTagQueryRsp<T> = {
  data: T[];
  start_index: number;
};
