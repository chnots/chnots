import type { PageRsp } from "@/common/types";
import request from "@/lib/request";
import type {
  ChnotMetaCommitReq,
  ChnotMetaCommitRsp,
  ChnotMetaListReq,
  ChnotMetaListRsp,
  ChnotSearchReq,
  ChnotSearchRspData,
  ChnotThreadMetaFetchReq,
  ChnotThreadMetaFetchRsp,
  ChnotThreadOrderArchiveReq,
  ChnotThreadOrderArchiveRsp,
  ChnotThreadOrderCommitReq,
  ChnotThreadOrderCommitRsp,
} from "./dto";

export const chnotSearch = async (
  req: ChnotSearchReq,
): Promise<PageRsp<ChnotSearchRspData>> => {
  return await request.postJson(`api/v1/chnot-search`, req);
};

export async function chnotMetaCommit(
  req: ChnotMetaCommitReq,
): Promise<ChnotMetaCommitRsp> {
  return await request.postJson(`api/v1/chnot-meta-commit`, req);
}

export async function chnotMetaList(
  req: ChnotMetaListReq,
): Promise<ChnotMetaListRsp> {
  if (req.otids.length === 0) {
    return { metas: [] };
  }
  return await request.postJson(`api/v1/chnot-meta-list`, req);
}

export async function chnotThreadOrderCommit(
  req: ChnotThreadOrderCommitReq,
): Promise<ChnotThreadOrderCommitRsp> {
  return await request.postJson(`api/v1/chnot-thread-order-commit`, req);
}


export async function chnotThreadOrderArchive(
  req: ChnotThreadOrderArchiveReq,
): Promise<ChnotThreadOrderArchiveRsp> {
  return await request.postJson(`api/v1/chnot-thread-order-archive`, req);
}

export const chnotThreadMetaFetch = async (
  req: ChnotThreadMetaFetchReq,
): Promise<ChnotThreadMetaFetchRsp> => {
  return await request.postJson(`api/v1/chnot-thread-meta-fetch`, req);
};
