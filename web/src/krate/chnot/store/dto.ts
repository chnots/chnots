import { ChnotRecord, ChnotMetadata, ChnotTag, ChnotKind } from "./db";

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
  record_id?: string;
  meta_id?: string;
  view_type: ListViewType;
  kinds: ChnotKind[];

  with_omited?: boolean;
  query?: string;
  start_index: number;
  page_size: number;
}

export interface ChnotQueryRsp {
  next_start: number;
  data: Chnot[];

  has_next: boolean;
}

export interface ChnotOverwriteReq {
  id?: string;
  meta_id?: string;
  content: string;
  kind: string;
  insert_time: Date;
}

export interface ChnotOverwriteRsp {
  chnot: Chnot;
}

export interface ChnotDeletionReq {
  chnot_id: string;
  logic: boolean;
}

export interface ChnotUpdateReq {
  meta_id: string;

  kspace?: string;

  pinned?: boolean;
  archive?: boolean;

  update_time: boolean;
}

export interface ChnotCommentAddReq {
  id: string;

  chnot_meta_id: string;
  content: string;

  insert_time: Date;
}

export interface ChnotTagQueryReq {
  query?: string;
  tag_tree: ChnotTagTreeType;

  start_index: number;
  page_size: number;
}

export type ChnotKFileRelationInsert = {
  insert_time: Date;
  chnot_meta_id: string;
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
  id: string;
  input: string;
  event: string;
}

export interface ToentGuessReq {
  input: string;
}

export interface ToentGuessRsp {
  toents: Toent[];
}
