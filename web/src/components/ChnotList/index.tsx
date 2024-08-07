import React from "react";
import { useInView } from "react-intersection-observer";

import { useInfiniteQuery } from "@tanstack/react-query";
import { ChnotQueryRsp, queryChnot } from "@/helpers/data-agent";
import { MarkdownChnot } from "../ChnotView/MarkdownChnot";
import { ChnotViewMode } from "../ChnotView";

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
                    <div className="group relative flex flex-col justify-start items-start w-full px-4 py-3 mb-2 gap-2 bg-white dark:bg-zinc-800 rounded-lg border border-white dark:border-zinc-800 hover:border-gray-200 dark:hover:border-zinc-700">
                      <MarkdownChnot
                        chnot={chnot}
                        viewMode={ChnotViewMode.Preview}
                        key={chnot.id}
                      />
                    </div>
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
    </div>
  );
}

export default ChnotList;
