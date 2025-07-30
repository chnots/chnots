import { TID } from "@/lib/id_util";
import { SyncAllEndpoints, SyncEndpoint } from "./po";

type TheSameKey = Object;
type SyncTableEnum = Object;
type SyncShakeRspEnum = Object;

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

type SyncFetchDataPageInfo = object;

export type SyncFetchTIDDto = {
  page: SyncFetchDataPageInfo;
  hist: boolean;
};
export type SyncFetchTIDRsp = {
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
