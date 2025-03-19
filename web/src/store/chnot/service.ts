import request from "@/utils/request";
import {
  ChnotDeletionReq,
  ChnotOverwriteReq,
  ChnotOverwriteRsp,
  ChnotQueryReq,
  ChnotQueryRsp,
  ChnotUpdateReq,
} from "./dto";

export const chnotQuery = async (
  req: ChnotQueryReq
): Promise<ChnotQueryRsp> => {
  return await request.post(`api/v1/chnot-query`, req);
};

export const chnotDelete = async (req: ChnotDeletionReq) => {
  return await request.post(`api/v1/chnot-deletion`, req);
};

export const chnotOverwrite = async (
  req: ChnotOverwriteReq
): Promise<ChnotOverwriteRsp> => {
  return await request.put(`api/v1/chnot`, req);
};

export const chnotUpdate = async (req: ChnotUpdateReq) => {
  return await request.post(`api/v1/chnot-update`, req);
};
