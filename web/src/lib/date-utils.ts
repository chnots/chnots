// Adopted from https://github.com/angelxmoreno/axios-date-transformer/blob/main/src/index.ts

import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";

dayjs.extend(relativeTime);

export const formatRelativeTime = (date: Date): string => {
  const now = dayjs();
  const target = dayjs(date);
  const diffMinutes = now.diff(target, "minute");

  if (diffMinutes < 1) return "just now";
  if (diffMinutes < 60) return target.fromNow();

  const sameDay = now.startOf("day").isSame(target.startOf("day"));
  if (sameDay) return target.format("HH:mm");

  const sameYear = now.year() === target.year();
  if (sameYear) return target.format("MM-DD HH:mm");

  return target.format("YYYY-MM-DD HH:mm");
};

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

export const chnotShortDate = (date?: Date) => {
  return dayjs(date ?? new Date()).format("YYMM-DD");
};

export const parseNaiveDateTime = (value?: string): Date | undefined => {
  if (!value) {
    return undefined;
  }

  const match = value.match(
    /^(\d{4})-(\d{2})-(\d{2})\s(\d{2}):(\d{2}):(\d{2})$/,
  );
  if (!match) {
    return undefined;
  }

  const [, y, m, d, hh, mm, ss] = match;
  return new Date(
    Number(y),
    Number(m) - 1,
    Number(d),
    Number(hh),
    Number(mm),
    Number(ss),
  );
};

export const tidToDate = (tid?: number): Date | undefined => {
  if (tid === undefined || Number.isNaN(tid)) {
    return undefined;
  }

  const abs = Math.abs(tid);
  const millis = abs >= 1_000_000_000_000_000 ? tid / 1000 : tid;
  const date = new Date(millis);
  if (Number.isNaN(date.getTime())) {
    return undefined;
  }
  return date;
};
