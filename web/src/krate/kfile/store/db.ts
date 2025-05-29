import { KSpace } from "../../model";

export interface KFile {
  id: string;

  kspace?: string;
  ori_filename: string;
  ori_last_modified: number;
  filesize: number;

  content_type: string;

  delete_time?: string; // Using ISO 8601 format for DateTime
  insert_time: string; // Using ISO 8601 format for DateTime
}

export interface InlineKFile {
  id: string;
  rid: string;
  kspace: string;
  archor: boolean;
  name: string;
  content: string;
  content_type: string;
  delete_time?: Date;
  insert_time: Date;
}
