import { v4 as uuid } from "uuid";

export const genId = () => {
  return uuid();
};

export const genTSID = () => {
  return Date.now();
};