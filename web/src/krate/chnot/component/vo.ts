import type { TID } from "@/lib/id_util";
import type { ChnotKind } from "../po";

export type ChnotMetaKind = {
  chnotOtid: TID;
  kind: ChnotKind;
};
