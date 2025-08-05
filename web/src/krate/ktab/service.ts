import request from "@/lib/request";
import {
  KTabCellsOverwriteReq,
  KTabOverwriteCellsRsp,
  KTabOverwriteMetaReq as KTabMetaOverwriteReq,
  KTabMetaOverwriteRsp as KTabMetaOverwriteRsp,
  KTabRowsQueryReq,
  KTabRowsQueryRsp,
  KTabMetaQueryReq as KTabMetaQueryReq,
  KTabMetaQueryRsp as KTabMetaQueryRsp,
} from "./dto";

export const ktabMetaRead = async (
  req: KTabMetaQueryReq,
): Promise<KTabMetaQueryRsp> => {
  return await request.postJson(`/api/v1/ktab-meta-read`, req);
};

export const ktabMetaOverwrite = async (
  req: KTabMetaOverwriteReq,
): Promise<KTabMetaOverwriteRsp> => {
  return await request.putJson(`/api/v1/ktab-meta-overwrite`, req);
};

export const ktabCellsRead = async (
  req: KTabRowsQueryReq,
): Promise<KTabRowsQueryRsp> => {
  return await request.postJson(`/api/v1/ktab-cells-read`, req);
};

export const ktabCellsOverwrite = async (
  req: KTabCellsOverwriteReq,
): Promise<KTabOverwriteCellsRsp> => {
  return await request.putJson(`/api/v1/ktab-cells-overwrite`, req);
};
