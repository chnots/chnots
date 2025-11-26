import request from "@/lib/request";
import type {
  KSpaceArchiveReq,
  KSpaceArchiveRsp,
  KSpaceCommitReq,
  KSpaceCommitRsp,
  KSpaceListReq,
  KSpaceListRsp,
} from "./dto";

export const kspaceList = async (
  req: KSpaceListReq,
): Promise<KSpaceListRsp> => {
  return await request.postJson(`/api/v1/kspace-list`, req);
};

export const kspaceCommit = async (
  req: KSpaceCommitReq,
): Promise<KSpaceCommitRsp> => {
  return await request.postJson(`/api/v1/kspace-commit`, req);
};

export const ksapceArchive = async (
  req: KSpaceArchiveReq,
): Promise<KSpaceArchiveRsp> => {
  return await request.postJson(`/api/v1/kspace-archive`, req);
};
