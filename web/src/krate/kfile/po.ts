import { TID } from "@/lib/id_util";

export type KFile = {
  sid: string;
  tid: TID;

  kspace?: string;
  ori_filename: string;
  ori_last_modified: number;
  filesize: number;

  content_type: string;

  omit_tid?: string; // Using ISO 8601 format for DateTime
};

export type InlineKFile = {
  sid: string;
  tid: TID;
  name: string;
  content: string;
  content_type: string;
};
