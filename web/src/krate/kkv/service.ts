import type { KKVCommitReq, KKVCommitRsp, KKVFetchReq, KKVFetchRsp } from './dto';
import request from '@/lib/request';

export const kkvCommit = async (req: KKVCommitReq): Promise<KKVCommitRsp> => {
  return await request.putJson('api/v1/kkv', req);
};

export const kkvFetch = async (req: KKVFetchReq): Promise<KKVFetchRsp> => {
  return await request.get('api/v1/kkv', req);
};
