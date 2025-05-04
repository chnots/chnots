import React, { ForwardedRef } from "react";
import KListItem from "@/common/component/klistitem";
import Icon from "@/common/component/icon";

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
      <KListItem focused={focused} onClick={handleClick} ref={ref}>
        <div className="relative flex flex-row align-middle">
          <Icon.Hash className="h-4 w-4 min-w-4 text-blue-600" />
          <div className="relative text-xs line-clamp-1 break-all">
            {tag.replace(RegExp("#"), "")}
          </div>
        </div>
      </KListItem>
    );
  }
);

ChnotTagListItem.displayName = "ChnotTagListItem";

export default ChnotTagListItem;
