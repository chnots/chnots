import type { KSpace } from "./po";
import type { Varchar } from "@/lib/types";

export type KSpaceListRsp = {
  kspaces: KSpace[];
};

export type KSpaceCommitReq = {
  kspace: KSpace;
};

export type KSpaceCommitRsp = object;

export type KSpaceListReq = object;

export type KSpaceArchiveReq = {
  kspace_name: Varchar<500>;
};
export type KSpaceArchiveRsp = object;
