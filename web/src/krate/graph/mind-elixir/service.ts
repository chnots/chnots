import type { MindElixirData } from "mind-elixir";
import type { KfileMetaFetchReqId } from "@/krate/kfile/dto";
import { inlineKFileDownload, inlineKFileUpload } from "@/krate/kfile/service";
import { genTID, genUID, type TID } from "@/lib/id_util";
import request from "@/lib/request";
import type {
  GraphHistoryListReq,
  GraphHistoryListRsp,
  MindElixirCommitReq,
  MindElixirCommitRsp,
  MindElixirHistoryApplyReq,
  MindElixirHistoryApplyRsp,
  MindElixirHistoryFetchReq,
  MindElixirHistoryFetchRsp,
  MindElixirLoadReq,
  MindElixirLoadRsp,
} from "../dto";

export type MindElixirChnotData = {
  otid: TID;
  data: MindElixirData;
};

export const fetchMindExilirInner = async (
  req: MindElixirLoadReq,
): Promise<MindElixirLoadRsp> => {
  return await request.postJson(`api/v1/mind-elixir-fetch`, req);
};

export const saveMindExilirInner = async (
  req: MindElixirCommitReq,
): Promise<MindElixirCommitRsp> => {
  return await request.postJson(`api/v1/mind-elixir-commit`, req);
};

export const mindElixirHistoryListInner = async (
  req: GraphHistoryListReq,
): Promise<GraphHistoryListRsp> => {
  return await request.postJson(`api/v1/mind-elixir-history-list`, req);
};

export const mindElixirHistoryFetchInner = async (
  req: MindElixirHistoryFetchReq,
): Promise<MindElixirHistoryFetchRsp> => {
  return await request.postJson(`api/v1/mind-elixir-history-fetch`, req);
};

export const mindElixirHistoryApplyInner = async (
  req: MindElixirHistoryApplyReq,
): Promise<MindElixirHistoryApplyRsp> => {
  return await request.postJson(`api/v1/mind-elixir-history-apply`, req);
};

export const fetchMindExilir = async (
  id: TID,
): Promise<MindElixirData | null> => {
  try {
    const rsp = await fetchMindExilirInner({
      otid: id,
    });
    if (!rsp.data) {
      return null;
    }
    return rsp.data;
  } catch (_e) {}
  return null;
};

export type SaveMindExilirProps = {
  otid: TID;
  data: MindElixirData;
  onSuccess: () => void;
  onFail: () => void;
};

export const saveMindExilir = async (props: SaveMindExilirProps) => {
  const { otid, data, onSuccess, onFail } = props;

  try {
    await saveMindExilirInner({
      otid,
      data,
    });
    onSuccess();
  } catch (_err) {
    onFail();
  }
};
