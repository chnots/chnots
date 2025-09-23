import request from "@/lib/request";
import {
  ChnotThreadQueryReq,
  ChnotThreadQueryRsp,
  ChnotTagQueryReq,
  ChnotOverwriteThreadMetaReq,
  MdwtRecordsReq,
  MdwtRecordsRsp,
  ChnotThreadMetaRsp,
  ChnotTagQueryRsp,
  ChnotOverwriteThreadOrderReq,
  ChnotOverwriteThreadOrderRsp,
  ChnotOverwriteMdwtReq,
  ChnotOverwriteMdwtRsp,
  ChnotOverwriteThreadMetaRsp,
  ChnotOverwriteMetaReq,
  ChnotOverwriteMetaRsp,
} from "./dto";
import { TID } from "@/lib/id_util";
import { ToentGuessReq, ToentGuessRsp } from "../toent/dto";
import { TodoEvent, ToentTimeEvent } from "../toent/po";

export const chnotThreadQuery = async (
  req: ChnotThreadQueryReq,
): Promise<ChnotThreadQueryRsp> => {
  return await request.postJson(`api/v1/query-chnot-thread`, req);
};

export const chnotOverwriteMdwts = async (
  req: ChnotOverwriteMdwtReq,
): Promise<ChnotOverwriteMdwtRsp> => {
  return await request.putJson(`api/v1/put-chnot-mdwts`, req);
};

export async function chnotOverwriteMetas(
  req: ChnotOverwriteMetaReq,
): Promise<ChnotOverwriteMetaRsp> {
  return await request.putJson(`api/v1/put-chnot-metas`, req);
}

export async function ChnotOverwriteThreadOrders(
  req: ChnotOverwriteThreadOrderReq,
): Promise<ChnotOverwriteThreadOrderRsp> {
  return await request.putJson(`api/v1/put-chnot-thread-orders`, req);
}

export const MdwtRecords = async (
  req: MdwtRecordsReq,
): Promise<MdwtRecordsRsp> => {
  return await request.postJson(`api/v1/query-mdwt-records`, req);
};

export const getChnotThreadMeta = async (
  chnot_otid: TID,
): Promise<ChnotThreadMetaRsp> => {
  return await request.get(`api/v1/get-chnot-thread-meta/${chnot_otid}`);
};

export const chnotThreadOverwriteMeta = async (
  req: ChnotOverwriteThreadMetaReq,
): Promise<ChnotOverwriteThreadMetaRsp> => {
  return await request.postJson(`api/v1/put-chnot-thread-meta`, req);
};

export const chnotThreadTagNames = async (
  req: ChnotTagQueryReq,
): Promise<ChnotTagQueryRsp<string>> => {
  return await request.postJson(`api/v1/query-chnot-tag-names`, req);
};

export const toentTimeEventGuess = async (
  req: ToentGuessReq,
): Promise<ToentGuessRsp<ToentTimeEvent>> => {
  return await request.postJson(`api/v1/toent-timeevent-guess`, req);
};

export const toentTodoEventGuess = async (
  req: ToentGuessReq,
): Promise<ToentGuessRsp<TodoEvent>> => {
  return await request.postJson(`api/v1/toent-todoevent-guess`, req);
};
