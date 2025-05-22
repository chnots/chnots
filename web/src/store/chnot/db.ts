export enum ChnotType {
  MarkdownWithToent = "mdwt",
  ExcalidrawV1 = "exdrv1",
  KFileV1 = "resov1",
}

export interface ChnotRecord {
  id: string;
  meta_id: string;
  content: string;
  omit_time?: Date;
  insert_time: Date;
}

export interface ChnotMetadata {
  id: string;
  workspace: string;
  kind: string;
  pin_time?: Date;
  delete_time?: Date;
  update_time?: Date;
  insert_time: Date;
}

export interface ChnotTag {
  id: string;
  workspace: string;
  tag: string;
  chnot_meta_id: string;
  insert_time: Date;
}
