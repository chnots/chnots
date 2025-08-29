import { Varchar } from "@/lib/types";
import { ChnotMeta, ChnotKind } from "../po";
import { TID } from "@/lib/id_util";

export type ChnotMetaKind = {
  otid: TID;
  kind: ChnotKind;
  kind_id: Varchar<200>;
};
