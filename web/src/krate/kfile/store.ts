import { create } from "zustand";
import { combine } from "zustand/middleware";

type State = object;

const getDefaultState = (): State => {
  return {};
};

export const useAttachmentStore = create(
  combine(getDefaultState(), (_set) => ({})),
);
