import { Namespace } from "@/model";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  namespaceMapByName: Map<string, Namespace>;
  current: Namespace;
}

const namespaces = [
  {
    name: "public",
    managers: ["work", "private"],
  },
  {
    name: "work",
    managers: ["private"],
  },
  {
    name: "private",
    managers: [],
  },
];

const getDefaultState = (): State => {
  return {
    namespaceMapByName: namespaces.reduce((acc, v) => {
      acc.set(v.name, v);
      return acc;
    }, new Map()),
    current: namespaces[0],
  };
};

export const useNamespaceStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    fetchNamespaces: async () => {
      const namespaceMap = get().namespaceMapByName;
      for (const namespace of namespaces) {
        namespaceMap.set(namespace.name, namespace);
      }
      set({ namespaceMapByName: namespaceMap });
      return namespaces;
    },
    changeNamespace: async (namespace: string) => {
      const newnamespace = get().namespaceMapByName.get(namespace);
      set({
        current: newnamespace,
      });
    },
    getCurrent: () => {
      return get().current;
    },
    getNamespace: (namespaceName: string) => {
      return get().namespaceMapByName.get(namespaceName);
    },
    namespaces: () => {
      return Object.values(get().namespaceMapByName);
    },
  }))
);
