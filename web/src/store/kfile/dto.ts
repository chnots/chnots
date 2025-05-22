import { InlineKFile, KFile } from "./db";

export interface KFileUploadReq {
  res_id: string;
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
  archor_intervals: number;
  ignore_conflict?: boolean;
}

export interface QueryInlineKFileReq {
  id?: string;
  rid?: string;
  with_del?: boolean;
  content_type?: string;
  name_like?: string;
}

export interface QueryInlineKFileRsp {
  res: InlineKFile[];
}

export interface KTVInserterReq {
  key: string;
  ttype: string;
  value: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface KTVInserterRsp {}

export interface KTVQueryReq {
  key: string;
  ttype: string;
}

export interface KTVQueryRsp {
  value?: string;
}
