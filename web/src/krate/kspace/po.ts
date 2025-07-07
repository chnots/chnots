import { OmitTID, TID } from "@/lib/id_util";
import { Varchar } from "@/lib/types";

export type KSpace = {
  name: Varchar<500>;
  omit_tid?: OmitTID;
  color: Varchar<100>;
  managers: string[];
  tid: TID;
};
