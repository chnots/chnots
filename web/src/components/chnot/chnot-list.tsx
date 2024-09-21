import React, { useEffect } from "react";
import { useInView } from "react-intersection-observer";

import { useChnotStore } from "@/store/chnot";
import useCurrentnamespace from "@/hooks/use-current-namespace";
import { ChnotListItem } from "./chnot-list-item";

export interface ChnotListProps {
  keyword?: string;
}

function ChnotList(props: ChnotListProps) {
  const [ref, inView] = useInView();
  const namespace = useCurrentnamespace();
  const {
    fetchMoreChnots,
    refreshChnots,
    isFetchingNextPage,
    hasNextPage,
    chnotMap: chnotMap,
    changeKeyword,
  } = useChnotStore();

  React.useEffect(() => {
    changeKeyword(props.keyword);
  }, [changeKeyword, props.keyword]);

  React.useEffect(() => {
    if (inView && !isFetchingNextPage && hasNextPage) {
      fetchMoreChnots();
    }
  }, [fetchMoreChnots, hasNextPage, inView, isFetchingNextPage]);

  useEffect(() => {
    refreshChnots();
  }, [namespace]);

  return (
    <ul className="w-full p-2 space-y-2">
      {chnotMap.values().map((chnot) => (
        <ChnotListItem chnot={chnot} />
      ))}
      <div className="flex justify-center">
        {isFetchingNextPage ? (
          "Loading more..."
        ) : hasNextPage ? (
          <button
            ref={ref}
            onClick={() => fetchMoreChnots()}
            disabled={!hasNextPage || isFetchingNextPage}
            className="border-none p-3 bg-transparent m-3 text-sm"
          >
            "Load Newer"
          </button>
        ) : (
          <div className="text-xs m-5">~ End ~</div>
        )}
      </div>
    </ul>
  );
}

export default ChnotList;
