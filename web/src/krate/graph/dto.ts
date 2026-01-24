import type { TID } from "@/lib/id_util";

type Value = Object;

export type ExcalidrawDataV2Dto = object;
export type ExcalidrawCommitReq = {
  otid: TID;
  data: string;
};
export type ExcalidrawCommitRsp = object;
export type ExcalidrawFetchReq = {
  otid: TID;
};
export type ExcalidrawFetchRsp = {
  data?: ExcalidrawDataV2Dto;
};
export type MindElixirCommitReq = {
  otid: TID;
  data: Value;
};
export type MindElixirCommitRsp = object;
export type MindElixirLoadReq = {
  otid: TID;
};
export type MindElixirLoadRsp = {
  data: Value;
};
