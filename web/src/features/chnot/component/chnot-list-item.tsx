import React, { ForwardedRef } from "react";
import { chnotShortDate } from "@/utils/date-utils";
import KListItem from "@/common/component/klistitem";
import Icon from "@/common/component/icon";
import { useChnotStore } from "@/store/chnot/store";
import { Chnot } from "@/store/chnot/dto";
import { chnotUpdate } from "@/store/chnot/service";
import { ChnotType } from "@/store/chnot/db";

const ChnotListItem = React.forwardRef(
  ({ chnot }: { chnot: Chnot }, ref: ForwardedRef<HTMLLIElement>) => {
    const { setCurrentChnotMetaId, getCurrentChnot, validateChnotCache } =
      useChnotStore();

    const onClick = (_: React.MouseEvent) => {
      setCurrentChnotMetaId(chnot.meta.id);
    };

    const currentChnot = getCurrentChnot();

    const isSelected = currentChnot?.record.id === chnot.record.id;

    const title = chnot.record.content.startsWith("# ")
      ? chnot.record.content.split("\n")[0]
      : chnot.record.content.substring(0, 500);

    const onDelete = async () => {
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
        onClick={onClick}
        ref={ref}
        key={chnot.record.id}
      >
        <div className="relative flex flex-row align-middle justify-between">
          <div className="flex flex-row space-x-1">
            {chnot.meta.kind === ChnotType.MarkdownWithToent && (
              <Icon.Ampersand className="h-4 w-4 min-w-4 text-green-600" />
            )}
            {chnot.meta.kind === ChnotType.ExcalidrawV1 && (
              <Icon.PencilRuler className="h-4 w-4 min-w-4 text-red-600" />
            )}
            <div className="text-gray-600 mr-2">
              {chnotShortDate(chnot.meta.insert_time)}
            </div>
            <div className="relative text-xs line-clamp-1 break-all">
              {title}
            </div>
          </div>
          <div className="opacity-0 hover:opacity-100">
            <button onClick={onDelete}>
              <Icon.Archive className="h-4" />
            </button>
          </div>
        </div>
      </KListItem>
    );
  }
);

ChnotListItem.displayName = "ChnotListItem";

export default ChnotListItem;
