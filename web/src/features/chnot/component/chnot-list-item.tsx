import { Chnot, useChnotStore } from "@/store/chnot";
import React, { ForwardedRef } from "react";
import { chnotShortDate } from "@/utils/date-utils";
import KListItem from "@/common/component/klistitem";

const ChnotListItem = React.forwardRef(
  (props: { chnot: Chnot }, ref: ForwardedRef<HTMLLIElement>) => {
    const { setCurrentChnot, getCurrentChnot } = useChnotStore();
    const chnot = props.chnot;

    const handleClick = (_: React.MouseEvent) => {
      setCurrentChnot(chnot);
    };

    const currentChnot = getCurrentChnot();

    const isSelected = currentChnot?.record.id === chnot.record.id;

    return (
      <KListItem
        focused={isSelected}
        onClick={handleClick}
        ref={ref}
        key={chnot.record.id}
        className="flex-col"
      >
        <div className="text-xs"> {chnotShortDate(chnot.meta.insert_time)}</div>
        <div className="text-xs line-clamp-2 break-all">
          {(chnot.record.content || "").replace(/<[^<>]+>/g, "")}
        </div>
      </KListItem>
    );
  }
);

ChnotListItem.displayName = "ChnotListItem";

export default ChnotListItem;
