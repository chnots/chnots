import { useEffect, useState } from "react";

import { useChnotStore } from "@/krate/chnot/store";
import KPageList from "@/common/component/kpagelist";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarSeparator,
} from "@/common/component/ui/sidebar";
import { useKSpaceStore } from "@/krate/kspace/store";
import { ChnotSidebarItem, ChnotSidebarTagItem } from "./chnot-sidebar-item";
import { chnotTagNameList } from "@/krate/mdwt/service";

const ChnotSidebar = () => {
  const {
    fetchMoreChnotThreads,
    refreshChnotThreads,
    isFetchingNextPage,
    threadMapByThreadId,
    tags,
    setTagsInset,
    kinds,
    searchStr,
  } = useChnotStore((store) => {
    return {
      fetchMoreChnotThreads: store.fetchMoreChnotThreads,
      refreshChnotThreads: store.refreshChnotThreads,
      isFetchingNextPage: store.isFetchingNextPage,
      threadMapByThreadId: store.threadMapByOtid,
      tags: store.tags,
      setTagsInset: store.setTagsInset,
      kinds: store.kinds,
      searchStr: store.query,
    };
  });

  const [tagList, setTagList] = useState<string[]>();

  const { mkspaces } = useKSpaceStore((store) => {
    return {
      mkspaces: store.mkspaces,
    };
  });

  useEffect(() => {
    if (tags) {
      chnotTagNameList({
        start_index: 0,
        page_size: 9999,
        tags,
        query: searchStr,
      }).then((rsp) => {
        setTagList(rsp.data);
      });
    } else {
      setTagList(undefined);
    }
    refreshChnotThreads();
  }, [tags, searchStr, mkspaces, kinds]);

  return (
    <Sidebar>
      <SidebarHeader className="text-sm"></SidebarHeader>
      <SidebarSeparator className="mx-0" />
      <SidebarContent>
        <div className="overflow-auto h-full overflow-x-hidden overflow-y-auto">
          {tagList && (
            <ul className="m-0 gap-2 pt-2 pr-1 pb-1 pl-2">
              {tagList.map((tagName) => (
                <ChnotSidebarTagItem
                  key={tagName}
                  tag={tagName}
                  onClick={() => {
                    setTagsInset([
                      ...new Set([...(tags?.Inset ?? []), tagName]),
                    ]);
                  }}
                />
              ))}
            </ul>
          )}
          <KPageList
            onFetchMore={fetchMoreChnotThreads}
            isFetchingNextPage={isFetchingNextPage}
            hasNextPage={threadMapByThreadId.hasNextPage}
          >
            {[...threadMapByThreadId.dbCache.values()].map((chnot) => (
              <ChnotSidebarItem
                chnotThread={chnot}
                key={chnot.meta.otid}
                showKSpace={mkspaces.length > 0}
              />
            ))}
          </KPageList>
        </div>
      </SidebarContent>
    </Sidebar>
  );
};

export default ChnotSidebar;
