import { insertMapAtIndex } from "@/lib/map-utils";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { ChnotThread, ChnotThreadQueryRsp, ChnotTagSearchType } from "./dto";
import { chnotThreadQuery } from "./service";
import { TID } from "@/lib/id_util";
import { DbCache } from "@/common/store";
import { kspaceStore } from "../kspace/store";
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
    threadMapByThreadId: newChnotMap(),
    query: undefined,
    isFetchingNextPage: false,
    tags: undefined,
  };
};

interface State {
  refreshChnots(): unknown;
  fetchMoreChnots(): unknown;

  /**
   * ChnotThread Map by ChnotThread Meta Id
   */
  threadMapByThreadId: DbCache<ChnotThread>;

  /**
   * Current ChnotThread Meta Id
   */
  curThreadOtid?: TID;

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
    fetchMoreChnotThreads: async () => {
      if (get().isFetchingNextPage) {
        return;
      }

      set((state) => {
        return {
          ...state,
          isFetchingNextPage: true,
        };
      });

      const {
        threadMapByThreadId: chnotMapByMetaId,
        query,
        tags,
        kinds,
      } = get();
      const cs: ChnotThreadQueryRsp = await chnotThreadQuery({
        start_index: chnotMapByMetaId.dbNextStartIndex,
        page_size: chnotMapByMetaId.dbPageSize,
        query: query,
        tags,
        kinds: kinds ?? [],
      });

      set((state) => {
        const cmm = state.threadMapByThreadId;
        const cm = cmm.dbCache;

        for (const c of cs.data) {
          cm.set(c.meta.otid, c);
        }

        return {
          ...state,
          threadMapByThreadId: {
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
    refreshChnotThreads: async () => {
      set((state) => {
        return { ...state, threadMapByThreadId: newChnotMap() };
      });

      await get().fetchMoreChnots();
    },
    overwriteChnotCache: (chnot: ChnotThread) => {
      set((state) => {
        const cmm = state.threadMapByThreadId;
        let dbCache = cmm.dbCache;
        if (dbCache.has(chnot.meta.otid)) {
          dbCache.set(chnot.meta.otid, chnot);
        } else {
          dbCache = insertMapAtIndex(0, chnot.meta.otid, chnot, dbCache);
        }
        cmm.dbCache = dbCache;
        return { ...state, threadMapByThreadId: cmm };
      });
    },
    setCurrentThreadOtid: (chnotMetaId?: TID) => {
      set((state) => {
        return { ...state, curThreadOtid: chnotMetaId };
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
    getCurrentThread: () => {
      const read = get();
      return read.curThreadOtid
        ? read.threadMapByThreadId.dbCache.get(read.curThreadOtid)
        : undefined;
    },
    validateChnotCache: (toRemoves: TID[]) => {
      const cmm = get().threadMapByThreadId;
      const dbCacheMap = cmm.dbCache;

      const toRemove2 = Array.from(
        [...dbCacheMap.values()]
          .filter((e) => {
            const result =
              e.meta.kspace == kspaceStore.getState().currentKSpace;
            return !result;
          })
          .map((e) => {
            return e.meta.otid;
          }),
      );

      for (const key of toRemove2) {
        if (key) {
          dbCacheMap.delete(key);
        }
      }

      for (const key of toRemoves) {
        dbCacheMap.delete(key);
      }

      set((prev) => {
        return {
          ...prev,
          threadMapByThreadId: cmm,
          curThreadOtid:
            prev.curThreadOtid && dbCacheMap.has(prev.curThreadOtid)
              ? prev.curThreadOtid
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
  })),
);
