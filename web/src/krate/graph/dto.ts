import type { MindElixirData } from "mind-elixir";
import type { TID } from "@/lib/id_util";

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
export type ExcalidrawLibraryDataV1Dto = object;
export type ExcalidrawLibraryCommitReq = {
  otid: TID;
  data: ExcalidrawLibraryDataV1Dto;
};
export type ExcalidrawLibraryCommitRsp = object;
export type ExcalidrawLibraryFetchReq = {
  otid: TID;
};
export type ExcalidrawLibraryFetchRsp = {
  data?: ExcalidrawLibraryDataV1Dto;
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
