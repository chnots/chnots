import { v4 as uuid } from "uuid";
import { number } from "zod";

export const genUId = () => {
  return uuid();
};

export type TID = number;
let counter = 0;

export const genTID = () => {
  const c = counter++ % 1000;
  return Date.now() * 1000 + c;
};