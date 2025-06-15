import { TID } from "@/lib/id_util";
import { ChnotRecord, ChnotMetadata, ChnotTag, ChnotKind } from "./po";

export type Chnot= {
  record: ChnotRecord;
  meta: ChnotMetadata;
}

export type ChnotTagTreeType = {
  kind: "tagtree";
  tagkind: "children" | "descendants";
  tagpath: string;
};
export type ListViewType = { kind: "timeline" } | ChnotTagTreeType;
export const listViewTypeGetTagPath = (lvt: ListViewType) => {
  if (lvt.kind === "tagtree") {
    return lvt.tagpath;
  } else {
    return undefined;
  }
};

export type ChnotQueryReq= {
  query?: string;
  meta_tid?: TID;
  record_tid?: TID;

  view_type: ListViewType;
  kinds: ChnotKind[];

  with_omited?: boolean;
  start_index: number;
  page_size: number;
}

export type ChnotQueryRsp= {
  next_start: number;
  data: Chnot[];

  has_next: boolean;
}

export type ChnotOverwriteReq= {
  tid?: TID;
  meta_tid?: TID;
  content: string;
  kind: string;
  kind_id?: string;
}

export type ChnotOverwriteRsp= {
  meta_tid: TID;
  rec_tid: TID;
  archor: boolean;
  kspace: string;
}

export type ChnotDeletionReq= {
  chnot_id: TID;
  logic: boolean;
}

export type ChnotUpdateReq= {
  meta_tid: TID;

  kspace?: string;

  pinned?: boolean;
  archive?: boolean;

  update_time: boolean;
}

export type ChnotCommentAddReq= {
  tid: TID;

  chnot_meta_id: TID;
  content: string;
}

export type ChnotTagQueryReq= {
  query?: string;
  tag_tree: ChnotTagTreeType;

  start_index: number;
  page_size: number;
}

export type ChnotKFileRelationInsert = {
  chnot_meta_id: TID;
};

export type ChnotTagQueryRsp= {
  data: ChnotTag[];

  start_index: number;
}

export type ChnotTagNamesRsp= {
  data: string[];

  start_index: number;
}

export type Toent= {
  tid: TID;
  input: string;
  event: string;
}

export type ToentGuessReq= {
  input: string;
}

export type ToentGuessRsp= {
  toents: Toent[];
}
