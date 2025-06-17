import { OmitTID, TID as TID } from "@/lib/id_util";
import { object } from "zod";

export enum ChnotKind {
  MarkdownWithToent = "mdwt",
  ExcalidrawV1 = "exdrv1",
  KFileV1 = "resov1",
  KTab = "ktabv1",
  LLMChat = "llm_chat",
}

// TODO: true type
export type ChnotTagType = object;

export type ChnotRecord = {
  tid: TID;
  meta_tid: TID;
  omit_tid?: OmitTID;
  content: string;
  archor: boolean;
};                       

export type ChnotMetadata = {
  tid: TID;
  kspace: string;
  kind: ChnotKind;
  pin_time?: Date;
  omit_tid?: OmitTID;
  archive_time?: Date;
};                       

export type ChnotTag = {
  tid: TID;
  omit_tid?: OmitTID;
  kspace: string;
  tag: string;
  category: ChnotTagType;
  meta_tid: TID;
};                       

export type ChnotKindId = {
  meta_tid: TID;
  omit_tid?: OmitTID;
  kind_id: string;
};           

export type ChnotKindRel = {
  meta_tid: TID;
  omit_tid?: OmitTID;
  kind_id: string;
};            