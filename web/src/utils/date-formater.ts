import dayjs from "dayjs";

export const chnotShortDate = (date: Date) => {
  return dayjs(date).format("YYMM-DD");
};
