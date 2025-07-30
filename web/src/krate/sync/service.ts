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
  req: GetSyncAllEndpointsReq
): Promise<GetSyncAllEndpointsRsp> => {
  return await request.get(`/api/v1/get-all-sync-endpoints`, req);
};

export const overwriteSyncAllEndpoints = async (
  req: SyncAllEndpointsReq
): Promise<SyncAllEndpointsRsp> => {
  return await request.post(`/api/v1/overwrite-all-sync-endpoints`, req);
};

export const syncToEndpoint = async (
  req: SyncToEndpointReq
): Promise<SyncToEndpointRsp> => {
  return await request.post(`/api/v1/sync-end-endpoint`, req);
};
