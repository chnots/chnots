import request from "@/utils/request";
import { create } from "zustand";
import { combine } from "zustand/middleware";

interface State {}

interface Resource {
  id: string;

  namespace?: string;
  oriFilename: string;

  contentType: string;

  deleteTime?: string; // Using ISO 8601 format for DateTime
  insertTime: string; // Using ISO 8601 format for DateTime
}

interface ResourceUploadRsp {
  resources: Resource[];
}

const getDefaultState = (): State => {
  return {};
};

export const useAttachmentStore = create(
  combine(getDefaultState(), (_set, get) => ({
    getState: () => get(),
    upload: async (file: File) => {
      const data = new FormData();
      data.append("file", file);

      const resources: ResourceUploadRsp = await request.put(
        "api/v1/resource",
        data,
        {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        }
      );

      return resources.resources[0];
    },
  }))
);
