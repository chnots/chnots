import { Namespace } from "@/model";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  namespaceMapByName: Map<string, Namespace>;
  currentNamespace: Namespace;
}

const namespaces = new Map<string, Namespace>([
  [
    "public",
    {
      name: "public",
      managers: ["work", "private"],
    },
  ],
  [
    "work",
    {
      name: "work",
      managers: ["private"],
    },
  ],
  [
    "private",
    {
      name: "private",
      managers: [],
    },
  ],
]);

const getDefaultState = (): State => {
  return {
    namespaceMapByName: namespaces,
    currentNamespace: (() => {
      const searchParams = new URLSearchParams(window.location.search.slice(1));
      console.log("search params:", location.hash);
      const ns = searchParams.get("ns");
      if (ns === null || !namespaces.has(ns)) {
        return namespaces.get("public")!;
      }

      return namespaces.get(ns)!;
    })(),
  };
};

export const useNamespaceStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    fetchNamespaces: async () => {
      set({ namespaceMapByName: namespaces });
      return namespaces;
    },
    changeNamespace: async (namespace: string) => {
      const newnamespace = get().namespaceMapByName.get(namespace);
      const searchParams = new URLSearchParams(window.location.search.slice(1));
      searchParams.set("ns", namespace);
      set({
        currentNamespace: newnamespace,
      });
    },
    getNamespace: (namespaceName: string) => {
      return get().namespaceMapByName.get(namespaceName);
    },
    namespaces: () => {
      return Object.values(get().namespaceMapByName);
    },
  }))
);
