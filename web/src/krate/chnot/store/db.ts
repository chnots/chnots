import { TID as TID } from "@/lib/id_util";

export enum ChnotKind {
  MarkdownWithToent = "mdwt",
  ExcalidrawV1 = "exdrv1",
  KFileV1 = "resov1",
  KTab = "ktabv1",
  LLMChat = "llm_chat",
}

export interface ChnotRecord {
  tid: TID;
  meta_tid: TID;
  content: string;
  omit_tid?: TID;
  archor: boolean;
}

export interface ChnotMetadata {
  tid: TID;
  kspace: string;
  kind: string;
  pin_time?: Date;
  omit_tid?: TID;
  archive_time?: Date;
}

export interface ChnotTag {
  tid: TID;
  kspace: string;
  tag: string;
  meta_id: TID;
}
