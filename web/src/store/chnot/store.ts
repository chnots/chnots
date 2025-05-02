import { insertMapAtIndex } from "@/utils/map-utils";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { useNamespaceStore } from "../namespace";
import {
  Chnot,
  ChnotQueryRsp,
  ChnotOverwriteReq,
  ChnotOverwriteRsp,
  ListViewType,
} from "./dto";
import { chnotOverwrite, chnotQuery } from "./service";

const getDefaultState = (): State => {
  return {
    fetchMoreChnots: () => {},
    refreshChnots: () => {},
    chnotMapByMetaId: new Map(),
    pageSize: 20,
    query: undefined,
    isFetchingNextPage: false,
    hasNextPage: true,
    listViewType: { kind: "timeline" }
  };
};

interface State {
  refreshChnots(): unknown;
  fetchMoreChnots(): unknown;

  pageSize: number;

  /**
   * Current Query Input
   */
  query?: string;

  /**
   * Chnot Map by Chnot Meta Id
   */
  chnotMapByMetaId: Map<string, Chnot>;


  listViewType: ListViewType

  /**
   * Current Chnot Meta Id
   */
  curMetaId?: string;

  isFetchingNextPage: boolean;
  hasNextPage: boolean;

}

export const useChnotStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
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
      const cs: ChnotQueryRsp = await chnotQuery({
        start_index: read.chnotMapByMetaId.size,
        page_size: read.pageSize,
        query: read.query,
        view_type: read.listViewType 
      });

      set((state) => {
        const arr = state.chnotMapByMetaId;

        for (const c of cs.data) {
          state.chnotMapByMetaId.set(c.meta.id, c);
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
        return { ...state, startIndex: 0, chnotMapByMetaId: new Map() };
      });

      await get().fetchMoreChnots();
    },
    overwriteChnot: async (
      req: ChnotOverwriteReq,
      overwriteCache: boolean
    ): Promise<ChnotOverwriteRsp> => {
      return chnotOverwrite(req).then((value: ChnotOverwriteRsp) => {
        if (overwriteCache) {
          const chnot = value.chnot;
          set((state) => {
            let cm = state.chnotMapByMetaId;
            if (cm.has(chnot.meta.id)) {
              cm.set(chnot.meta.id, chnot);
            } else {
              cm = insertMapAtIndex(0, chnot.meta.id, chnot, cm);
            }
            return { ...state, chnotMapByMetaId: cm };
          });
        }
        return value;
      });
    },
    setCurrentChnotMetaId: (chnotMetaId?: string) => {
      set((state) => {
        return { ...state, curMetaId: chnotMetaId };
      });
    },
    setTagKeyword: (tagKeyword?: string) => {
      set((state) => {
        return { ...state, tagPath: tagKeyword };
      });
    },
    getCurrentChnot: () => {
      const read = get();
      return read.curMetaId
        ? read.chnotMapByMetaId.get(read.curMetaId)
        : undefined;
    },
    validateChnotCache: (toRemoves: string[]) => {
      const map = get().chnotMapByMetaId;

      const toRemove2 = Array.from(
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

      for (const key of toRemove2) {
        map.delete(key);
      }

      for (const key of toRemoves) {
        map.delete(key);
      }

      set((prev) => {
        return {
          ...prev,
          chnotMapByMetaId: map,
          curMetaId:
            prev.curMetaId && map.has(prev.curMetaId)
              ? prev.curMetaId
              : undefined,
        };
      });
    },
    setListViewType: (newType: ListViewType) => {
      set((prev) => {
        return {
          ...prev, listViewType: newType
        }
      })
    }
  }))
);
