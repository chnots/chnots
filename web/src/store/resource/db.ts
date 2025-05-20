import { Namespace } from '../../model';

export interface Resource {
  id: string;

  namespace?: string;
  ori_filename: string;
  ori_last_modified: number;
  filesize: number;

  content_type: string;

  delete_time?: string; // Using ISO 8601 format for DateTime
  insert_time: string; // Using ISO 8601 format for DateTime
}

export interface InlineResource {
  id: string;
  rid: string;
  namespace: string;
  archor: boolean;
  name: string;
  content: string;
  content_type: string;
  delete_time?: Date;
  insert_time: Date;
}