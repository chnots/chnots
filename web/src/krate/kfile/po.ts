import { OmitTID, TID } from "@/lib/id_util";     

export type InlineKFile = {
  sid: string;
  tid: TID;
  content: string;
};                

export type KFileMeta = {
  id: string;
  omit_tid?: OmitTID;
  inline: boolean;
  archor: boolean;
  tid: TID;
  filename: string;
  content_type: string;
  last_modified: TID;
  sid: string;
  filesize: number;
};                
