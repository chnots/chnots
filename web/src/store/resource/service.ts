import request from "@/utils/request";
import {
  InsertInlineResourceReq,
  QueryInlineResourceReq,
  QueryInlineResourceRsp,
  ResourceUploadReq,
  ResourceUploadRsp,
} from "./dto";

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


export const insertInlineResource = async (req: InsertInlineResourceReq) => {
  return await request.put("api/v1/inline-resource", req);
}

export const queryInlineResource = async (req: QueryInlineResourceReq): Promise<QueryInlineResourceRsp> => {
  return await request.get("api/v1/inline-resource", req);
}