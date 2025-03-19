import { ChnotRecord, ChnotMetadata, ChnotTag } from "./db";

export interface Chnot {
  record: ChnotRecord;
  meta: ChnotMetadata;
}

export interface ChnotQueryReq {
  record_id?: string;
  meta_id?: string;
  tag_keyword?: string;
  
  with_omited?: boolean;
  query?: string;
  start_index: number;
  page_size: number;
}

export interface ChnotQueryRsp {
  data: Chnot[];

  start_index: number;
}

export interface ChnotOverwriteReq {
  chnot: ChnotRecord;
  kind: string;
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

  namespace?: string;

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

  start_index: number;
  page_size: number;
}

export interface ChnotTagQueryRsp {
  data: ChnotTag[];

  start_index: number;
}

export interface ChnotTagNamesRsp {
  data: string[];

  start_index: number;
}
