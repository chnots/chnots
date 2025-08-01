import { TID } from "@/lib/id_util";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  onSearch: boolean;
  showSidebar: boolean;
  globalLog: string;
}

const getDefaultState = (): State => {
  return {
    onSearch: false,
    showSidebar: true,
    globalLog: "",
  };
};

export const useCommonStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    toggleNaviSearch: () => {
      set((prev) => {
        return { ...prev, showSidebar: !prev.onSearch };
      });
    },
    toggleSidebar: () => {
      set((prev) => {
        return { ...prev, showSidebar: !prev.showSidebar };
      });
    },
    getNaviSearch: () => {
      return get().onSearch;
    },
    appendLog: (log: string) => {
      set((prev) => {
        return {
          ...prev,
          globalLog:
            prev.globalLog +
            "\n\n" +
            new Date().toLocaleTimeString() +
            ": " +
            log,
        };
      });
    },
  }))
);

export type DbCache<T> = {
  /**
   * for db result empty hole
   */
  dbNextStartIndex: number;
  dbPageSize: number;
  hasNextPage: boolean;
  dbCache: Map<TID, T>;
};

