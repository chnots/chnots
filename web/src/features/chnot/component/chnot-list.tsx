import { useEffect } from "react";

import ChnotListItem from "./chnot-list-item";
import { useChnotStore } from "@/store/chnot/store";
import KPageList from "@/common/component/kpagelist";
import ChnotSearch from "./chnot-search";

function ChnotList({ keyword }: { keyword?: string }) {
  const {
    fetchMoreChnots,
    isFetchingNextPage,
    hasNextPage,
    chnotMapByMetaId,
    changeKeyword,
  } = useChnotStore();

  useEffect(() => {
    changeKeyword(keyword);
  }, [changeKeyword, keyword]);

  return (
    <div className="relative">
      <ChnotSearch />
      
      <KPageList
        onFetchMore={fetchMoreChnots}
        isFetchingNextPage={isFetchingNextPage}
        hasNextPage={hasNextPage}
      >
        {[...chnotMapByMetaId.values()].map((chnot) => (
          <ChnotListItem chnot={chnot} key={chnot.record.id} />
        ))}
      </KPageList>
    </div>
  );
}

export default ChnotList;
