import { insertMapAtIndex } from "@/lib/map-utils";
import { create, useStore } from "zustand";
import { combine } from "zustand/middleware";
import {
  ChnotThreadListRspData,
  ChnotThreadListRsp,
  MdwtTagSearchType,
} from "./dto";
import { TID } from "@/lib/id_util";
import { DbCache } from "@/common/store";
import { kspaceStore } from "../kspace/store";
import { ChnotKind, ChnotMeta } from "./po";
import { chnotThreadList } from "./service";
import { useShallow } from "zustand/react/shallow";

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
    fetchMoreChnotThreads: () => {},
    refreshChnots: () => {},
    threadMapByThreadId: newChnotMap(),
    cachedChnotMap: new Map(),
    query: undefined,
    isFetchingNextPage: false,
    tags: undefined,
    changeKeyword: function (query?: string): void {},

    overwriteChnotCache: function (chnot: ChnotThreadListRspData): void {},
    setCurrentThreadOtid: function (chnotMetaId?: TID): void {},
    setTagKeyword: function (tagKeyword?: string): void {},
    setChnotKinds: function (): void {},
    getCurrentThread: function () {
      return undefined;
    },
    setChnotMeta: function (meta: ChnotMeta): void {},
    getChnotMeta: function (otid: TID): ChnotMeta | undefined {
      return undefined;
    },
    validateChnotCache: function (toRemoves: TID[]): void {},
    setTagsInset: function (newType: string[]): void {},
    setTags: function (newTags?: MdwtTagSearchType): void {},
  };
};

interface State {
  refreshChnots(): unknown;
  fetchMoreChnotThreads(): unknown;
  changeKeyword(query?: string): void;
  overwriteChnotCache(chnot: ChnotThreadListRspData): void;
  setCurrentThreadOtid(chnotMetaId?: TID): void;
  setTagKeyword(tagKeyword?: string): void;
  setChnotKinds(changeKinds: (kinds?: ChnotKind[]) => ChnotKind[]): void;
  getCurrentThread(): ChnotThreadListRspData | undefined;
  setChnotMeta(meta: ChnotMeta): void;
  getChnotMeta(otid: TID): ChnotMeta | undefined;
  validateChnotCache(toRemoves: TID[]): void;
  setTagsInset(newType: string[]): void;
  setTags(newTags?: MdwtTagSearchType): void;

  /**
   * ChnotThread Map by ChnotThread Meta Id
   */
  threadMapByThreadId: DbCache<ChnotThreadListRspData>;

  /**
   * Current ChnotThread Meta Id
   */
  curThreadOtid?: TID;

  /**
   * ChnotMeta Cache
   */
  cachedChnotMap: Map<TID, ChnotMeta>;

  /**
   * Current Query Input
   */
  query?: string;
  tags?: MdwtTagSearchType;
  kinds?: ChnotKind[];
  isFetchingNextPage: boolean;
}

export const chnotStore = create(
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
      const cs: ChnotThreadListRsp = await chnotThreadList({
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

      await get().fetchMoreChnotThreads();
    },
    overwriteChnotCache: (chnot: ChnotThreadListRspData) => {
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
    setChnotMeta: (meta: ChnotMeta) => {
      set((state) => {
        const newMap = state.cachedChnotMap.set(meta.otid, meta);
        return { ...state, cachedChnotMap: newMap };
      });
    },
    getChnotMeta: (otid: TID) => {
      return get().cachedChnotMap.get(otid);
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
    setTags: (newTags?: MdwtTagSearchType) => {
      set((prev) => {
        return {
          ...prev,
          tags: newTags,
        };
      });
    },
  })),
);

export function useChnotStore<T>(selector: (state: State) => T) {
  return useStore(
    chnotStore!,
    useShallow((store) => {
      return selector(store);
    }),
  );
}
