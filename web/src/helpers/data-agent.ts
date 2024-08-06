import request from "./request";
import { Chnot } from "../model";

export interface ChnotOverwriteReq {
  chnot: Chnot;
}

export interface ChnotOverwriteRsp {}

export const overwriteChnot = async (
  req: ChnotOverwriteReq
): Promise<ChnotOverwriteRsp> => {
  return request.post(`api/v1/chnot/overwrite`, req);
};

export interface ChnotDeletionReq {
  chnot_id: string;
  logic: boolean;
}

export interface ChnotDeletionRsp {}

export const deleteChnot = async (
  req: ChnotDeletionReq
): Promise<ChnotDeletionRsp> => {
  return request.post(`api/v1/chnot/deletion`, { req });
};

export interface ChnotQueryReq {
  query?: string;
  start_index: number;
  page_size: number;
}

export interface ChnotQueryRsp {
  data: Chnot[];
  this_start: number;
  next_start: number;
}

export const queryChnot = async (
  req: ChnotQueryReq
): Promise<ChnotQueryRsp> => {
  return request.query(`api/v1/chnot/query`, req);
};
