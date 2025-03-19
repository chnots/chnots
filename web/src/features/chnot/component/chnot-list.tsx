import React from "react";
import { useInView } from "react-intersection-observer";

import ChnotListItem from "./chnot-list-item";
import { useChnotStore } from "@/store/chnot/store";
import KPageList from "@/common/component/kpagelist";
import ChnotSearch from "./chnot-search";

export interface ChnotListProps {
  keyword?: string;
}

function ChnotList(props: ChnotListProps) {
  const {
    fetchMoreChnots,
    isFetchingNextPage,
    hasNextPage,
    chnotMap,
    changeKeyword,
  } = useChnotStore();

  React.useEffect(() => {
    changeKeyword(props.keyword);
  }, [changeKeyword, props.keyword]);

  return (
    <div className="relative">
      <ChnotSearch />
      <KPageList
        onFetchMore={fetchMoreChnots}
        isFetchingNextPage={isFetchingNextPage}
        hasNextPage={hasNextPage}
      >
        {[...chnotMap.values()].map((chnot) => (
          <ChnotListItem chnot={chnot} key={chnot.record.id} />
        ))}
      </KPageList>
    </div>
  );
}

export default ChnotList;
