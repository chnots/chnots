import type { ReactNode } from "react";
import { create, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import type { DbCache } from "@/common/store";
import type { PageRsp } from "@/common/types";
import type { TID } from "@/lib/id_util";
import { insertMapAtIndex } from "@/lib/map-utils";
import { kspaceStore } from "../kspace/store";
import type { ChnotSearchRspData } from "./dto";
import type { MdwtTagSearchType } from "../mdwt/dto";
import type { ChnotKind, ChnotMeta } from "./po";
import { chnotSearch } from "./service";

const emptyCacheMap = <ChnotSearchRspData>(): DbCache<ChnotSearchRspData> => {
  return {
    nextStartIn: 0,
    pageSize: 20,
    hasMore: true,
    cache: new Map(),
  };
};

export interface StateChnotLike {
  title?: string;
  meta: {
    otid: TID;
    kspace: string;
    pin_tid?: TID;
  };
}

export type HeaderAction = {
  key: string;
  actions: ReactNode;
};

interface ChnotStore {
  /**
   * Current Query Input
   */
  searchStr?: string;
  tags?: MdwtTagSearchType;
  kinds?: ChnotKind[];

  changeSearchStr(query?: string): void;
  setChnotKinds(changeKinds: (kinds?: ChnotKind[]) => ChnotKind[]): void;
  setTagsInset(newType: string[] | undefined): void;
  curOtid?: TID;

  /**
   * Header Actions - allow child components to register actions in header
   */
  headerActions: HeaderAction[];
  registerHeaderActions: (key: string, actions: ReactNode) => void;
  unregisterHeaderActions: (key: string) => void;

  /**
   * Map by Otid
   */
  mapByOtid: DbCache<ChnotSearchRspData>;

  isFetchingNextPage: boolean;
  setChangeCompCurOtid(setter?: (otid?: TID) => void): void;

  changeCompCurOtid?: (otid?: TID) => void;

  getMeta(otid: TID): ChnotSearchRspData | undefined;
  overwritePart(otid: TID, chnot: Partial<ChnotSearchRspData>): void;
  setCurOtid(cutOtid?: TID): void;
  unvalidate(toRemoves: TID[]): void;

  clearCache(): Promise<void>;
  fetchMore(): Promise<void>;
}

export const cachedChnotMapByOtid: Map<TID, ChnotMeta> = new Map();

export const chnotHeadStore = create<ChnotStore>((set, get) => ({
  searchStr: undefined,
  tags: undefined,
  kinds: undefined,
  isFetchingNextPage: false,
  curOtid: undefined,
  headerActions: [],

  mapByOtid: emptyCacheMap(),

  changeCompCurOtid: undefined,

  registerHeaderActions: (key: string, actions: ReactNode) => {
    set((state) => {
      const existing = state.headerActions.find((a) => a.key === key);
      if (existing) {
        return {
          ...state,
          headerActions: state.headerActions.map((a) =>
            a.key === key ? { key, actions } : a,
          ),
        };
      }
      return {
        ...state,
        headerActions: [...state.headerActions, { key, actions }],
      };
    });
  },

  unregisterHeaderActions: (key: string) => {
    set((state) => ({
      ...state,
      headerActions: state.headerActions.filter((a) => a.key !== key),
    }));
  },

  changeSearchStr: async (query?: string) => {
    set((state) => ({ ...state, searchStr: query }));
  },

  setCurrOtid: (ChnotSearchRspDataId?: TID) => {
    set((state) => ({ ...state, curOtid: ChnotSearchRspDataId }));
  },

  setChnotKinds: (changeKinds: (kinds?: ChnotKind[]) => ChnotKind[]) => {
    set((state) => ({
      ...state,
      kinds: [...new Set(changeKinds(state.kinds))],
    }));
  },

  setTagsInset: (tags: string[] | undefined) => {
    set((prev) => ({
      ...prev,
      tags: tags ? { id: "Inset", data: tags } : undefined,
    }));
  },

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
    const pageRsp: PageRsp<ChnotSearchRspData> = await chnotSearch({
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

  overwrite: (chnot: ChnotSearchRspData) => {
    set((state) => {
      const cmm = state.mapByOtid;
      const dbCache = cmm.cache;
      if (dbCache.has(chnot.meta.otid)) {
        dbCache.set(chnot.meta.otid, chnot);
      } else {
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

  overwritePart: (otid: TID, chnot: Partial<ChnotSearchRspData>) => {
    set((state) => {
      const cmm = state.mapByOtid;
      const dbCache = cmm.cache;
      const saved = dbCache.get(otid);
      if (saved) {
        if (!chnot.title && chnot.meta) {
          dbCache.set(otid, { ...saved, meta: chnot.meta });
        } else if (!chnot.meta && chnot.title) {
          dbCache.set(otid, { ...saved, title: chnot.title });
        } else if (chnot.meta && chnot.title) {
          dbCache.set(otid, { title: chnot.title, meta: chnot.meta });
        }

        return {
          ...state,
          mapByOtid: {
            cache: dbCache,
            nextStartIn: cmm.nextStartIn,
            pageSize: cmm.pageSize,
            hasMore: cmm.hasMore,
          },
        };
      } else if (chnot.meta && chnot.title) {
        const cache = insertMapAtIndex(
          0,
          chnot.meta.otid,
          { meta: chnot.meta, title: chnot.title },
          dbCache,
        );
        return {
          ...state,
          mapByOtid: {
            cache: cache,
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
    const curKSpace = kspaceStore.getState().currentKSpace;

    const toRemove2 = Array.from(
      [...dbCacheMap.values()]
        .filter((e) => {
          const result = e.meta.kspace === curKSpace;
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
        prev.curOtid && dbCacheMap.has(prev.curOtid) ? prev.curOtid : undefined,
    }));
  },
}));

export function useChnotStore<S>(selector: (state: ChnotStore) => S) {
  return useStore(
    chnotHeadStore,
    useShallow((store) => {
      return selector(store);
    }),
  );
}

export function useRegisterHeaderActions(key: string, actions: ReactNode) {
  const registerHeaderActions = useChnotStore((s) => s.registerHeaderActions);
  const unregisterHeaderActions = useChnotStore(
    (s) => s.unregisterHeaderActions,
  );

  return { registerHeaderActions, unregisterHeaderActions };
}
