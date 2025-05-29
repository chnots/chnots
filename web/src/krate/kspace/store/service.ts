import request from "@/utils/request";
import { KSpaceOverwriteReq, KSpaceOverwriteRsp, KSpaceQueryAllReq, KSpaceQueryAllRsp } from "./dto";

export const allKSpaces = async (
  req: KSpaceQueryAllReq
): Promise<KSpaceQueryAllRsp> => {
  return await request.get(`/api/v1/kspace-all`, req);
};

export const overwriteKSpace = async (
  req: KSpaceOverwriteReq
): Promise<KSpaceOverwriteRsp> => {
  return await request.put(`/api/v1/kspace`, req);
};


