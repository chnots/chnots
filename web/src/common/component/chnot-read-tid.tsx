import type { TID } from "@/lib/id_util";
import dayjs from "dayjs";

const ReadableTID = ({ tid }: { tid: TID }) => {
  const date = new Date(tid / 1000);
  return (
    <span title={date.toLocaleString()} className="text-xs px-2">
      {dayjs(date).format("YYMM-DD")}
    </span>
  );
};

export default ReadableTID;