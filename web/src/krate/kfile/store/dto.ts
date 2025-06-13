import { TID } from "@/lib/id_util";
import { InlineKFile, KFile } from "./db";

export interface KFileUploadReq {
  res_id: TID;
  filename: string;
  chunk_no: number;
  total_chunks: number;
  chunk: Blob;
  filetype: string;
  last_modified: number;
  filesize: number;
}

export interface KFileUploadRsp {
  kfile?: KFile;
  finished: boolean;
}

export interface InsertInlineKFileReq {
  res: InlineKFile;
  kkv_key: string;
  archor_intervals: number;
  ignore_conflict?: boolean;
}

export interface InsertInlineKFileRsp {
  true_sid: string;
}

export interface QueryInlineKFileReq {
  tid?: TID;
  kkv_key?: string;
  with_del?: boolean;
  content_type?: string;
  name_like?: string;
}

export interface QueryInlineKFileRsp {
  res: InlineKFile[];
}

export interface KKVInserterReq {
  key: string;
  kind: string;
  value: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface KKVInserterRsp {}

export interface KKVQueryReq {
  key: string;
  kind: "to_file" | "chnot_sub_type";
}

export interface KKVQueryRsp {
  value?: string;
}
