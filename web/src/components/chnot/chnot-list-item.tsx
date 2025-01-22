import { Chnot, useChnotStore } from "@/store/chnot";
import clsx from "clsx";
import React, { ForwardedRef } from "react";
import { chnotShortDate } from "../../utils/date-utils";

export const ChnotListItem = React.forwardRef(
  (props: { chnot: Chnot }, ref: ForwardedRef<HTMLLIElement>) => {
    const { setCurrentChnot, getCurrentChnot } = useChnotStore();
    const chnot = props.chnot;

    const handleClick = (_: React.MouseEvent) => {
      setCurrentChnot(chnot);
    };

    const isSelected = getCurrentChnot()?.record.id === chnot.record.id;

    return (
      <li
        className={clsx(
          "list-none rounded-2xl p-3 pl-6 grid gap-1 relative select-none border",
          "group hover:cursor-pointer",
          isSelected
            ? "bg-white text-black"
            : "text-gray-700 border-transparent hover:bg-white hover:border-gray-200"
        )}
        onClick={handleClick}
        ref={ref}
        key={chnot.record.id}
      >
        <div className="text-xs"> {chnotShortDate(chnot.meta.insert_time)}</div>
        <div className="text-xs line-clamp-2 break-all">
          {(chnot.record.content || "").replace(/<[^<>]+>/g, "")}
        </div>
      </li>
    );
  }
);
