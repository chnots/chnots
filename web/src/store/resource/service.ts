import request from "@/utils/request";
import { InsertInlineResourceReq, QueryInlineResourceReq, QueryInlineResourceRsp, ResourceUploadRsp } from "./dto";

export const resourceUpload = async (file: File): Promise<ResourceUploadRsp> => {
  const data = new FormData();
  data.append("file", file);
  return await request.put("api/v1/resource", data,
    {
      "Content-Type": "multipart/form-data",
    },
  );
};


export const insertInlineResource = async (req: InsertInlineResourceReq) => {
  return await request.put("api/v1/inline-resource", req);
}

export const queryInlineResource = async (req: QueryInlineResourceReq): Promise<QueryInlineResourceRsp> => {
  return await request.get("api/v1/inline-resource", req);
}