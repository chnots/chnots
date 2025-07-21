import { OmitTID, TID } from "@/lib/id_util";
import { Varchar } from "@/lib/types";

export type KSpace = {
  name: Varchar<500>;
  color: Varchar<100>;
  managers: string[];
  tid: TID;
};
