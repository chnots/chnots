import { Varchar } from "@/lib/types";
import { KSpace } from "./po";

export type KSpaceQueryAllRsp = {
  kspaces: KSpace[];
};

export type KSpaceOverwriteReq = {
  kspace: KSpace;
};

export type KSpaceOverwriteRsp = object;

export type KSpaceQueryAllReq = object;

export type KSpaceDeletionReq = {
  kspace_name: Varchar<500>;
};
export type KSpaceDeletionRsp = object;
