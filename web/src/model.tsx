export enum ChnotType {
  MarkdownWithToent = "mdwt",
}

export interface Chnot {
  id: string;
  perm_id: string;
  content: string;
  type: ChnotType;
  delete_time?: Date;
  insert_time: Date;
  update_time: Date;
}
