import { InlineResource, Resource } from "./db";

export interface ResourceUploadReq {
  res_id: string;
  filename: string;
  chunk_no: number;
  total_chunks: number;
  chunk: Blob;
  filetype: string;
  last_modified: number;
  filesize: number;
}

export interface ResourceUploadRsp {
  resource?: Resource;
  finished: boolean;
}

export interface InsertInlineResourceReq {
  res: InlineResource;
  archor_intervals: number;
  ignore_conflict?: boolean;
}

export interface QueryInlineResourceReq {
  id?: string;
  rid?: string;
  with_del?: boolean;
  content_type?: string;
  name_like?: string;
}

export interface QueryInlineResourceRsp {
  res: InlineResource[];
}

export interface KTVInserterReq {
  key: string;
  ttype: string;
  value: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface KTVInserterRsp {}

export interface KTVQueryReq {
  key: string;
  ttype: string;
}

export interface KTVQueryRsp {
  value?: string;
}
