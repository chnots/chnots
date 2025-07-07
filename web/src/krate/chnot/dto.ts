import { TID } from "@/lib/id_util";
import {
  ChnotRecord,
  ChnotMetadata,
  ChnotTag,
  ChnotKind,
  ChnotKindRel,
} from "./po";
import { DbText, Varchar } from "@/lib/types";
import { TodoEvent } from "../toent/po";

export type Chnot = {
  record: ChnotRecord;
  meta: ChnotMetadata;
};

export type ChnotTagSearchType = {
  Inset: string[];
};

export type ChnotQueryReq = {
  query?: string;
  meta_otid?: TID;
  record_otid?: TID;
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
  meta_otid?: TID;
  content: DbText;
  kind: ChnotKind;
  kind_id?: string;
};

export type ChnotOverwriteRsp = {
  meta_otid: TID;
  rec_tid: TID;
  kspace: Varchar<40>;
  archor: boolean;
  meta_tid?: TID;
  todo_event?: TodoEvent;
};

export type ChnotArchiveReq = {
  meta_otid: TID;
  logic: boolean;
};

export type ChnotUpdateReq = {
  meta_otid: TID;
  kspace?: Varchar<40>;
  pinned?: boolean;
  archive?: boolean;
};

export type ChnotCommentAddReq = {
  tid: TID;

  chnot_meta_otid: TID;
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
  chnot_meta_otid: TID;
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

export type ChnotArchiveRsp = object;

export type ChnotTagUpdateReq = {
  content: DbText;
  meta_otid: TID;
  kspace: Varchar<40>;
};

export type ChnotKindRelQueryRsp = {
  kind_rel: ChnotKindRel;
};
export type ChnotKindRelQueryReq = {
  meta_otid: TID;
};
