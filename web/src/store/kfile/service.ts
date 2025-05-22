import request from "@/utils/request";
import { BASE_URL } from "@/utils/request";
import {
  InsertInlineKFileReq,
  KTVInserterReq,
  KTVQueryReq,
  KTVQueryRsp,
  QueryInlineKFileReq,
  QueryInlineKFileRsp,
  KFileUploadReq,
  KFileUploadRsp,
} from "./dto";
import { KFile } from "./db";

export const kfileUpload = async ({
  chunk,
  res_id,
  filename,
  chunk_no,
  total_chunks,
  filetype,
  last_modified,
  filesize,
}: KFileUploadReq): Promise<KFileUploadRsp> => {
  const data = new FormData();
  data.append("chunk", chunk);
  data.append("res_id", res_id);
  data.append("filename", filename);
  data.append("chunk_no", chunk_no.toString());
  data.append("total_chunks", total_chunks.toString());
  data.append("filetype", filetype);
  data.append("last_modified", last_modified.toString());
  data.append("filesize", filesize.toString());

  return await request.post("api/v1/kfile", data);
};

export const kfileQueryInfo = async (
  resId: string
): Promise<{ res?: KFile }> => {
  return await request.get("api/v1/kfile-info/" + resId);
};

export const insertInlineKFile = async (req: InsertInlineKFileReq) => {
  return await request.put("api/v1/inline-kfile", req);
};

export const queryInlineKFile = async (
  req: QueryInlineKFileReq
): Promise<QueryInlineKFileRsp> => {
  return await request.get("api/v1/inline-kfile", req);
};

export const getResouceDownloadUrl = (kfile: KFile): string => {
  return BASE_URL + "/api/v1/kfile/" + kfile.id;
};

export const insertKTV = async (req: KTVInserterReq) => {
  return await request.put("api/v1/kv", req);
};

export const queryKTV = async (req: KTVQueryReq): Promise<KTVQueryRsp> => {
  return await request.get("api/v1/kv", req);
};
