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
          <div
            className="my-2 inline-flex items-center gap-2 rounded-full border border-sidebar-border/60 bg-sidebar-accent/30 px-3 py-1.5"
            aria-live="polite"
          >
            <span
              className="h-1.5 w-1.5 rounded-full bg-muted-foreground/90 animate-bounce"
              style={{ animationDelay: "0ms", animationDuration: "0.9s" }}
            />
            <span
              className="h-1.5 w-1.5 rounded-full bg-muted-foreground/80 animate-bounce"
              style={{ animationDelay: "120ms", animationDuration: "0.9s" }}
            />
            <span
              className="h-1.5 w-1.5 rounded-full bg-muted-foreground/70 animate-bounce"
              style={{ animationDelay: "240ms", animationDuration: "0.9s" }}
            />
            <span className="text-[0.68rem] text-muted-foreground">
              Loading
            </span>
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
