import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  onSearch: boolean;
}

const getDefaultState = (): State => {
  return {
    onSearch: false,
  };
};

export const useCommonStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    toggleNaviSearch: () => {
      const current = get().onSearch;
      set({
        onSearch: !current,
      });
    },
    getNaviSearch: () => {
      return get().onSearch;
    },
  }))
);
