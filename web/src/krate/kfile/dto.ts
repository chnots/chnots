import { TID } from "@/lib/id_util";
import { InlineKFile, KFileMeta } from "./po";

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
  meta_id: string;
  res: InlineKFile;
  archor_intervals: number;
  filename?: string;
  content_type: string;
};                        

export type InsertInlineKFileRsp = {
  true_sid: string;
};             

export type QueryInlineKFileReq = {
  sid?: string;
  meta_id?: string;
  with_omit?: boolean;
};                        

export type QueryInlineKFileRsp = {
  res: InlineKFile[];
};             



export type QueryKFileReq = {
  meta_id: string;
};                               

export type QueryKFileMetaRsp = {
  meta?: KFileMeta;
};              