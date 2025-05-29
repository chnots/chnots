import { KSpace } from "@/krate/kspace/store/po";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  kspaceMapByName: Map<string, KSpace>;
  currentKSpace: KSpace;
}

const kspaces = new Map<string, KSpace>([
  [
    "public",
    {
      name: "public",
      managers: ["work", "private"],
      color: "#282828",
    },
  ],
  [
    "work",
    {
      name: "work",
      managers: ["private"],
      color: "#0000aa",
    },
  ],
  [
    "private",
    {
      name: "private",
      managers: [],
      color: "#aa0000",
    },
  ],
]);

const getDefaultState = (): State => {
  return {
    kspaceMapByName: kspaces,
    currentKSpace: (() => {
      const searchParams = new URLSearchParams(window.location.search.slice(1));
      console.log("search params:", location.hash);
      const ns = searchParams.get("ns");
      if (ns === null || !kspaces.has(ns)) {
        return kspaces.get("public")!;
      }

      return kspaces.get(ns)!;
    })(),
  };
};

export const useKSpaceStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    fetchKSpaces: async () => {
      set({ kspaceMapByName: kspaces });
      return kspaces;
    },
    changeKSpace: async (kspace: string) => {
      console.log("change ns:", kspace);
      const newkspace = get().kspaceMapByName.get(kspace);
      set({
        currentKSpace: newkspace,
      });
    },
    getKSpace: (kspaceName: string) => {
      return get().kspaceMapByName.get(kspaceName);
    },
    kspaces: () => {
      return [...get().kspaceMapByName.values()];
    },
  }))
);
