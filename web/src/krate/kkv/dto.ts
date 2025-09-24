import { DbText, Varchar } from "@/lib/types";
import { KKV } from "./po";
import { TID } from "@/lib/id_util";

export type KKVType = object;

export type KKVFetchReq = {
  key: Varchar<500>;
  kind: Varchar<100>;
};

export type KKVListReq = {
  key?: Varchar<500>;
  kind?: Varchar<100>;
  kspace?: string;
};

export type KKVFetchRsp = {
  value?: DbText;
  tid?: TID;
};

export type KKVListRsp = {
  kkvs: KKV[];
};

export type KKVCommitReq = {
  key: Varchar<500>;
  kind: Varchar<100>;
  value: DbText;
};

export type KKVCommitRsp = object;

export type KKVArchiveReq = {
  key: string;
  kind: string;
};

export type KKVArchiveRsp = object;
