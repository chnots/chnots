import request from "@/lib/request";
import {
  ChnotThreadArchiveReq,
  ChnotThreadQueryReq,
  ChnotThreadQueryRsp,
  ChnotThreadTagQueryReq,
  ChnotOverwriteThreadMetaReq,
  MdwtRecordsReq,
  MdwtRecordsRsp,
  ChnotThreadMetaRsp,
  ChnotThreadTagQueryRsp,
  ChnotOverwriteMetaReq,
  ChnotOverwriteMetaRsp,
  ChnotOverwriteMdwtReq,
  ChnotOverwriteMdwtRsp,
} from "./dto";
import { TID } from "@/lib/id_util";
import { ToentGuessReq, ToentGuessRsp } from "../toent/dto";
import { TodoEvent, ToentTimeEvent } from "../toent/po";

export const chnotQuery = async (
  req: ChnotThreadQueryReq,
): Promise<ChnotThreadQueryRsp> => {
  return await request.postJson(`api/v1/chnot-query`, req);
};

export const chnotDelete = async (req: ChnotThreadArchiveReq) => {
  return await request.postJson(`api/v1/chnot-deletion`, req);
};

export const chnotOverwriteMdwts = async (
  req: ChnotOverwriteMdwtReq,
): Promise<ChnotOverwriteMdwtRsp> => {
  return await request.putJson(`api/v1/chnot-overwrite-mdwts`, req);
};

export const chnotOverwriteBlockMetas = async (
  req: ChnotOverwriteMetaReq,
): Promise<ChnotOverwriteMetaRsp> => {
  return await request.putJson(`api/v1/chnot-overwrite-block-metas`, req);
};

export const MdwtRecords = async (
  req: MdwtRecordsReq,
): Promise<MdwtRecordsRsp> => {
  return await request.postJson(`api/v1/mdwt-blocks`, req);
};

export const chnotMeta = async (
  chnot_otid: TID,
): Promise<ChnotThreadMetaRsp> => {
  return await request.get(`api/v1/chnot-meta/${chnot_otid}`);
};

export const chnotOverwriteMeta = async (req: ChnotOverwriteThreadMetaReq) => {
  return await request.postJson(`api/v1/chnot-overwrite-meta`, req);
};

export const chnotTagNames = async (
  req: ChnotThreadTagQueryReq,
): Promise<ChnotThreadTagQueryRsp<string>> => {
  return await request.postJson(`api/v1/chnot-tag-names`, req);
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
