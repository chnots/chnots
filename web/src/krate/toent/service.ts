import request from "@/lib/request";
import type {
  ToentGuessReq,
  ToentGuessRsp,
  ToentInstListReq,
  ToentInstListRsp,
} from "./dto";
import type { TodoEvent, ToentTimeEvent } from "./po";

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

export const toentInstList = async (
  req: ToentInstListReq,
): Promise<ToentInstListRsp> => {
  return await request.postJson(`api/v1/toent-inst-list`, req);
};
