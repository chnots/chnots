import type { TID } from "@/lib/id_util";
import type { Varchar } from "@/lib/types";
import type { InlineKFile, KFileMeta } from "./po";

export type KFileUploadRsp = {
  kfile?: KFileMeta;
  finished: boolean;
};

export type InlineKFileUploadReq = {
  meta_id: Varchar<100>;
  otid: TID;
  res: InlineKFile;
  archor_intervals: number;
  filename?: Varchar<1024>;
  content_type: Varchar<200>;
  binaryp: boolean;
};

export type InlineKFileUploadRsp = {
  true_sid: Varchar<100>;
};

export type KfileMetaFetchReqId = { Otid: TID } | { Id: string };

export type InlineKFileDownloadReq = {
  req_id: KfileMetaFetchReqId;
  with_omit?: boolean;
};

export type InlineKFileDownloadRsp = {
  meta?: KFileMeta;
  file?: InlineKFile;
};

export type KfileMetaFetchReq = {
  req_id: KfileMetaFetchReqId;
  history_and_archor?: boolean;
};

export type KfileMetaFetchRsp = {
  meta?: KFileMeta;
};

export type KfileHistoryListReq = {
  otid: TID;
};

export type KfileHistoryVersion = {
  tid: TID;
};

export type KfileHistoryListRsp = {
  versions: KfileHistoryVersion[];
};

export type KfileHistoryFetchReq = {
  otid: TID;
  tid: TID;
};

export type KfileHistoryFetchRsp = {
  meta?: KFileMeta;
  file?: InlineKFile;
};

export type KfileHistoryApplyReq = {
  otid: TID;
  tid: TID;
};

export type KfileHistoryApplyRsp = {
  meta?: KFileMeta;
  file?: InlineKFile;
};

export type InlineKFileUploadDirectlyReq = {
  file: InlineKFile;
};
export type InlineKFileUploadDirectlyRsp = object;
export type InlineKFileDownloadBySidReq = {
  sid: Varchar<100>;
};
export type InlineKFileDownloadBySidRsp = {
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
  db_store?: boolean;
  binaryp: boolean;
};
