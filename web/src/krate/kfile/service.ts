import request from "@/lib/request";
import { BASE_URL } from "@/lib/request";
import {
  InsertInlineKFileReq,
  QueryInlineKFileReq,
  QueryInlineKFileRsp,
  KFileUploadReq,
  KFileUploadRsp,
  InsertInlineKFileRsp,
} from "./dto";
import { chnotShortDate } from "@/lib/date-utils";
import { TID } from "@/lib/id_util";
import { KFileMeta } from "./po";

export const kfileUpload = async ({
  upload_id,
  meta_id,
  chunk,
  filename,
  chunk_no,
  total_chunks,
  last_modified,
  filesize,
  content_type,
}: KFileUploadReq): Promise<KFileUploadRsp> => {
  const data = new FormData();
  data.append("chunk", chunk);
  data.append("filename", filename);
  data.append("chunk_no", chunk_no.toString());
  data.append("total_chunks", total_chunks.toString());
  data.append("last_modified", last_modified.toString());
  data.append("filesize", filesize.toString());
  data.append("content_type", content_type.toString());
  data.append("meta_id", meta_id);
  data.append("upload_id", upload_id);

  return await request.post("api/v1/kfile", data);
};

export const kfileQueryInfo = async (
  sid: string
): Promise<{ res?: KFileMeta }> => {
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

export const getResouceDownloadUrl = (kfile: KFileMeta): string => {
  return (
    BASE_URL +
    "/api/v1/kfile/" +
    kfile.sid +
    "/" +
    encodeURI(chnotShortDate() + "-" + kfile.filename)
  );
};
