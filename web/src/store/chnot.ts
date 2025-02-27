import { insertMapAtIndex } from "@/utils/map-utils";
import request from "@/utils/request";
import { validate } from "uuid";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { useNamespaceStore } from "./namespace";

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

export interface ChnotUpdateReq {
  meta_id: string;

  namespace?: string;

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
  currentChnotIndex?: string;

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
      return request.post(`api/v1/chnot-deletion`, req);
    },
    overwriteChnot: async (
      req: ChnotOverwriteReq,
      overwriteCache: boolean
    ): Promise<ChnotOverwriteRsp> => {
      return request
        .put<ChnotOverwriteRsp>(`api/v1/chnot`, req)
        .then((value: ChnotOverwriteRsp) => {
          if (overwriteCache) {
            const chnot = value.chnot;
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
      return request.post(`api/v1/chnot-update`, req);
    },
    setCurrentChnot: (chnot?: Chnot) => {
      set((state) => {
        return { ...state, currentChnotIndex: chnot?.meta.id };
      });
    },
    getCurrentChnot: () => {
      const read = get();
      return read.currentChnotIndex
        ? read.chnotMap.get(read.currentChnotIndex)
        : undefined;
    },
    validateChnotCache: (toRemoves: string[]) => {
      const map = get().chnotMap;

      const toRemove = Array.from(
        map
          .values()
          .filter((e) => {
            const result =
              e.meta.namespace ==
              useNamespaceStore.getState().currentNamespace.name;
            return !result;
          })
          .map((e) => {
            return e.record.id;
          })
      );

      for (const key of toRemove) {
        map.delete(key);
      }

      for (const key of toRemoves) {
        map.delete(key);
      }

      console.log("===", toRemove);

      set((prev) => {
        return {
          ...prev,
          chnotMap: map,
          currentChnotIndex:
            prev.currentChnotIndex && map.has(prev.currentChnotIndex)
              ? prev.currentChnotIndex
              : undefined,
        };
      });
    },
  }))
);
