import React from "react";
import { useInView } from "react-intersection-observer";

const KPageList = ({
  children,
  onFetchMore,
  isFetchingNextPage,
  hasNextPage,
}: {
  children: React.ReactNode;
  onFetchMore: () => void;
  isFetchingNextPage: boolean;
  hasNextPage: boolean;
}) => {
  const [ref, inView] = useInView();

  React.useEffect(() => {
    if (inView && !isFetchingNextPage && hasNextPage) {
      onFetchMore();
    }
  }, [hasNextPage, inView, isFetchingNextPage, onFetchMore]);

  return (
    <ul className="m-0 grid gap-2 pt-2 pr-1 pb-1 pl-2">
      {children}
      <div className="flex justify-center">
        {isFetchingNextPage ? (
          <div className="flex items-center gap-1 py-3">
            <span
              className="h-2 w-2 rounded-full bg-muted-foreground animate-bounce"
              style={{ animationDelay: "0ms", animationDuration: "0.6s" }}
            />
            <span
              className="h-2 w-2 rounded-full bg-muted-foreground animate-bounce"
              style={{ animationDelay: "150ms", animationDuration: "0.6s" }}
            />
            <span
              className="h-2 w-2 rounded-full bg-muted-foreground animate-bounce"
              style={{ animationDelay: "300ms", animationDuration: "0.6s" }}
            />
          </div>
        ) : hasNextPage ? (
          <button
            type="button"
            ref={ref}
            onClick={() => onFetchMore()}
            disabled={!hasNextPage || isFetchingNextPage}
            className="border-none p-3 bg-transparent m-3 text-sm"
          >
            Load Newer
          </button>
        ) : (
          <div className="text-xs m-5">~ End ~</div>
        )}
      </div>
    </ul>
  );
};

export default KPageList;
