import type { TID } from "@/lib/id_util";
import type { Varchar } from "@/lib/types";

export enum ChnotKind {
  MDWT = "mdwt",
  ExcalidrawV1 = "exdrv1",
  KFileV1 = "resov1",
  KTab = "ktabv1",
  LLMChat = "llm_chat",
  MindMapV1 = "mindmapv1",
  ThreadV1 = "threadv1",
}

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
  closed: boolean;
  tid: TID;
};
