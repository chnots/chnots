import request from "@/lib/request";
import {
  GetSyncAllEndpointsReq,
  GetSyncAllEndpointsRsp,
  SyncAllEndpointsReq,
  SyncAllEndpointsRsp,
  SyncToEndpointReq,
  SyncToEndpointRsp,
} from "./dto";

export const getSyncAllEndpoints = async (
  req: GetSyncAllEndpointsReq,
): Promise<GetSyncAllEndpointsRsp> => {
  return await request.postJson(`/api/v1/sync-endpoint-list`, req);
};

export const overwriteSyncAllEndpoints = async (
  req: SyncAllEndpointsReq,
): Promise<SyncAllEndpointsRsp> => {
  return await request.postJson(`/api/v1/sync-endpoint-commit`, req);
};

export const syncToEndpoint = async (
  req: SyncToEndpointReq,
): Promise<SyncToEndpointRsp> => {
  return await request.postJson(`/api/v1/sync-endpoint-sync`, req);
};
