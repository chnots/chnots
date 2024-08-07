import { Domain } from "@/model";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  domainMapByName: Record<string, Domain>;
  current: Domain;
}

const domains = [
  {
    name: "public",
    managers: ["work", "private"],
  },
  {
    name: "work",
    managers: ["private"],
  },
  {
    name: "pivate",
    managers: [],
  },
];

const getDefaultState = (): State => {
  return {
    domainMapByName: domains.reduce((acc, v) => {
      acc[v.name] = v;
      return acc;
    }, {} as Record<string, any>),
    current: domains[0],
  };
};

export const useDomainStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    fetchDomains: async () => {
      const domainMap = get().domainMapByName;
      for (const domain of domains) {
        domainMap[domain.name] = domain;
      }
      set({ domainMapByName: domainMap });
      return domains;
    },
    changeDomain: async (domain: string) => {
      const newDomain = get().domainMapByName[domain];
      set({
        current: newDomain,
      });
    },
    getCurrent: () => {
      return get().current;
    },
    getDomain: (domainName: string) => {
      return get().domainMapByName[domainName];
    },
    domains: () => {
      return Object.values(get().domainMapByName);
    },
  }))
);
