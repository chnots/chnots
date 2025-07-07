import { KSpace } from "@/krate/kspace/po";
import { genTID } from "@/lib/id_util";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  kspaceMapByName: Map<string, KSpace>;
  currentKSpace: string;
  mkspaces: string[];
}

const kspaces = new Map<string, KSpace>([
  [
    "public",
    {
      name: "public",
      managers: ["work", "private"],
      color: "#282828",
      tid: genTID(),
    },
  ],
  [
    "work",
    {
      name: "work",
      managers: ["private"],
      color: "#0000aa",
      tid: genTID(),
    },
  ],
  [
    "private",
    {
      name: "private",
      managers: [],
      color: "#aa0000",
      tid: genTID(),
    },
  ],
]);

const getDefaultState = (): State => {
  return {
    kspaceMapByName: kspaces,
    currentKSpace: (() => {
      const searchParams = new URLSearchParams(window.location.search.slice(1));
      console.log("search params:", location.hash);
      return searchParams.get("ns") ?? "public";
    })(),
    mkspaces: [],
  };
};

export const useKSpaceStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    fetchKSpaces: async () => {
      set({ kspaceMapByName: kspaces });
      return kspaces;
    },
    setKSpace: async (kspace: string) => {
      set((prev) => {
        return {
          ...prev,
          mkspaces: [],
          currentKSpace: kspace,
        };
      });
    },
    setMKSpaces: (mkspaces: string[]) => {
      set((prev) => {
        return {
          ...prev,
          mkspaces: [...new Set(mkspaces)].filter(
            (e) => e != prev.currentKSpace,
          ),
        };
      });
    },
    removeMKSpace: (mkspace: string) => {
      set((prev) => {
        return {
          ...prev,
          mkspaces: prev.mkspaces.filter((e) => e != mkspace),
        };
      });
    },
    toggleMKSpace: (mkspace: string) => {
      set((prev) => {
        const mkspaces = prev.mkspaces.includes(mkspace)
          ? prev.mkspaces.filter((e) => e != mkspace)
          : [...new Set([...prev.mkspaces, mkspace])];
        return {
          ...prev,
          mkspaces,
        };
      });
    },
    getKSpace: (kspaceName: string) => {
      return get().kspaceMapByName.get(kspaceName);
    },
    allKSpaces: () => {
      return [...get().kspaceMapByName.values()];
    },
    getCurrentKSpace: () => {
      const read = get();
      return read.kspaceMapByName.get(read.currentKSpace);
    },
  })),
);
