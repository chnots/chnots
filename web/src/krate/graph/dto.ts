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
  data: Value;
};
export type ExcalidrawLibraryCommitRsp = object;
export type ExcalidrawLibraryFetchReq = {
  otid: TID;
};
export type ExcalidrawLibraryFetchRsp = {
  data?: Value;
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

export type GraphHistoryListReq = {
  otid: TID;
};

export type GraphHistoryVersion = {
  tid: TID;
};

export type GraphHistoryListRsp = {
  versions: GraphHistoryVersion[];
};

export type ExcalidrawHistoryFetchReq = {
  otid: TID;
  tid: TID;
};

export type ExcalidrawHistoryFetchRsp = {
  data?: ExcalidrawDataV2Dto;
};

export type ExcalidrawHistoryApplyReq = {
  otid: TID;
  tid: TID;
};

export type ExcalidrawHistoryApplyRsp = {
  data?: ExcalidrawDataV2Dto;
};

export type MindElixirHistoryFetchReq = {
  otid: TID;
  tid: TID;
};

export type MindElixirHistoryFetchRsp = {
  data?: MindElixirDataV2Dto;
};

export type MindElixirHistoryApplyReq = {
  otid: TID;
  tid: TID;
};

export type MindElixirHistoryApplyRsp = {
  data?: MindElixirDataV2Dto;
};
