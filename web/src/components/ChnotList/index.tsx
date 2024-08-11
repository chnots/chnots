import React, { useEffect } from "react";
import { useInView } from "react-intersection-observer";

import ChnotView from "../ChnotView";
import { ChnotViewMode } from "../ChnotView";
import { useChnotStore } from "@/store/v1/chnot";
import { Button } from "@mui/joy";
import useCurrentDomain from "@/hooks/useCurrentDomain";

export interface ChnotListProps {
  keyword?: string;
}

function ChnotList(props: ChnotListProps) {
  const [ref, inView] = useInView();
  const domain = useCurrentDomain();
  const {
    fetchMoreChnots,
    refreshChnots,
    isFetchingNextPage,
    hasNextPage,
    chnotPages,
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
  }, [domain]);

  return (
    <div>
      {chnotPages.map((page) => (
        <React.Fragment key={page.index}>
          {page.chnots.map((chnot) => (
            <ChnotView
              chnot={chnot}
              viewMode={ChnotViewMode.Preview}
              key={chnot.id}
              createInput={false}
            />
          ))}
        </React.Fragment>
      ))}
      <div>
        <Button
          ref={ref}
          onClick={() => fetchMoreChnots()}
          disabled={!hasNextPage || isFetchingNextPage}
        >
          {isFetchingNextPage
            ? "Loading more..."
            : hasNextPage
            ? "Load Newer"
            : "Nothing more to load"}
        </Button>
      </div>
    </div>
  );
}

export default ChnotList;
