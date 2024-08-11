import request from "@/helpers/request";
import { create } from "zustand";
import { combine } from "zustand/middleware";

export enum ChnotType {
  MarkdownWithToent = "mdwt",
}

export interface ChnotComment {
  id: string;
  chnot_perm_id: string;
  content: string;
  insert_time: Date;
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
  comments?: ChnotComment[];
}

const getDefaultState = (): State => {
  return {
    fetchMoreChnots: () => {},
    chnotPages: [],
    pageSize: 20,
    startIndex: 0,
    query: undefined,
    isFetchingNextPage: false,
    hasNextPage: true,
  };
};

export interface ChnotQueryReq {
  query?: string;
  start_index: number;
  page_size: number;
}

export interface ChnotQueryRsp {
  data: Chnot[];

  this_start: number;
  next_start: number;
  has_more: boolean;
}

export interface ChnotOverwriteReq {
  chnot: Chnot;
}

export interface ChnotOverwriteRsp {}

export interface ChnotDeletionReq {
  chnot_id: string;
  logic: boolean;
}

export interface ChnotDeletionRsp {}

interface ChnotPage {
  chnots: Chnot[];
  index: number;
}

export interface ChnotUpdateReq {
  chnot_id: string;

  pinned?: boolean;
  archive?: boolean;

  update_time: boolean;
}

export interface ChnotCommentAddReq {
  id: string;

  chnot_perm_id: string;
  content: string;

  insert_time: Date;
}

interface State {
  fetchMoreChnots(): unknown;

  pageSize: number;
  startIndex: number;

  query?: string;

  chnotPages: ChnotPage[];

  isFetchingNextPage: boolean;
  hasNextPage: boolean;
}

export const useChnotStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    fetchMoreChnots: async () => {
      set((state) => {
        return {
          ...state,
          isFetchingNextPage: true,
        };
      });

      const read = get();
      const cs: ChnotQueryRsp = await request.query(`api/v1/chnot/query`, {
        start_index: read.startIndex,
        page_size: read.pageSize,
        query: read.query,
      });

      set((state) => {
        const arr = state.chnotPages;

        arr.push({
          chnots: cs.data,
          index: cs.this_start,
        });

        return {
          ...state,
          chnots: arr,
          startIndex: cs.next_start,
          hasNextPage: cs.has_more,
          isFetchingNextPage: false,
        };
      });
    },
    changeQuery: async (query?: string) => {
      set((state) => {
        return { ...state, query: query, startIndex: 0 };
      });

      await get().fetchMoreChnots();
    },
    refreshChnots: async () => {
      set((state) => {
        return { ...state, startIndex: 0, chnotPages: [] };
      });

      await get().fetchMoreChnots();
    },
    deleteChnot: async (req: ChnotDeletionReq) => {
      return request.post(`api/v1/chnot/deletion`, req);
    },
    overwriteChnot: async (req: ChnotOverwriteReq) => {
      return request.post(`api/v1/chnot/overwrite`, req);
    },
    updateChnot: async (req: ChnotUpdateReq) => {
      return request.post(`api/v1/chnot/update`, req);
    },
    addComment: async (req: ChnotCommentAddReq) => {
      return request.put(`api/v1/chnot/comment`, req);
    },
  }))
);
