import React from "react";
import { useInView } from "react-intersection-observer";

import { useInfiniteQuery } from "@tanstack/react-query";
import { ChnotQueryRsp, queryChnot } from "@/helpers/data-agent";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

export interface ChnotListProps {
  query?: string;
}

function ChnotList(props: ChnotListProps) {
  const [ref, inView] = useInView();

  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
    error,
    status,
  } = useInfiniteQuery({
    queryKey: ["chnots", props.query],
    queryFn: async ({ pageParam }): Promise<ChnotQueryRsp> => {
      const e = await queryChnot({
        query: props.query,
        start_index: pageParam,
        page_size: 20,
      });
      return e;
    },
    initialPageParam: 0,
    getNextPageParam: (lastPage) => lastPage.next_start,
    enabled: true,
  });

  React.useEffect(() => {
    if (inView) {
      fetchNextPage();
    }
  }, [fetchNextPage, inView]);

  return (
    <div>
      {status === "pending" ? (
        <p>...Loading...</p>
      ) : status === "error" ? (
        <span>Error: {error.message}</span>
      ) : (
        <>
          {data?.pages.map(
            (page) =>
              page && (
                <React.Fragment key={page.next_start}>
                  {page.data.map((chnot) => (
                    <p
                      style={{
                        border: "1px solid gray",
                        borderRadius: "5px",
                        padding: "10rem 1rem",
                        background: `hsla(40%, 60%, 80%, 1)`,
                      }}
                      key={chnot.id}
                    >
                      {chnot.content}
                    </p>
                  ))}
                </React.Fragment>
              )
          )}
          <div>
            <button
              ref={ref}
              onClick={() => fetchNextPage()}
              disabled={!hasNextPage || isFetchingNextPage}
            >
              {isFetchingNextPage
                ? "Loading more..."
                : hasNextPage
                ? "Load Newer"
                : "Nothing more to load"}
            </button>
          </div>
        </>
      )}
      <ReactQueryDevtools initialIsOpen />
    </div>
  );
}

export default ChnotList;
