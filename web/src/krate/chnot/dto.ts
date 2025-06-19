import { TID } from "@/lib/id_util";
import {
  ChnotRecord,
  ChnotMetadata,
  ChnotTag,
  ChnotKind,
  ChnotKindRel,
} from "./po";

export type Chnot = {
  record: ChnotRecord;
  meta: ChnotMetadata;
};

export type ChnotTagSearchType = {
  Inset: string[];
};

export type ChnotQueryReq = {
  query?: string;
  meta_tid?: TID;
  record_tid?: TID;
  tags?: ChnotTagSearchType;
  kinds: ChnotKind[];
  with_omitted?: boolean;
  with_archive?: boolean;
  start_index: number;
  page_size: number;
};

export type ChnotQueryRsp = {
  next_start: number;
  data: Chnot[];

  has_next: boolean;
};

export type ChnotOverwriteReq = {
  meta_tid?: TID;
  content: string;
  kind: ChnotKind;
  kind_id?: string;
};

export type ChnotOverwriteRsp = {
  meta_tid: TID;
  rec_tid: TID;
  kspace: string;
  archor: boolean;
};

export type ChnotDeletionReq = {
  meta_tid: TID;
  logic: boolean;
};

export type ChnotUpdateReq = {
  meta_tid: TID;
  kspace?: string;
  pinned?: boolean;
  archive?: boolean;
};

export type ChnotCommentAddReq = {
  tid: TID;

  chnot_meta_id: TID;
  content: string;
};

export type ChnotTagQueryReq = {
  query?: string;
  tags?: ChnotTagSearchType;
  remove_params?: boolean;
  start_index: number;
  page_size: number;
};

export type ChnotKFileRelationInsert = {
  chnot_meta_id: TID;
};

export type ChnotTagQueryRsp = {
  data: ChnotTag[];

  start_index: number;
};

export type ChnotTagNamesRsp = {
  data: string[];

  start_index: number;
};

export type Toent = {
  tid: TID;
  input: string;
  event: string;
};

export type ToentGuessReq = {
  input: string;
};

export type ToentGuessRsp = {
  toents: Toent[];
};

export type ChnotUpdateRsp = object;

export type ChnotDeletionRsp = object;

export type ChnotTagUpdateReq = {
  content: string;
  meta_tid: TID;
  kspace: string;
};

export type ChnotKindRelQueryRsp = {
  kind_rel: ChnotKindRel;
};
export type ChnotKindRelQueryReq = {
  meta_tid: TID;
};
