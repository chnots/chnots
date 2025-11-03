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
  title: Varchar<500>;
  kspace: Varchar<40>;
  pin_tid?: Date;
  archive_tid?: Date;
  tid: TID;
};

export type ChnotMeta = {
  otid: TID;
  kind: ChnotKind;
  kspace: Varchar<40>;
  archive_tid?: TID;
  pin_tid?: TID;
  tid: TID;
};

export type ChnotThreadOrder = {
  otid: TID;
  thread_otid: TID;
  korder: number;
  tid: TID;
};
