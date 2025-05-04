import { Namespace } from '../../model';

export interface Resource {
  id: string;

  namespace?: string;
  oriFilename: string;

  contentType: string;

  deleteTime?: string; // Using ISO 8601 format for DateTime
  insertTime: string; // Using ISO 8601 format for DateTime
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