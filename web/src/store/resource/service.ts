import request from "@/utils/request";
import { BASE_URL } from "@/utils/request";
import {
  InsertInlineResourceReq,
  KTVInserterReq,
  KTVQueryReq,
  KTVQueryRsp,
  QueryInlineResourceReq,
  QueryInlineResourceRsp,
  ResourceUploadReq,
  ResourceUploadRsp,
} from "./dto";
import { Resource } from "./db";

export const resourceUpload = async ({
  chunk,
  res_id,
  filename,
  chunk_no,
  total_chunks,
  filetype,
  last_modified,
  filesize,
}: ResourceUploadReq): Promise<ResourceUploadRsp> => {
  const data = new FormData();
  data.append("chunk", chunk);
  data.append("res_id", res_id);
  data.append("filename", filename);
  data.append("chunk_no", chunk_no.toString());
  data.append("total_chunks", total_chunks.toString());
  data.append("filetype", filetype);
  data.append("last_modified", last_modified.toString());
  data.append("filesize", filesize.toString());

  return await request.post("api/v1/resource", data);
};

export const resourceQueryInfo = async (resId: string): Promise<{ res?: Resource }> => {
  return await request.get("api/v1/resource-info/" + resId);
};

export const insertInlineResource = async (req: InsertInlineResourceReq) => {
  return await request.put("api/v1/inline-resource", req);
};

export const queryInlineResource = async (
  req: QueryInlineResourceReq
): Promise<QueryInlineResourceRsp> => {
  return await request.get("api/v1/inline-resource", req);
};

export const getResouceDownloadUrl = (resource: Resource): string => {
  return BASE_URL + "/api/v1/resource/" + resource.id;
};

export const insertKTV = async (req: KTVInserterReq) => {
  return await request.put("api/v1/kv", req);
};

export const queryKTV = async (req: KTVQueryReq): Promise<KTVQueryRsp> => {
  return await request.get("api/v1/kv", req);
};
