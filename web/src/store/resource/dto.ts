import { InlineResource, Resource } from "./db";

export interface ResourceUploadRsp {
  resources: Resource[];
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
  res: InlineResource[]
}