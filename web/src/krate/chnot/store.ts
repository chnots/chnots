import { insertMapAtIndex } from "@/lib/map-utils";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { Chnot, ChnotQueryRsp, ChnotTagSearchType } from "./dto";
import { chnotQuery } from "./service";
import { TID } from "@/lib/id_util";
import { DbCache } from "@/common/store";
import { useKSpaceStore } from "../kspace/store";
import { ChnotKind } from "./po";

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
    tags: undefined,
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
  curMetaId?: TID;

  /**
   * Current Query Input
   */
  query?: string;
  tags?: ChnotTagSearchType;
  kinds?: ChnotKind[];
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

      const { chnotMapByMetaId, query, tags, kinds } = get();
      const cs: ChnotQueryRsp = await chnotQuery({
        start_index: chnotMapByMetaId.dbNextStartIndex,
        page_size: chnotMapByMetaId.dbPageSize,
        query: query,
        tags,
        kinds: kinds ?? [],
      });

      set((state) => {
        const cmm = state.chnotMapByMetaId;
        const cm = cmm.dbCache;

        for (const c of cs.data) {
          cm.set(c.meta.tid, c);
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
    overwriteChnotCache: (chnot: Chnot) => {
      set((state) => {
        const cmm = state.chnotMapByMetaId;
        let cm = cmm.dbCache;
        if (cm.has(chnot.meta.tid)) {
          cm.set(chnot.meta.tid, chnot);
        } else {
          cm = insertMapAtIndex(0, chnot.meta.tid, chnot, cm);
        }
        cmm.dbCache = cm;
        return { ...state, chnotMapByMetaId: cmm };
      });
    },
    setCurrentChnotMetaId: (chnotMetaId?: TID) => {
      set((state) => {
        return { ...state, curMetaId: chnotMetaId };
      });
    },
    setTagKeyword: (tagKeyword?: string) => {
      set((state) => {
        return { ...state, tagPath: tagKeyword };
      });
    },
    setChnotKinds: (changeKinds: (kinds?: ChnotKind[]) => ChnotKind[]) => {
      set((state) => {
        return { ...state, kinds: [...new Set(changeKinds(state.kinds))] };
      });
    },
    getCurrentChnot: () => {
      const read = get();
      return read.curMetaId
        ? read.chnotMapByMetaId.dbCache.get(read.curMetaId)
        : undefined;
    },
    validateChnotCache: (toRemoves: TID[]) => {
      const cmm = get().chnotMapByMetaId;
      const map = cmm.dbCache;

      const toRemove2 = Array.from(
        map
          .values()
          .filter((e) => {
            const result =
              e.meta.kspace == useKSpaceStore.getState().currentKSpace;
            return !result;
          })
          .map((e) => {
            return e.record.tid;
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
    setTagsInset: (newType: string[]) => {
      set((prev) => {
        return {
          ...prev,
          tags: { Inset: newType },
        };
      });
    },
    setTags: (newTags?: ChnotTagSearchType) => {
      set((prev) => {
        return {
          ...prev,
          tags: newTags,
        };
      });
    },
  }))
);
