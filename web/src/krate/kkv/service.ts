import request from "@/lib/request";
import { KKVInserterReq, KKVQueryReq, KKVQueryRsp } from "./dto";

export const insertKKV = async (req: KKVInserterReq) => {
  return await request.putJson("api/v1/kv", req);
};

export const queryKKV = async (req: KKVQueryReq): Promise<KKVQueryRsp> => {
  return await request.get("api/v1/kv", req);
};
