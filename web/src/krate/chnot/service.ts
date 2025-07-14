import request from "@/lib/request";
import {
  ChnotArchiveReq,
  ChnotKindRelQueryRsp,
  ChnotOverwriteRecordReq,
  ChnotOverwriteRecordRsp,
  ChnotQueryReq,
  ChnotQueryRsp,
  ChnotTagNamesRsp,
  ChnotTagQueryReq,
  ChnotOverwriteMetaReq,
  ToentGuessReq,
  ToentGuessRsp,
} from "./dto";
import { TID } from "@/lib/id_util";

export const chnotQuery = async (
  req: ChnotQueryReq
): Promise<ChnotQueryRsp> => {
  return await request.post(`api/v1/chnot-query`, req);
};

export const chnotDelete = async (req: ChnotArchiveReq) => {
  return await request.post(`api/v1/chnot-deletion`, req);
};

export const chnotOverwriteRecord = async (
  req: ChnotOverwriteRecordReq
): Promise<ChnotOverwriteRecordRsp> => {
  return await request.put(`api/v1/chnot-overwrite-record`, req);
};

export const chnotOverwriteMeta = async (req: ChnotOverwriteMetaReq) => {
  return await request.post(`api/v1/chnot-overwrite-meta`, req);
};

export const chnotTagNames = async (
  req: ChnotTagQueryReq,
): Promise<ChnotTagNamesRsp> => {
  return await request.post(`api/v1/chnot-tag-names`, req);
};

export const toentGuess = async (
  req: ToentGuessReq,
): Promise<ToentGuessRsp> => {
  return await request.post(`api/v1/toent-guess`, req);
};

export const chnotQueryKindRel = async (
  chnotMetaTid: TID,
): Promise<ChnotKindRelQueryRsp> => {
  return await request.get(`/api/v1/chnot-query-kind-rel`, {
    meta_otid: chnotMetaTid,
  });
};
