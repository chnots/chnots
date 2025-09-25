import { TID } from "@/lib/id_util";
import { Varchar } from "@/lib/types";

export type Point2PointInfo = object;

export type SyncLogTransient = {
  remote_id: Varchar<100>;
  table_name: Varchar<100>;
  end_sync_in: TID;
  start_tid_ex: TID;
  sync_finish_tid: TID;
};

export type SyncEndpoint = {
  ip: string;
  port: number;
};

export type SyncAllEndpoints = {
  endpoints: SyncEndpoint[];
};

export type TidCompare = {
  tid: TID;
  lstate: object;
  rstate: object;
};

export type SyncLogTransientCommit = {
  remote_id: Varchar<100>;
  table_name: Varchar<100>;
  end_sync_in: TID;
  start_tid_ex: TID;
  sync_finish_tid: TID;
};
