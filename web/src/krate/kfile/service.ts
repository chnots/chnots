import request from "@/lib/request";
import { BASE_URL } from "@/lib/request";
import {
  KfileInlineUploadReq,
  KfileInlineDownloadReq,
  KfileInlineDownloadRsp,
  KFileUploadReq,
  KFileUploadRsp,
  KfileInlineUploadRsp,
  KfileMetaFetchReq,
  KfileMetaFetchRsp,
} from "./dto";
import { chnotShortDate } from "@/lib/date-utils";
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

  return await request.postFormdata("api/v1/kfile-asset-chunk-upload", data);
};

export const kfileMetaFetch = async (
  req: KfileMetaFetchReq,
): Promise<KfileMetaFetchRsp> => {
  return await request.postJson("api/v1/kfile-meta-fetch", req);
};

export const kfileInlineUpload = async (
  req: KfileInlineUploadReq,
): Promise<KfileInlineUploadRsp> => {
  return await request.putJson("api/v1/kfile-inline-upload", req);
};

export const kfileInlineDownload = async (
  req: KfileInlineDownloadReq,
): Promise<KfileInlineDownloadRsp> => {
  return await request.get("api/v1/kfile-inline-download", req);
};

export const getResouceDownloadUrl = (kfile: KFileMeta): string => {
  return `${BASE_URL}/api/v1/kfile-asset-download/${kfile.id}/${encodeURI(chnotShortDate() + "-" + kfile.filename)}`;
};
