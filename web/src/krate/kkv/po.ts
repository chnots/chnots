import { OmitTID, TID } from "@/lib/id_util";
import { KKVType } from "./dto";
import { DbText, Varchar } from "@/lib/types";

export type KKV = {
  key: Varchar<500>;
  kind: Varchar<100>;
  kspace: Varchar<40>;
  tid: TID;
  archor: boolean;
  value: DbText;
};

export type KKVTransient = {
  key: Varchar<500>;
  value: DbText;
  tid: TID;
};
