import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  onSearch: boolean;
  showSidebar: boolean;
}

const getDefaultState = (): State => {
  return {
    onSearch: false,
    showSidebar: true,
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
  }))
);
