import { OmitTID, TID as TID } from "@/lib/id_util";
import { DbText, Varchar } from "@/lib/types";
import { TodoEvent } from "../toent/po";

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
  meta_tid: TID;
  omit_tid?: OmitTID;
  tid: TID;
  todo_event?: TodoEvent;
  content: DbText;
  archor: boolean;
};

export type ChnotMetadata = {
  tid: TID;
  kspace: Varchar<40>;
  kind: ChnotKind;
  pin_time?: Date;
  omit_tid?: OmitTID;
  archive_time?: Date;
};

export type ChnotTag = {
  tag: Varchar<800>;
  meta_tid: TID;
  omit_tid?: OmitTID;
  kspace: Varchar<40>;
  tid: TID;
};

export type ChnotKindId = {
  meta_tid: TID;
  omit_tid?: OmitTID;
  kind_id: string;
};

export type ChnotKindRel = {
  meta_tid: TID;
  omit_tid?: OmitTID;
  kind_id: Varchar<200>;
  tid: TID;
};
