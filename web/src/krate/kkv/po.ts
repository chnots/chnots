import { KKVType } from "./dto";

export type KKV = {
  key: string;
  kind: KKVType;
  kspace: string;
  value: string;
  update_time?: Date;
  insert_time: Date;
};              
