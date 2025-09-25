import { TID as TID } from "@/lib/id_util";
import { Varchar } from "@/lib/types";

export enum ChnotKind {
  MDWT = "mdwt",
  ExcalidrawV1 = "exdrv1",
  KFileV1 = "resov1",
  KTab = "ktabv1",
  LLMChat = "llm_chat",
}

export type ChnotThreadMeta = {
  otid: TID;
  kspace: Varchar<40>;
  pin_time?: Date;
  archive_time?: Date;
  tid: TID;
};

export type ChnotMeta = {
  otid: TID;
  kind: ChnotKind;
  kind_id: Varchar<200>;
  kspace: Varchar<200>;
  tid: TID;
};

export type ChnotThreadOrder = {
  otid: TID;
  thread_otid: TID;
  korder: number;
  tid: TID;
};
