import request from "@/lib/request";
import {
  ChnotThreadListReq,
  ChnotThreadListRsp,
  ChnotThreadMetaFetchCommitReq,
  ChnotThreadMetaFetchRsp,
  ChnotThreadOrderCommitReq,
  ChnotThreadOrderCommitRsp,
  ChnotThreadMetaFetchCommitRsp,
  ChnotMetaCommitReq,
  ChnotMetaCommitRsp,
  ChnotThreadMetaFetchReq,
  ChnotMetaListReq,
  ChnotMetaListRsp,
} from "./dto";

export const chnotThreadList = async (
  req: ChnotThreadListReq,
): Promise<ChnotThreadListRsp> => {
  return await request.postJson(`api/v1/chnot-thread-list`, req);
};

export async function chnotMetaCommit(
  req: ChnotMetaCommitReq,
): Promise<ChnotMetaCommitRsp> {
  return await request.postJson(`api/v1/chnot-meta-commit`, req);
}

export async function chnotMetaList(
  req: ChnotMetaListReq,
): Promise<ChnotMetaListRsp> {
  return await request.postJson(`api/v1/chnot-meta-list`, req);
}

export async function chnotThreadOrderCommit(
  req: ChnotThreadOrderCommitReq,
): Promise<ChnotThreadOrderCommitRsp> {
  return await request.postJson(`api/v1/chnot-thread-order-commit`, req);
}

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
