import { InlineResource, Resource } from "./db";

export interface ResourceUploadRsp {
  resources: Resource[];
}

export interface InsertInlineResourceReq {
  res: InlineResource,
  archor_intervals: number;
}