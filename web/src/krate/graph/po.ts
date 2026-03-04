import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";

export enum GraphKind {
  ExcalidrawV2 = "exdrv2",
  MindElixirV1 = "mielixirv1",
  ExcalidrawLibraryV1 = "exdrlibv1",
}

export type GraphMeta = {
  otid: TID;
  archor: boolean;
  kind: GraphKind;
  content: DbText;
  tid: TID;
};
export type GraphData = {
  sid: Varchar<100>;
  tid: TID;
  content: DbText;
};
