import { InlineKFile, KFileMeta } from "./po";
import { Varchar } from "@/lib/types";

export type KFileUploadReq = {
  upload_id: string;
  meta_id: string;
  filename: string;
  chunk_no: number;
  total_chunks: number;
  chunk: Blob;
  last_modified: number;
  filesize: number;
  content_type: string;
};

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

export type KfileInlineDownloadReq = {
  sid?: Varchar<100>;
  meta_id?: Varchar<100>;
  with_omit?: boolean;
};

export type KfileInlineDownloadRsp = {
  res: InlineKFile[];
};

export type KfileMetaFetchReq = {
  meta_id: Varchar<100>;
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
