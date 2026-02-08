import type { TID } from "@/lib/id_util";
import type { MindElixirData } from "mind-elixir";

type Value = Object;
type MindElixirDataV2Dto = MindElixirData;

export type ExcalidrawDataV2Dto = object;
export type ExcalidrawCommitReq = {
  otid: TID;
  data: ExcalidrawDataV2Dto;
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
  data: MindElixirDataV2Dto;
};
export type MindElixirCommitRsp = object;
export type MindElixirLoadReq = {
  otid: TID;
};
export type MindElixirLoadRsp = {
  data?: MindElixirDataV2Dto;
};
