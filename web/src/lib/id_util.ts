import { v4 as uuid } from "uuid";
import { number } from "zod";

export const genUID = () => {
  return uuid().replaceAll("-", "") + "-UID";
};

export type TID = number;
export type OmitTID = TID;
let counter = 0;

export const genTID = () => {
  const c = counter++ % 1000;
  return Date.now() * 1000 + c;
};

export const omit_tid_never = () => {
  return -404;
};