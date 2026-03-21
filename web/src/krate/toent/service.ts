import request from "@/lib/request";
import type {
  ToentGuessReq,
  ToentGuessRsp,
  ToentInstCommitReq,
  ToentInstCommitRsp,
  ToentInstCountReq,
  ToentInstCountRsp,
  ToentSearchReq,
  ToentSearchRsp,
  ToentTodoStateCommitReq,
  ToentTodoStateCommitRsp,
} from "./dto";
import type { TodoEvent, ToentTimeEvent } from "./toent-model";

export const toentTimeEventGuess = async (
  req: ToentGuessReq,
): Promise<ToentGuessRsp<ToentTimeEvent>> => {
  return await request.postJson(`api/v1/toent-timeevent-guess`, req);
};

export const toentTodoEventGuess = async (
  req: ToentGuessReq,
): Promise<ToentGuessRsp<TodoEvent>> => {
  return await request.postJson(`api/v1/toent-todoevent-guess`, req);
};

export const toentInstCount = async (
  req: ToentInstCountReq,
): Promise<ToentInstCountRsp> => {
  return await request.postJson(`api/v1/toent-inst-count`, req);
};

export const toentSearch = async (
  req: ToentSearchReq,
): Promise<ToentSearchRsp> => {
  return await request.postJson(`api/v1/toent-search`, req);
};

export const toentTodoStateCommit = async (
  req: ToentTodoStateCommitReq,
): Promise<ToentTodoStateCommitRsp> => {
  return await request.postJson(`api/v1/toent-todo-state-commit`, req);
};

export const toentInstCommit = async (
  req: ToentInstCommitReq,
): Promise<ToentInstCommitRsp> => {
  return await request.postJson(`api/v1/toent-inst-commit`, req);
};
