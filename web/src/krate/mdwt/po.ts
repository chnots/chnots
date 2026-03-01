import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";
import type { TodoEvent } from "../toent/po";

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
  tid: TID;
};
