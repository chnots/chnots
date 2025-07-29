import { DbText, SharedStr, Varchar } from "@/lib/types";
import { KKV } from "./po";
import { TID } from "@/lib/id_util";

export type KKVType = object;

export type KKVQueryOneReq = {
  key: Varchar<500>;
  kind: Varchar<100>;
};

export type KKVQueryManyReq = {
  key?: Varchar<500>;
  kind?: Varchar<100>;
  kspace?: string;
};

export type KKVQueryOneRsp = {
  value?: DbText;
  tid?: TID;
};

export type KKVQueryManyRsp = {
  kkvs: KKV[];
};

export type KKVOverwriteReq = {
  key: Varchar<500>;
  kind: Varchar<100>;
  value: DbText;
};

export type KKVOverwriteRsp = object;

export type KKVDeleteReq = {
  key: string;
  kind: string;
};

export type KKVDeleteRsp = object;

export type KKVInserterReq = {
  key: string;
  kind: string;
  value: string;
};

export type KKVInserterRsp = object;

export type KKVQueryReq = {
  key: string;
  kind: "to_file" | "chnot_sub_type";
};

export type KKVQueryRsp = {
  value?: string;
};
