import { Workspace } from "@/model";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {
  workspaceMapByName: Map<string, Workspace>;
  currentWorkspace: Workspace;
}

const workspaces = new Map<string, Workspace>([
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
    workspaceMapByName: workspaces,
    currentWorkspace: (() => {
      const searchParams = new URLSearchParams(window.location.search.slice(1));
      console.log("search params:", location.hash);
      const ns = searchParams.get("ns");
      if (ns === null || !workspaces.has(ns)) {
        return workspaces.get("public")!;
      }

      return workspaces.get(ns)!;
    })(),
  };
};

export const useWorkspaceStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    fetchWorkspaces: async () => {
      set({ workspaceMapByName: workspaces });
      return workspaces;
    },
    changeWorkspace: async (workspace: string) => {
      console.log("change ns:", workspace);
      const newworkspace = get().workspaceMapByName.get(workspace);
      set({
        currentWorkspace: newworkspace,
      });
    },
    getWorkspace: (workspaceName: string) => {
      return get().workspaceMapByName.get(workspaceName);
    },
    workspaces: () => {
      return [...get().workspaceMapByName.values()];
    },
  }))
);
