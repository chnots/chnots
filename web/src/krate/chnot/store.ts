import { insertMapAtIndex } from "@/lib/map-utils";
import { create, useStore } from "zustand";
import { combine } from "zustand/middleware";
import { ChnotSearchRspThread, ChnotSearchRsp, MdwtTagSearchType } from "./dto";
import { TID } from "@/lib/id_util";
import { DbCache } from "@/common/store";
import { kspaceStore } from "../kspace/store";
import { ChnotKind, ChnotMeta } from "./po";
import { chnotThreadList } from "./service";
import { useShallow } from "zustand/react/shallow";

const cacheMap = () => {
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
    refreshChnotThreads: () => {},
    threadMapByOtid: cacheMap(),
    cachedChnotMap: new Map(),
    query: undefined,
    isFetchingNextPage: false,
    tags: undefined,
    changeSearchStr: function (query?: string): void {},

    overwriteChnotCache: function (chnot: ChnotSearchRspThread): void {},
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
  refreshChnotThreads(): unknown;
  fetchMoreChnotThreads(): unknown;
  changeSearchStr(query?: string): void;
  overwriteChnotCache(chnot: ChnotSearchRspThread): void;
  setCurrentThreadOtid(chnotMetaId?: TID): void;
  setTagKeyword(tagKeyword?: string): void;
  setChnotKinds(changeKinds: (kinds?: ChnotKind[]) => ChnotKind[]): void;
  getCurrentThread(): ChnotSearchRspThread | undefined;
  setChnotMeta(meta: ChnotMeta): void;
  getChnotMeta(otid: TID): ChnotMeta | undefined;
  validateChnotCache(toRemoves: TID[]): void;
  setTagsInset(newType: string[]): void;
  setTags(newTags?: MdwtTagSearchType): void;

  /**
   * ChnotThread Map by ChnotThread Meta Id
   */
  threadMapByOtid: DbCache<ChnotSearchRspThread>;

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

      const { threadMapByOtid: chnotMapByMetaId, query, tags, kinds } = get();
      console.log("read", get());
      const cs: ChnotSearchRsp = await chnotThreadList({
        start_index: chnotMapByMetaId.dbNextStartIndex,
        page_size: chnotMapByMetaId.dbPageSize,
        query: query,
        tags,
        kinds: kinds ?? [],
      });

      set((state) => {
        const cmm = state.threadMapByOtid;
        const cm = cmm.dbCache;

        for (const c of cs.data) {
          cm.set(c.meta.otid, c);
        }

        return {
          ...state,
          threadMapByOtid: {
            ...cmm,
            dbNextStartIndex: cs.next_start,
            hasNextPage: cs.has_next,
          },
          isFetchingNextPage: false,
        };
      });
    },
    changeSearchStr: async (query?: string) => {
      console.log("change keyword", query);
      set((state) => {
        return { ...state, query: query, startIndex: 0 };
      });
    },
    refreshChnotThreads: async () => {
      set((state) => {
        return { ...state, threadMapByOtid: cacheMap() };
      });

      await get().fetchMoreChnotThreads();
    },
    overwriteChnotCache: (chnot: ChnotSearchRspThread) => {
      set((state) => {
        const cmm = state.threadMapByOtid;
        let dbCache = cmm.dbCache;
        if (dbCache.has(chnot.meta.otid)) {
          dbCache.set(chnot.meta.otid, chnot);
        } else {
          dbCache = insertMapAtIndex(0, chnot.meta.otid, chnot, dbCache);
        }
        cmm.dbCache = dbCache;
        return { ...state, threadMapByOtid: cmm };
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
        ? read.threadMapByOtid.dbCache.get(read.curThreadOtid)
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
      const cmm = get().threadMapByOtid;
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
          threadMapByOtid: cmm,
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
