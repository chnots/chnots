import { TID as TID } from "@/lib/id_util";
import { DbText, Varchar } from "@/lib/types";
import { TodoEvent, TodoPriorityEnum, TodoStateEnum } from "../toent/po";

export enum ChnotKind {
  MDWT = "mdwt",
  ExcalidrawV1 = "exdrv1",
  KFileV1 = "resov1",
  KTab = "ktabv1",
  LLMChat = "llm_chat",
}

export type MdwtRecord = {
  otid: TID;
  tid: TID;
  todo_event?: TodoEvent;
  content: DbText;
  archor: boolean;
};

export type ChnotThreadMeta = {
  otid: TID;
  kspace: Varchar<40>;
  pin_time?: Date;
  archive_time?: Date;
  tid: TID;
};

export type ChnotTag = {
  tag: Varchar<800>;
  thread_otid: TID;
  kspace: Varchar<40>;
  tid: TID;
};

export type ChnotMeta = {
  otid: TID;
  thread_otid: TID;
  kind: ChnotKind;
  kind_id: Varchar<200>;
  korder: number;
  tid: TID;
};

export type ChnotToent = {
  chnot_otid: TID;
  thread_otid: TID;
  todo_state?: TodoStateEnum;
  todo_priority?: TodoPriorityEnum;
  todo_closed: boolean;
  note?: DbText;
  tid: TID;
};
