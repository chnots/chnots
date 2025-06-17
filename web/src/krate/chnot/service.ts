import request from "@/lib/request";
import {
  ChnotDeletionReq,
  ChnotKindRelQueryRsp,
  ChnotOverwriteReq,
  ChnotOverwriteRsp,
  ChnotQueryReq,
  ChnotQueryRsp,
  ChnotTagNamesRsp,
  ChnotTagQueryReq,
  ChnotUpdateReq,
  ToentGuessReq,
  ToentGuessRsp,
} from "./dto";
import { TID } from "@/lib/id_util";

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

export const chnotTagNames = async (
  req: ChnotTagQueryReq
): Promise<ChnotTagNamesRsp> => {
  return await request.post(`api/v1/chnot-tag-names`, req);
};

export const chnotKFileRelationInsert = async (
  req: ChnotTagQueryReq
): Promise<void> => {
  return await request.post(`api/v1/chnot-tag-names`, req);
};

export const toentGuess = async (
  req: ToentGuessReq
): Promise<ToentGuessRsp> => {
  return await request.post(`api/v1/toent-guess`, req);
};

export const chnotQueryKindRel = async (
  chnotMetaTid: TID
): Promise<ChnotKindRelQueryRsp> => {
  return await request.get(`/api/v1/chnot-query-kind-rel`, {
    meta_tid: chnotMetaTid,
  });
};

