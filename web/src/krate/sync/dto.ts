import type { SyncAllEndpoints, SyncEndpoint } from "./po";
import type { TID } from "@/lib/id_util";

type TheSameKey = object;
type SyncTableEnum = object;
type SyncShakeRspEnum = object;

export type SyncShakeReq = {
  client_id: string;
  db_version: string;
  table_name: SyncTableEnum;
  start_tid_ex: TID;
};
export type SyncShakeRsp = {
  instance_id: string;
  data: SyncShakeRspEnum;
};
export type SyncFetchSameKeyReq = {
  sync_id: string;
  table_name: string;
  start_tid_ex: string;
};
export type SyncFetchSameKeyRsp = {
  same_keys: TheSameKey[];
};
export type SyncFetchAbsentReq = {
  table_name: SyncTableEnum;
  tids: TID[];
};

type FetchDataType = object;

export type SyncFetchDataReq = {
  table_name: SyncTableEnum;
  tids: FetchDataType;
};

export type SyncShakeDto = {
  instance_id: string;
  db_version: string;
};

type SyncTIDListPage = object;

export type SyncTIDListDto = {
  page: SyncTIDListPage;
  hist: boolean;
};
export type SyncTIDListRsp = {
  data: TID[];
};
export type SyncAllEndpointsReq = {
  data: SyncAllEndpoints;
};
export type SyncAllEndpointsRsp = object;

export type GetSyncAllEndpointsReq = object;
export type GetSyncAllEndpointsRsp = {
  data: SyncAllEndpoints;
};

export type SyncToEndpointReq = {
  endpoint: SyncEndpoint;
};
export type SyncToEndpointRsp = object;
