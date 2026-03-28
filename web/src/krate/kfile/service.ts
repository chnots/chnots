import { chnotShortDate } from "@/lib/date-utils";
import request, { BASE_URL } from "@/lib/request";
import type {
  InlineKFileDownloadReq,
  InlineKFileDownloadRsp,
  InlineKFileUploadReq,
  InlineKFileUploadRsp,
  KFileUploadRsp,
  KfileAssetChunkUploadReq,
  KfileHistoryApplyReq,
  KfileHistoryApplyRsp,
  KfileHistoryFetchReq,
  KfileHistoryFetchRsp,
  KfileHistoryListReq,
  KfileHistoryListRsp,
  KfileMetaFetchReq,
  KfileMetaFetchRsp,
} from "./dto";
import type { KFileMeta } from "./po";

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
  otid,
  binaryp,
  db_store,
}: KfileAssetChunkUploadReq): Promise<KFileUploadRsp> => {
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
  data.append("otid", otid.toString());
  data.append("binaryp", binaryp.toString());
  if (db_store) {
    data.append("db_store", db_store.toString());
  }

  return await request.postFormdata("api/v1/kfile-asset-chunk-upload", data);
};

export const kfileMetaFetch = async (
  req: KfileMetaFetchReq,
): Promise<KfileMetaFetchRsp> => {
  return await request.postJson("api/v1/kfile-meta-fetch", req);
};

export const inlineKFileUpload = async (
  req: InlineKFileUploadReq,
): Promise<InlineKFileUploadRsp> => {
  return await request.putJson("api/v1/kfile-inline-upload", req);
};

export const inlineKFileDownload = async (
  req: InlineKFileDownloadReq,
): Promise<InlineKFileDownloadRsp> => {
  return await request.postJson("api/v1/kfile-inline-download", req);
};

export const getResouceDownloadUrl = (kfile: KFileMeta): string => {
  return `${BASE_URL}/api/v1/kfile-asset-download/${kfile.id}/${encodeURI(`${chnotShortDate()}-${kfile.filename}`)}`;
};

export const kfileHistoryList = async (
  req: KfileHistoryListReq,
): Promise<KfileHistoryListRsp> => {
  return await request.postJson("api/v1/kfile-history-list", req);
};

export const kfileHistoryFetch = async (
  req: KfileHistoryFetchReq,
): Promise<KfileHistoryFetchRsp> => {
  return await request.postJson("api/v1/kfile-history-fetch", req);
};

export const kfileHistoryApply = async (
  req: KfileHistoryApplyReq,
): Promise<KfileHistoryApplyRsp> => {
  return await request.postJson("api/v1/kfile-history-apply", req);
};
