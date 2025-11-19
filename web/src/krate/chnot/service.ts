import type {
  ChnotMetaCommitReq,
  ChnotMetaCommitRsp,
  ChnotMetaListReq,
  ChnotMetaListRsp,
  ChnotSearchReq,
  ChnotSearchRspSingle,
  ChnotThreadMetaFetchCommitReq,
  ChnotThreadMetaFetchCommitRsp,
  ChnotThreadMetaFetchReq,
  ChnotThreadMetaFetchRsp,
  ChnotThreadOrderCommitReq,
  ChnotThreadOrderCommitRsp,
} from './dto';
import type { PageRsp } from '@/common/types';
import request from '@/lib/request';

export const chnotThreadSearch = async (
  req: ChnotSearchReq,
): Promise<PageRsp<ChnotSearchRspSingle>> => {
  return await request.postJson(`api/v1/chnot-thread-search`, req);
};

export const chnotSingleSearch = async (
  req: ChnotSearchReq,
): Promise<PageRsp<ChnotSearchRspSingle>> => {
  return await request.postJson(`api/v1/chnot-single-search`, req);
};

export async function chnotMetaCommit(req: ChnotMetaCommitReq): Promise<ChnotMetaCommitRsp> {
  return await request.postJson(`api/v1/chnot-meta-commit`, req);
}

export async function chnotMetaList(req: ChnotMetaListReq): Promise<ChnotMetaListRsp> {
  if (req.otids.length == 0) {
    return { metas: [] };
  }

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
