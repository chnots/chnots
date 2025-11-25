import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";

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
