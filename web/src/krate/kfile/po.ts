import type { TID } from "@/lib/id_util";
import type { DbText, Varchar } from "@/lib/types";

export type InlineKFile = {
  sid: Varchar<100>;
  tid: TID;
  content: DbText;
};

export type KFileMeta = {
  otid: TID;
  id: Varchar<100>;
  inline: boolean;
  archor: boolean;
  filename: Varchar<1024>;
  content_type: Varchar<200>;
  last_modified: TID;
  sid: Varchar<100>;
  filesize: number;
  tid: TID;
};
