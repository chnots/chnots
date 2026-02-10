import { create, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import type { DbCache } from "@/common/store";
import type { PageRsp } from "@/common/types";
import type { TID } from "@/lib/id_util";
import { insertMapAtIndex } from "@/lib/map-utils";
import { kspaceStore } from "../kspace/store";
import type {
  ChnotSearchReq,
  ChnotSearchRspSingle,
  ChnotSearchRspThread,
  MdwtTagSearchType,
} from "./dto";
import type { ChnotKind, ChnotMeta } from "./po";
import { chnotSingleSearch, chnotThreadSearch } from "./service";

const emptyCacheMap = <T>(): DbCache<T> => {
  return {
    nextStartIn: 0,
    pageSize: 20,
    hasMore: true,
    cache: new Map(),
  };
};

export enum ChnotViewType {
  Single,
  Thread,
}

export interface StateChnotLike {
  title?: string;
  meta: {
    otid: TID;
    kspace: string;
    pin_tid?: TID;
  };
}

interface HeadState {
  /**
   * Current Query Input
   */
  searchStr?: string;
  tags?: MdwtTagSearchType;
  kinds?: ChnotKind[];

  changeSearchStr(query?: string): void;
  setChnotKinds(changeKinds: (kinds?: ChnotKind[]) => ChnotKind[]): void;
  setTagsInset(newType: string[] | undefined): void;
  setViewType(viewType: ChnotViewType): void;
}

export const chnotHeadStore = create<HeadState>((set, _get) => ({
  searchStr: undefined,
  tags: undefined,
  kinds: undefined,
  isFetchingNextPage: false,

  changeSearchStr: async (query?: string) => {
    set((state) => ({ ...state, searchStr: query }));
  },

  setCurrOtid: (chnotMetaId?: TID) => {
    set((state) => ({ ...state, curOtid: chnotMetaId }));
  },

  setChnotKinds: (changeKinds: (kinds?: ChnotKind[]) => ChnotKind[]) => {
    set((state) => ({
      ...state,
      kinds: [...new Set(changeKinds(state.kinds))],
    }));
  },

  setTagsInset: (newType: string[] | undefined) => {
    set((prev) => ({
      ...prev,
      tags: newType ? { Inset: newType } : undefined,
    }));
  },

  setViewType: (viewType: ChnotViewType) => {
    set((prev) => ({ ...prev, viewType }));
  },
}));

export const cachedChnotMapByOtid: Map<TID, ChnotMeta> = new Map();

interface State<T extends StateChnotLike> {
  curOtid?: TID;

  /**
   * Map by Otid
   */
  mapByOtid: DbCache<T>;

  isFetchingNextPage: boolean;
  setChangeCompCurOtid(setter?: (otid?: TID) => void): void;

  changeCompCurOtid?: (otid?: TID) => void;

  getMeta(otid: TID): T | undefined;
  overwrite(chnot: T): void;
  overwritePart(otid: TID, chnot: Partial<T>): void;
  setCurOtid(cutOtid?: TID): void;
  unvalidate(toRemoves: TID[]): void;

  clearCache(): Promise<void>;
  fetchMore(): Promise<void>;
}

export const createChnotStore = <T extends StateChnotLike>(
  searchApi: (req: ChnotSearchReq) => Promise<PageRsp<T>>,
) =>
  create<State<T>>((set, get) => ({
    searchStr: undefined,
    tags: undefined,
    kinds: undefined,
    curOtid: undefined,
    isFetchingNextPage: false,

    mapByOtid: emptyCacheMap(),

    changeCompCurOtid: undefined,

    fetchMore: async () => {
      const { isFetchingNextPage, mapByOtid } = get();
      if (isFetchingNextPage || !mapByOtid.hasMore) {
        return;
      }

      set((state) => ({
        ...state,
        isFetchingNextPage: true,
      }));

      const { searchStr, tags, kinds } = chnotHeadStore.getState();
      const pageRsp: PageRsp<T> = await searchApi({
        start_index: mapByOtid.nextStartIn,
        page_size: mapByOtid.pageSize,
        query: searchStr,
        tags,
        kinds: kinds ?? [],
      });

      set((state) => {
        const cm = state.mapByOtid.cache;

        for (const chnot of pageRsp.data) {
          cm.set(chnot.meta.otid, chnot);
        }

        return {
          ...state,
          mapByOtid: {
            pageSize: state.mapByOtid.pageSize,
            nextStartIn: pageRsp.next_start,
            hasMore: pageRsp.has_next,
            cache: cm,
          },
          isFetchingNextPage: false,
        };
      });
    },

    overwrite: (chnot: T) => {
      set((state) => {
        const cmm = state.mapByOtid;
        let dbCache = cmm.cache;
        if (dbCache.has(chnot.meta.otid)) {
          dbCache.set(chnot.meta.otid, chnot);
        } else {
          dbCache = insertMapAtIndex(0, chnot.meta.otid, chnot, dbCache);
        }
        return {
          ...state,
          mapByOtid: {
            cache: dbCache,
            nextStartIn: cmm.nextStartIn + 1,
            pageSize: cmm.pageSize,
            hasMore: cmm.hasMore,
          },
        };
      });
    },

    overwritePart: (otid: TID, chnot: Partial<T>) => {
      set((state) => {
        const cmm = state.mapByOtid;
        const dbCache = cmm.cache;
        const meta = dbCache.get(otid);
        if (meta) {
          dbCache.set(otid, { ...meta, ...chnot });
          return {
            ...state,
            mapByOtid: {
              cache: dbCache,
              nextStartIn: cmm.nextStartIn + 1,
              pageSize: cmm.pageSize,
              hasMore: cmm.hasMore,
            },
          };
        } else {
          return state;
        }
      });
    },

    clearCache: async () => {
      set((state) => ({ ...state, mapByOtid: emptyCacheMap() }));
    },

    setCurOtid: (curOtid?: TID) => {
      set((state) => ({ ...state, curOtid }));
    },

    setChangeCompCurOtid: (setter?: (otid?: TID) => void) => {
      set((state) => ({ ...state, changeCompCurOtid: setter }));
    },

    getMeta: (otid: TID) => {
      return get().mapByOtid.cache.get(otid);
    },

    unvalidate: (toRemoves: TID[]) => {
      const cmm = get().mapByOtid;
      const dbCacheMap = cmm.cache;

      const toRemove2 = Array.from(
        [...dbCacheMap.values()]
          .filter((e) => {
            const result =
              e.meta.kspace === kspaceStore.getState().currentKSpace;
            return !result;
          })
          .map((e) => e.meta.otid),
      );

      for (const key of toRemove2) {
        if (key) {
          dbCacheMap.delete(key);
        }
      }

      for (const key of toRemoves) {
        dbCacheMap.delete(key);
      }

      set((prev) => ({
        ...prev,
        mapByOtid: cmm,
        curOtid:
          prev.curOtid && dbCacheMap.has(prev.curOtid)
            ? prev.curOtid
            : undefined,
      }));
    },

    setTagsInset: (newType: string[] | undefined) => {
      set((prev) => ({
        ...prev,
        tags: newType ? { Inset: newType } : undefined,
      }));
    },

    setViewType: (viewType: ChnotViewType) => {
      set((prev) => ({ ...prev, viewType }));
    },
  }));

export function useChnotHeadStore<S>(selector: (state: HeadState) => S) {
  return useStore(
    chnotHeadStore,
    useShallow((store) => {
      return selector(store);
    }),
  );
}

const chnotSingleStore = createChnotStore(chnotSingleSearch);
export function useChnotSingleStore<S>(
  selector: (state: State<ChnotSearchRspSingle>) => S,
) {
  return useStore(
    chnotSingleStore,
    useShallow((store) => {
      return selector(store);
    }),
  );
}

const chnotThreadStore = createChnotStore(chnotThreadSearch);
export function useChnotThreadStore<S>(
  selector: (state: State<ChnotSearchRspThread>) => S,
) {
  return useStore(
    chnotThreadStore,
    useShallow((store) => {
      return selector(store);
    }),
  );
}
