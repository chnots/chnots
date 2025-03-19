import React, { ForwardedRef } from "react";
import KListItem from "@/common/component/klistitem";

const ChnotTagListItem = React.forwardRef(
  (
    {
      tag,
      focused,
      handleClick,
    }: { tag: string; focused?: boolean; handleClick: () => void },
    ref: ForwardedRef<HTMLLIElement>
  ) => {
    return (
      <KListItem
        focused={focused}
        onClick={handleClick}
        ref={ref}
        className="flex-col"
      >
        <div className="text-xs line-clamp-2 break-all">{tag}</div>
      </KListItem>
    );
  }
);

ChnotTagListItem.displayName = "ChnotTagListItem";

export default ChnotTagListItem;
