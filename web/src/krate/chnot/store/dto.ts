import { TID } from "@/lib/id_util";
import { ChnotRecord, ChnotMetadata, ChnotTag, ChnotKind } from "./po";

export interface Chnot {
  record: ChnotRecord;
  meta: ChnotMetadata;
}

export type ChnotTagTreeType = {
  kind: "tagtree";
  tagkind: "children" | "descendants";
  tagpath: string;
};
export type ListViewType = { kind: "timeline" } | ChnotTagTreeType;
export const listViewTypeGetTagPath = (lvt: ListViewType) => {
  if (lvt.kind === "tagtree") {
    return lvt.tagpath;
  } else {
    return undefined;
  }
};

export interface ChnotQueryReq {
  query?: string;
  meta_tid?: TID;
  record_tid?: TID;

  view_type: ListViewType;
  kinds: ChnotKind[];

  with_omited?: boolean;
  start_index: number;
  page_size: number;
}

export interface ChnotQueryRsp {
  next_start: number;
  data: Chnot[];

  has_next: boolean;
}

export interface ChnotOverwriteReq {
  tid?: TID;
  meta_tid?: TID;
  content: string;
  kind: string;
  kind_id?: string;
}

export interface ChnotOverwriteRsp {
  meta_tid: TID;
  rec_tid: TID;
  archor: boolean;
  kspace: string;
}

export interface ChnotDeletionReq {
  chnot_id: TID;
  logic: boolean;
}

export interface ChnotUpdateReq {
  meta_tid: TID;

  kspace?: string;

  pinned?: boolean;
  archive?: boolean;

  update_time: boolean;
}

export interface ChnotCommentAddReq {
  tid: TID;

  chnot_meta_id: TID;
  content: string;
}

export interface ChnotTagQueryReq {
  query?: string;
  tag_tree: ChnotTagTreeType;

  start_index: number;
  page_size: number;
}

export type ChnotKFileRelationInsert = {
  chnot_meta_id: TID;
};

export interface ChnotTagQueryRsp {
  data: ChnotTag[];

  start_index: number;
}

export interface ChnotTagNamesRsp {
  data: string[];

  start_index: number;
}

export interface Toent {
  tid: TID;
  input: string;
  event: string;
}

export interface ToentGuessReq {
  input: string;
}

export interface ToentGuessRsp {
  toents: Toent[];
}
