import { KSpace } from "./po";

export type KSpaceQueryAllReq = object;

export type KSpaceQueryAllRsp = {
  kspaces: KSpace[];
};

export type KSpaceOverwriteReq = {
  kspace: KSpace;
};

export type KSpaceOverwriteRsp = object;
