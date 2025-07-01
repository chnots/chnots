import { OmitTID, TID } from "@/lib/id_util";
import { KKVType } from "./dto";
import { DbText, Varchar } from "@/lib/types";

export type KKV = {
  key: Varchar<500>;
  kind: KKVType;
  kspace: Varchar<40>;
  omit_tid?: OmitTID;
  value: DbText;
};
