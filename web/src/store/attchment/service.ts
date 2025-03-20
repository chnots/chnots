import request from "@/utils/request";

export const resourceUpload = async (file: File): Promise<ResourceUploadRsp> => {
  const data = new FormData();
  data.append("file", file);
  return await request.put("api/v1/resource", data,
    {
      "Content-Type": "multipart/form-data",
    },
  );
};
