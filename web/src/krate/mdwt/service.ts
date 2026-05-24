import request from "@/lib/request";
import type { MdwtCommitRsp } from "../chnot/dto";
import type {
  MdwtCommitReq,
  MdwtContentLoadReq,
  MdwtContentLoadRsp,
  MdwtHistoryApplyReq,
  MdwtHistoryApplyRsp,
  MdwtHistoryFetchReq,
  MdwtHistoryFetchRsp,
  MdwtHistoryListReq,
  MdwtHistoryListRsp,
  MdwtRecordsReq,
  MdwtRecordsRsp,
  MdwtTagListReq,
  MdwtTagListRsp,
} from "./dto";

export const mdwtCommit = async (
  req: MdwtCommitReq,
): Promise<MdwtCommitRsp> => {
  return await request.postJson(`api/v1/mdwt-commit`, req);
};

export const mdwtRecordList = async (
  req: MdwtRecordsReq,
): Promise<MdwtRecordsRsp> => {
  return await request.postJson(`api/v1/mdwt-list`, req);
};
export const chnotTagNameList = async (
  req: MdwtTagListReq,
): Promise<MdwtTagListRsp<string>> => {
  return await request.postJson(`api/v1/mdwt-tag-name-list`, req);
};

export const allMdwtTagRefresh = async (): Promise<void> => {
  return await request.postJson(`api/v1/all-mdwt-tag-refresh`);
};

export const mdwtHistoryList = async (
  req: MdwtHistoryListReq,
): Promise<MdwtHistoryListRsp> => {
  return await request.postJson(`api/v1/mdwt-history-list`, req);
};

export const mdwtHistoryFetch = async (
  req: MdwtHistoryFetchReq,
): Promise<MdwtHistoryFetchRsp> => {
  return await request.postJson(`api/v1/mdwt-history-fetch`, req);
};

export const mdwtHistoryApply = async (
  req: MdwtHistoryApplyReq,
): Promise<MdwtHistoryApplyRsp> => {
  return await request.postJson(`api/v1/mdwt-history-apply`, req);
};

export const mdwtContentLoad = async (
  req: MdwtContentLoadReq,
): Promise<MdwtContentLoadRsp> => {
  return await request.postJson(`api/v1/mdwt-content-load`, req);
};
