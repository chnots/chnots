import { AxiosResponse } from "axios";
import request from "./request";
import { Chnot } from "../model";

export interface ChnotOverwriteReq {
  chnot: Chnot;
}

export interface ChnotOverwriteRsp {}

export const overwriteChnot = async (
  req: ChnotOverwriteReq
): Promise<AxiosResponse<ChnotOverwriteRsp>> => {
  return request.post(`api/v1/chnot/overwrite`, req);
};

export interface ChnotDeletionReq {
  chnot_id: string;
  logic: boolean;
}

export interface ChnotDeletionRsp {}

export const deleteChnot = async (
  req: ChnotDeletionReq
): Promise<AxiosResponse<ChnotDeletionRsp>> => {
  return request.post(`api/v1/chnot/deletion`, { req });
};

export interface ChnotQueryReq {
  query: string;
  offset: number;
  limit: number;
}

export interface ChnotQueryRsp {
  result: Chnot[];
}

export const queryChnot = async (
  req: ChnotQueryReq
): Promise<AxiosResponse<ChnotQueryRsp>> => {
  return request.post(`api/v1/chnot/query`, { req });
};
