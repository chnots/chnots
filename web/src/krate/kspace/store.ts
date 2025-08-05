import { KSpace } from "@/krate/kspace/po";
import { create, useStore } from "zustand";
import { combine } from "zustand/middleware";
import { allKSpaces } from "./service";
import { useShallow } from "zustand/react/shallow";

interface KSpaceState {
  kspaceMapByName: Map<string, KSpace>;
  currentKSpace: string;
  mkspaces: string[];
  currentKSpaceObj: () => KSpace | undefined;
  selectKSpace: (name: string) => void;
  toggleMKSpace: (mkspace: string) => void;
  refreshKSpaces: () => void;
  allKSpaces: () => KSpace[];
}

const getDefaultState = (): KSpaceState => {
  return {
    kspaceMapByName: new Map(),
    currentKSpace: (() => {
      const searchParams = new URLSearchParams(window.location.search.slice(1));
      console.log("search params:", location.hash);
      return searchParams.get("ns") ?? "public";
    })(),
    mkspaces: [],
    currentKSpaceObj: () => {
      return undefined;
    },
    selectKSpace: () => {},
    toggleMKSpace: () => {},
    refreshKSpaces: () => {},
    allKSpaces: () => {
      return [];
    },
  };
};

export const kspaceStore = create(
  combine(getDefaultState(), (set, get) => ({
    refreshKSpaces: async () => {
      const rsp = await allKSpaces({});
      const kspaceMap = new Map();
      rsp.kspaces.forEach((k) => {
        kspaceMap.set(k.name, k);
      });

      set({ kspaceMapByName: kspaceMap });
      console.log("refresh kspace, ", kspaceMap);
    },
    selectKSpace: async (kspace: string) => {
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
    getKSpace: (name: string) => {
      return get().kspaceMapByName.get(name);
    },
    allKSpaces: () => {
      return [...get().kspaceMapByName.values()];
    },
    currentKSpaceObj: () => {
      const read = get();
      return read.kspaceMapByName.get(read.currentKSpace);
    },
  })),
);

export function useKSpaceStore<T>(selector: (state: KSpaceState) => T) {
  return useStore(
    kspaceStore!,
    useShallow((store) => {
      return selector(store);
    }),
  );
}
