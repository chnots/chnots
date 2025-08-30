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

export type InsertInlineKFileReq = {
  meta_id: Varchar<100>;
  res: InlineKFile;
  archor_intervals: number;
  filename?: Varchar<1024>;
  content_type: Varchar<200>;
};

export type InsertInlineKFileRsp = {
  true_sid: Varchar<100>;
};

export type QueryInlineKFileReq = {
  sid?: Varchar<100>;
  meta_id?: Varchar<100>;
  with_omit?: boolean;
};

export type QueryInlineKFileRsp = {
  res: InlineKFile[];
};

export type QueryKFileReq = {
  meta_id: Varchar<100>;
};

export type QueryKFileMetaRsp = {
  meta?: KFileMeta;
};

export type KFileInlineInsert2Req = {
  file: InlineKFile;
};
export type KFileInlineInsert2Rsp = object;
export type KFileInlineGetBySidReq = {
  sid: Varchar<100>;
};
export type KFileInlineGetBySidRsp = {
  file?: InlineKFile;
};

export type KFileChunkUploadReq = {
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
