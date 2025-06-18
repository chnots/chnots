import { SharedStr } from "@/lib/types";
import { KKV } from "./po";

export type KKVType = object;

export type KKVQueryOneReq = {
  key: string;
  kind: KKVType;
};            

export type KKVQueryManyReq = {
  key?: string;
  kind?: KKVType;
  kspace?: SharedStr;
};              

export type KKVQueryOneRsp = {
  value?: string;
};            

export type KKVQueryManyRsp = {
  kkvs: KKV[];
};              

export type KKVOverwriteReq = {
  key: string;
  kind: KKVType;
  value: string;
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