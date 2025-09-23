import request from "@/lib/request";
import {
  ChnotThreadListReq,
  ChnotThreadListRsp,
  ChnotTagListReq,
  ChnotThreadMetaFetchCommitReq,
  MdwtRecordsReq,
  MdwtRecordsRsp,
  ChnotThreadMetaFetchRsp,
  ChnotTagListRsp,
  ChnotThreadOrderCommitReq,
  ChnotThreadOrderCommitRsp,
  ChnotMdwtCommitReq,
  ChnotMdwtCommitRsp,
  ChnotThreadMetaFetchCommitRsp,
  ChnotMetaCommitReq,
  ChnotMetaCommitRsp,
  ChnotThreadMetaFetchReq,
} from "./dto";
import { TID } from "@/lib/id_util";
import { ToentGuessReq, ToentGuessRsp } from "../toent/dto";
import { TodoEvent, ToentTimeEvent } from "../toent/po";

export const chnotThreadList = async (
  req: ChnotThreadListReq,
): Promise<ChnotThreadListRsp> => {
  return await request.postJson(`api/v1/chnot-thread-list`, req);
};

export const chnotMdwtCommit = async (
  req: ChnotMdwtCommitReq,
): Promise<ChnotMdwtCommitRsp> => {
  return await request.postJson(`api/v1/chnot-mdwt-commit`, req);
};

export async function ChnotMetaCommit(
  req: ChnotMetaCommitReq,
): Promise<ChnotMetaCommitRsp> {
  return await request.postJson(`api/v1/chnot-meta-commit`, req);
}

export async function ChnotThreadOrderCommit(
  req: ChnotThreadOrderCommitReq,
): Promise<ChnotThreadOrderCommitRsp> {
  return await request.postJson(`api/v1/chnot-thread-order-commit`, req);
}

export const mdwtRecordList = async (
  req: MdwtRecordsReq,
): Promise<MdwtRecordsRsp> => {
  return await request.postJson(`api/v1/chnot-mdwt-list`, req);
};

export const chnotThreadMetaFetch = async (
  req: ChnotThreadMetaFetchReq,
): Promise<ChnotThreadMetaFetchRsp> => {
  return await request.postJson(`api/v1/chnot-thread-meta-fetch`, req);
};

export const chnotThreadMetaOverwrite = async (
  req: ChnotThreadMetaFetchCommitReq,
): Promise<ChnotThreadMetaFetchCommitRsp> => {
  return await request.postJson(`api/v1/chnot-thread-meta-commit`, req);
};

export const chnotTagNameList = async (
  req: ChnotTagListReq,
): Promise<ChnotTagListRsp<string>> => {
  return await request.postJson(`api/v1/chnot-tag-name-list`, req);
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
