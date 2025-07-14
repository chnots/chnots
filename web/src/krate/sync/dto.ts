import { TID } from "@/lib/id_util";

type TheSameKey = Object;

export type SyncShakeReq = {
  client_id: string;
  client_app_version: string;
  server_id: string;
};
export type SyncShakeRsp = {
  stop: boolean;
  last_sync_time?: TID;
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
  table_name: string;
  tids: TID[];
};
