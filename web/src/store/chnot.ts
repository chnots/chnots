import { insertMapAtIndex } from "@/utils/map-utils";
import request from "@/utils/request";
import { create } from "zustand";
import { combine } from "zustand/middleware";

export enum ChnotType {
  MarkdownWithToent = "mdwt",
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
  namespace: string;
  kind: string;
  pin_time?: Date;
  delete_time?: Date;
  update_time?: Date;
  insert_time: Date;
}

export interface Chnot {
  record: ChnotRecord;
  meta: ChnotMetadata;
}

const getDefaultState = (): State => {
  return {
    fetchMoreChnots: () => {},
    refreshChnots: () => {},
    chnotMap: new Map(),
    pageSize: 20,
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

  start_index: number;
}

export interface ChnotOverwriteReq {
  chnot: ChnotRecord;
  kind: string;
}

export interface ChnotOverwriteRsp {
  chnot: Chnot;
}

export interface ChnotDeletionReq {
  chnot_id: string;
  logic: boolean;
}

export interface ChnotDeletionRsp {}

export interface ChnotUpdateReq {
  chnot_id: string;

  pinned?: boolean;
  archive?: boolean;

  update_time: boolean;
}

export interface ChnotCommentAddReq {
  id: string;

  chnot_meta_id: string;
  content: string;

  insert_time: Date;
}

interface State {
  refreshChnots(): unknown;
  fetchMoreChnots(): unknown;

  pageSize: number;

  query?: string;

  chnotMap: Map<string, Chnot>;
  currentChnot?: Chnot;

  isFetchingNextPage: boolean;
  hasNextPage: boolean;
}

export const useChnotStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    queryChnot: async ({
      record_id,
      meta_id,
      with_omited,
    }: {
      record_id?: string;
      meta_id?: string;
      with_omited?: boolean;
    }) => {
      const cs: ChnotQueryRsp = await request.post(`api/v1/chnot-query`, {
        record_id,
        meta_id,
        with_omited,
        start_index: 0,
        page_size: 1,
      });
      return cs.data.at(0);
    },
    fetchMoreChnots: async () => {
      if (get().isFetchingNextPage) {
        return;
      }

      set((state) => {
        return {
          ...state,
          isFetchingNextPage: true,
        };
      });

      const read = get();
      const cs: ChnotQueryRsp = await request.post(`api/v1/chnot-query`, {
        start_index: read.chnotMap.size,
        page_size: read.pageSize,
        query: read.query,
      });

      set((state) => {
        const arr = state.chnotMap;

        for (const c of cs.data) {
          state.chnotMap.set(c.meta.id, c);
        }

        return {
          ...state,
          chnots: arr,
          startIndex: cs.start_index + cs.data.length,
          hasNextPage: cs.data.length >= read.pageSize,
          isFetchingNextPage: false,
        };
      });
    },
    changeKeyword: async (query?: string) => {
      set((state) => {
        return { ...state, query: query, startIndex: 0 };
      });

      await get().refreshChnots();
    },
    refreshChnots: async () => {
      set((state) => {
        return { ...state, startIndex: 0, chnotMap: new Map() };
      });

      await get().fetchMoreChnots();
    },
    deleteChnot: async (req: ChnotDeletionReq) => {
      return request.post(`api/v1/chnot/deletion`, req);
    },
    overwriteChnot: async (
      req: ChnotOverwriteReq,
      overwriteCache: boolean
    ): Promise<ChnotOverwriteRsp> => {
      return request
        .put<ChnotOverwriteRsp>(`api/v1/chnot`, req)
        .then((value: ChnotOverwriteRsp) => {
          if (overwriteCache) {
            let chnot = value.chnot;
            set((state) => {
              let cm = state.chnotMap;
              if (cm.has(chnot.meta.id)) {
                cm.set(chnot.meta.id, chnot);
              } else {
                cm = insertMapAtIndex(0, chnot.meta.id, chnot, cm);
              }
              return { ...state, chnotMap: cm };
            });
          }
          return value;
        });
    },
    updateChnot: async (req: ChnotUpdateReq) => {
      return request.post(`api/v1/chnot/update`, req);
    },
    setCurrentChnot: (chnot?: Chnot) => {
      set((state) => {
        return { ...state, currentChnot: chnot };
      });
    },
    getCurrentChnot: () => {
      return get().currentChnot;
    },
  }))
);
