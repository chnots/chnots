import request from "@/lib/request";
import { BASE_URL } from "@/lib/request";
import {
  InsertInlineKFileReq,
  KKVInserterReq,
  KKVQueryReq,
  KKVQueryRsp,
  QueryInlineKFileReq,
  QueryInlineKFileRsp,
  KFileUploadReq,
  KFileUploadRsp,
  InsertInlineKFileRsp,
} from "./dto";
import { KFile } from "./po";
import { chnotShortDate } from "@/lib/date-utils";
import { TID } from "@/lib/id_util";

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
  data.append("res_id", res_id.toString());
  data.append("filename", filename);
  data.append("chunk_no", chunk_no.toString());
  data.append("total_chunks", total_chunks.toString());
  data.append("filetype", filetype);
  data.append("last_modified", last_modified.toString());
  data.append("filesize", filesize.toString());

  return await request.post("api/v1/kfile", data);
};

export const kfileQueryInfo = async (
  sid: string
): Promise<{ res?: KFile }> => {
  return await request.get("api/v1/kfile-info/" + sid);
};

export const insertInlineKFile = async (
  req: InsertInlineKFileReq
): Promise<InsertInlineKFileRsp> => {
  return await request.put("api/v1/inline-kfile", req);
};

export const queryInlineKFile = async (
  req: QueryInlineKFileReq
): Promise<QueryInlineKFileRsp> => {
  return await request.get("api/v1/inline-kfile", req);
};

export const getResouceDownloadUrl = (kfile: KFile): string => {
  return (
    BASE_URL +
    "/api/v1/kfile/" +
    kfile.sid +
    "/" +
    encodeURI(chnotShortDate() + "-" + kfile.ori_filename)
  );
};

export const insertKKV = async (req: KKVInserterReq) => {
  return await request.put("api/v1/kv", req);
};

export const queryKKV = async (req: KKVQueryReq): Promise<KKVQueryRsp> => {
  return await request.get("api/v1/kv", req);
};
