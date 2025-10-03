import { TID } from "@/lib/id_util";
import { DbText, Varchar } from "@/lib/types";
import { TodoEvent, TodoStateEnum, TodoPriorityEnum } from "../toent/po";

export type MdwtRecord = {
  otid: TID;
  tid: TID;
  todo_event?: TodoEvent;
  content: DbText;
  archor: boolean;
};
export type MdwtTag = {
  tag: Varchar<800>;
  mdwt_otid: TID;
  kspace: Varchar<40>;
  tid: TID;
};
