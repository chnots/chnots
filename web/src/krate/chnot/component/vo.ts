import { Varchar } from "@/lib/types";
import { ChnotKind } from "../po";
import { TID } from "@/lib/id_util";

export type ChnotMetaKind = {
  chnotOtid: TID;
  kind: ChnotKind;
  kindId: Varchar<200>;
};
