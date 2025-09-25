import request from "@/lib/request";
import { MdwtCommitRsp } from "../chnot/dto";
import {
  MdwtCommitReq,
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
