import { insertMapAtIndex } from "@/utils/map-utils";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { useKSpaceStore } from "../kspace";
import {
  Chnot,
  ChnotQueryRsp,
  ChnotOverwriteReq,
  ChnotOverwriteRsp,
  ListViewType,
} from "./dto";
import { chnotOverwrite, chnotQuery } from "./service";
import { DbCache } from "../common";

const newChnotMap = () => {
  return {
    dbNextStartIndex: 0,
    dbPageSize: 20,
    hasNextPage: true,
    dbCache: new Map(),
  };
};

const getDefaultState = (): State => {
  return {
    fetchMoreChnots: () => {},
    refreshChnots: () => {},
    chnotMapByMetaId: newChnotMap(),
    query: undefined,
    isFetchingNextPage: false,
    listViewType: { kind: "timeline" },
  };
};

interface State {
  refreshChnots(): unknown;
  fetchMoreChnots(): unknown;

  /**
   * Chnot Map by Chnot Meta Id
   */
  chnotMapByMetaId: DbCache<Chnot>;

  /**
   * Current Chnot Meta Id
   */
  curMetaId?: string;

  /**
   * Current Query Input
   */
  query?: string;
  listViewType: ListViewType;
  isFetchingNextPage: boolean;
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

      const { chnotMapByMetaId, query, listViewType } = get();
      const cs: ChnotQueryRsp = await chnotQuery({
        start_index: chnotMapByMetaId.dbNextStartIndex,
        page_size: chnotMapByMetaId.dbPageSize,
        query: query,
        view_type: listViewType,
      });

      set((state) => {
        const cmm = state.chnotMapByMetaId;
        const cm = cmm.dbCache;

        for (const c of cs.data) {
          cm.set(c.meta.id, c);
        }

        return {
          ...state,
          chnotMapByMetaId: {
            ...cmm,
            dbNextStartIndex: cs.next_start,
            hasNextPage: cs.has_next,
          },
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
        return { ...state, chnotMapByMetaId: newChnotMap() };
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
            const cmm = state.chnotMapByMetaId;
            let cm = cmm.dbCache;
            if (cm.has(chnot.meta.id)) {
              cm.set(chnot.meta.id, chnot);
            } else {
              cm = insertMapAtIndex(0, chnot.meta.id, chnot, cm);
            }
            cmm.dbCache = cm;
            return { ...state, chnotMapByMetaId: cmm };
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
        ? read.chnotMapByMetaId.dbCache.get(read.curMetaId)
        : undefined;
    },
    validateChnotCache: (toRemoves: string[]) => {
      const cmm = get().chnotMapByMetaId;
      const map = cmm.dbCache;

      const toRemove2 = Array.from(
        map
          .values()
          .filter((e) => {
            const result =
              e.meta.kspace == useKSpaceStore.getState().currentKSpace.name;
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
          chnotMapByMetaId: cmm,
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
          ...prev,
          listViewType: newType,
        };
      });
    },
  }))
);
