import React, { ForwardedRef } from "react";
import { chnotShortDate } from "@/utils/date-utils";
import KListItem from "@/common/component/klistitem";
import Icon from "@/common/component/icon";
import { useChnotStore } from "@/store/chnot/store";
import { Chnot } from "@/store/chnot/dto";
import { chnotUpdate } from "@/store/chnot/service";

const ChnotListItem = React.forwardRef(
  ({ chnot }: { chnot: Chnot }, ref: ForwardedRef<HTMLLIElement>) => {
    const { setCurrentChnot, getCurrentChnot, validateChnotCache } =
      useChnotStore();

    const handleClick = (_: React.MouseEvent) => {
      setCurrentChnot(chnot);
    };

    const currentChnot = getCurrentChnot();

    const isSelected = currentChnot?.record.id === chnot.record.id;

    const title = chnot.record.content.startsWith("# ")
      ? chnot.record.content.split("\n")[0]
      : chnot.record.content.substring(0, 500);

    const handleDelete = async () => {
      await chnotUpdate({
        meta_id: chnot.meta.id,
        archive: true,
        update_time: false,
      });
      validateChnotCache([chnot.meta.id]);
    };

    return (
      <KListItem
        focused={isSelected}
        onClick={handleClick}
        ref={ref}
        key={chnot.record.id}
      >
        <div className="relative flex flex-row justify-between">
          <div className="text-xs">
            {chnotShortDate(chnot.meta.insert_time)}
          </div>

          <div className="opacity-0 hover:opacity-100">
            <button onClick={handleDelete}>
              <Icon.Archive className="h-4" />
            </button>
          </div>
        </div>

        <div className="relative text-xs line-clamp-2 break-all">{title}</div>
      </KListItem>
    );
  }
);

ChnotListItem.displayName = "ChnotListItem";

export default ChnotListItem;
