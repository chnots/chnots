import { TID } from "@/lib/id_util";
import { KKVType } from "./dto";

export type KKV = {
  key: string;
  kind: KKVType;
  kspace: string;
  tid: TID;
  value: string;
  update_time?: Date;
};
