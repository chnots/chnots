import { TID as TID } from "@/lib/id_util";
import { DbText, Varchar } from "@/lib/types";
import { TodoEvent, TodoPriorityEnum, TodoStateEnum } from "../toent/po";

export enum ChnotKind {
  MarkdownWithToent = "mdwt",
  ExcalidrawV1 = "exdrv1",
  KFileV1 = "resov1",
  KTab = "ktabv1",
  LLMChat = "llm_chat",
}

// TODO: true type
export type ChnotTagType = object;

export type MdwtRecord = {
  otid: TID;
  tid: TID;
  todo_event?: TodoEvent;
  content: DbText;
  archor: boolean;
};

export type ChnotMetadata = {
  otid: TID;
  kspace: Varchar<40>;
  pin_time?: Date;
  archive_time?: Date;
  tid: TID;
};

export type ChnotTag = {
  tag: Varchar<800>;
  meta_otid: TID;
  kspace: Varchar<40>;
  tid: TID;
};

export type ChnotBlockMeta = {
  otid: TID;
  chnot_otid: TID;
  kind: ChnotKind;
  kind_id: Varchar<200>;
  korder: number;
  tid: TID;
};

export type ChnotBlockToent = {
  block_otid: TID;
  chnot_otid: TID;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  todo_closed: boolean;
  note?: DbText;
  tid: TID;
};
