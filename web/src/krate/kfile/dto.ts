import type { TID } from "@/lib/id_util";
import type { Varchar } from "@/lib/types";
import type { InlineKFile, KFileMeta } from "./po";

export type KFileUploadRsp = {
  kfile?: KFileMeta;
  finished: boolean;
};

export type KfileInlineUploadReq = {
  meta_id: Varchar<100>;
  otid: TID;
  res: InlineKFile;
  archor_intervals: number;
  filename?: Varchar<1024>;
  content_type: Varchar<200>;
};

export type KfileInlineUploadRsp = {
  true_sid: Varchar<100>;
};

export type KfileMetaFetchReqId = { Otid: TID } | { Id: string };

export type KfileInlineDownloadReq = {
  req_id: KfileMetaFetchReqId;
  with_omit?: boolean;
};

export type KfileInlineDownloadRsp = {
  meta?: KFileMeta;
  file?: InlineKFile;
};

export type KfileMetaFetchReq = {
  req_id: KfileMetaFetchReqId;
};

export type KfileMetaFetchRsp = {
  meta?: KFileMeta;
};

export type KfileInlineUploadDirectlyReq = {
  file: InlineKFile;
};
export type KfileInlineUploadDirectlyRsp = object;
export type KfileInlineDownloadBySidReq = {
  sid: Varchar<100>;
};
export type KfileInlineDownloadBySidRsp = {
  file?: InlineKFile;
};

export type KfileAssetChunkUploadReq = {
  upload_id: string;
  meta_id: string;
  otid: number;
  filename: string;
  chunk_no: number;
  total_chunks: number;
  chunk: Blob;
  last_modified: number;
  filesize: number;
  content_type: string;
};
