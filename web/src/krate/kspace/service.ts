import request from "@/lib/request";
import {
  KSpaceDeletionReq,
  KSpaceDeletionRsp,
  KSpaceOverwriteReq,
  KSpaceOverwriteRsp,
  KSpaceQueryAllReq,
  KSpaceQueryAllRsp,
} from "./dto";

export const allKSpaces = async (
  req: KSpaceQueryAllReq
): Promise<KSpaceQueryAllRsp> => {
  return await request.get(`/api/v1/kspace-all`, req);
};

export const overwriteKSpace = async (
  req: KSpaceOverwriteReq
): Promise<KSpaceOverwriteRsp> => {
  return await request.put(`/api/v1/kspace-overwrite`, req);
};

export const deleteKSpace = async (
  req: KSpaceDeletionReq
): Promise<KSpaceDeletionRsp> => {
  return await request.put(`/api/v1/kspace-deletion`, req);
};
