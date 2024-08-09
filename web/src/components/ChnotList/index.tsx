import React, { useEffect } from "react";
import { useInView } from "react-intersection-observer";

import ChnotView from "../ChnotView";
import { ChnotViewMode } from "../ChnotView";
import { useChnotStore } from "@/store/v1/chnot";
import { Button } from "@mui/joy";

export interface ChnotListProps {
  query?: string;
}

function ChnotList(props: ChnotListProps) {
  const [ref, inView] = useInView();
  const {
    fetchMoreChnots,
    isFetchingNextPage,
    hasNextPage,
    chnotPages,
    changeQuery,
  } = useChnotStore();

  React.useEffect(() => {
    changeQuery(props.query);
  }, [changeQuery, props.query]);

  useEffect(() => {
    console.log("changed: ", chnotPages.length);
  }, [chnotPages]);

  React.useEffect(() => {
    if (inView && !isFetchingNextPage && hasNextPage) {
      fetchMoreChnots();
    }
  }, [fetchMoreChnots, hasNextPage, inView, isFetchingNextPage]);

  return (
    <div>
      {chnotPages.map((page) => (
        <React.Fragment key={page.index}>
          {page.chnots.map((chnot) => (
            <ChnotView
              chnot={chnot}
              viewMode={ChnotViewMode.Preview}
              key={chnot.id}
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
