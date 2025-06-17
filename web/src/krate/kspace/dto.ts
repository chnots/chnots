import { KSpace } from "./po";

export type KSpaceQueryAllRsp = {
  kspaces: KSpace[];
};             

export type KSpaceOverwriteReq = {
  kspace: KSpace;
};             

export type KSpaceOverwriteRsp = object;             

export type KSpaceQueryAllReq = object;             

