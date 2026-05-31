import { combine } from "zustand/middleware";
import { create } from "zustand";

import type { ChnotKind } from "@/krate/chnot/po";
import type { TID } from "@/lib/id_util";

type SelectedItem = { otid: TID; kind: ChnotKind };

type State = {
  selectedItem: SelectedItem | undefined;
};

const getDefaultState = (): State => ({
  selectedItem: undefined,
});

export const useMdwtThreadStore = create(
  combine(getDefaultState(), (set) => ({
    setSelectedItem: (item: SelectedItem | undefined) => set({ selectedItem: item }),
  })),
);
