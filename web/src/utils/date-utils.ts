// Adopted from https://github.com/angelxmoreno/axios-date-transformer/blob/main/src/index.ts

import dayjs from "dayjs";

export const recursiveDateConversion = (data: any): any => {
  if (typeof data === "object") {
    for (const key in data) {
      if (typeof data[key] === "string" && isDateString(data[key])) {
        data[key] = new Date(data[key]);
      } else if (typeof data[key] === "object") {
        data[key] = recursiveDateConversion(data[key]);
      }
    }
  }

  return data;
};

const isDateString = (value: any): boolean => {
  const dateRegex =
    /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d*)?(?:[-+]\d{2}:?\d{2}|Z)?$/;
  return dateRegex.test(value);
};


export const chnotShortDate = (date: Date) => {
  return dayjs(date).format("YYMM-DD");
};
