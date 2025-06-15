import { TID } from "@/lib/id_util";
import { InlineKFile, KFile } from "./po";

export type KFileUploadReq = {
  res_id: TID;
  filename: string;
  chunk_no: number;
  total_chunks: number;
  chunk: Blob;
  filetype: string;
  last_modified: number;
  filesize: number;
};

export type KFileUploadRsp = {
  kfile?: KFile;
  finished: boolean;
};

export type InsertInlineKFileReq = {
  res: InlineKFile;
  meta_id: string;
  archor_intervals: number;
  ignore_conflict?: boolean;
};

export type InsertInlineKFileRsp = {
  true_sid: string;
};

export type QueryInlineKFileReq = {
  tid?: TID;
  kkv_key?: string;
  with_del?: boolean;
  content_type?: string;
  name_like?: string;
};

export type QueryInlineKFileRsp = {
  res: InlineKFile[];
};

export type KKVInserterReq = {
  key: string;
  kind: string;
  value: string;
};

export type KKVInserterRsp = object;

export type KKVQueryReq = {
  key: string;
  kind: "to_file" | "chnot_sub_type";
};

export type KKVQueryRsp = {
  value?: string;
};
