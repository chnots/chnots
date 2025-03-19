import request from "@/utils/request";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { resourceUpload } from "./service";

type State = object;

const getDefaultState = (): State => {
  return {};
};

export const useAttachmentStore = create(
  combine(getDefaultState(), (_set, get) => ({
    getState: () => get(),
    upload: async (file: File) => {
      const resources: ResourceUploadRsp = await resourceUpload(file);

      return resources.resources[0];
    },
  }))
);
