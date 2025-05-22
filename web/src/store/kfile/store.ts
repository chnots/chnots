import request from "@/utils/request";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { kfileUpload } from "./service";
import { KFileUploadRsp } from "./dto";

type State = object;

const getDefaultState = (): State => {
  return {};
};

export const useAttachmentStore = create(
  combine(getDefaultState(), (_set, get) => ({
    getState: () => get(),
  }))
);
