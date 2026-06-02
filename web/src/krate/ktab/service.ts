import request from "@/lib/request";
import type {
  KTabCellCommitReq,
  KTabCellListReq,
  KTabCellListRsp,
  KTabMetaCommitRsp,
  KTabMetaFetchReq,
  KTabMetaFetchRsp,
  KTabOverwriteCellsRsp,
  KTabOverwriteMetaReq,
  KTabRowDeleteReq,
  KTabRowDeleteRsp,
} from "./dto";

export const ktabMetaFetch = async (
  req: KTabMetaFetchReq,
): Promise<KTabMetaFetchRsp> => {
  return await request.postJson(`/api/v1/ktab-meta-fetch`, req);
};

export const ktabMetaCommit = async (
  req: KTabOverwriteMetaReq,
): Promise<KTabMetaCommitRsp> => {
  return await request.postJson(`/api/v1/ktab-meta-commit`, req);
};

export const ktabCellList = async (
  req: KTabCellListReq,
): Promise<KTabCellListRsp> => {
  return await request.postJson(`/api/v1/ktab-cell-list`, req);
};

export const ktabCellCommit = async (
  req: KTabCellCommitReq,
): Promise<KTabOverwriteCellsRsp> => {
  return await request.postJson(`/api/v1/ktab-cell-commit`, req);
};

export const ktabRowDelete = async (
  req: KTabRowDeleteReq,
): Promise<KTabRowDeleteRsp> => {
  return await request.postJson(`/api/v1/ktab-row-delete`, req);
};
