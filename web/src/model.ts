export enum ChnotType {
  MarkdownWithToent = "mdwt",
}

export interface Chnot {
  archive_time?: Date;
  id: string;
  perm_id: string;
  content: string;
  type: ChnotType;
  domain: string;
  pinned: boolean;
  delete_time?: Date;
  insert_time: Date;
  update_time: Date;
}

export interface Domain {
  name: string;
  managers: string[];
}
