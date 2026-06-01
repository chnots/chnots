import { combine } from "zustand/middleware";
import { create } from "zustand";

import type { ChnotKind } from "@/krate/chnot/po";
import type { TID } from "@/lib/id_util";

export type MdwtThreadItem = { otid: TID; kind: ChnotKind };

type State = {
  selectedItem: MdwtThreadItem | undefined;
  fullscreenItem: MdwtThreadItem | undefined;
};

const getDefaultState = (): State => ({
  selectedItem: undefined,
  fullscreenItem: undefined,
});

export const useMdwtThreadStore = create(
  combine(getDefaultState(), (set) => ({
    setSelectedItem: (item: MdwtThreadItem | undefined) =>
      set({ selectedItem: item }),
    setFullscreenItem: (item: MdwtThreadItem | undefined) =>
      set({ fullscreenItem: item }),
  })),
);
