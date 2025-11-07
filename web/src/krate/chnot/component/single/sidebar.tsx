import { useEffect, useState } from "react";

import KPageList from "@/common/component/kpagelist";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarSeparator,
} from "@/common/component/ui/sidebar";
import { useKSpaceStore } from "@/krate/kspace/store";
import { ChnotSidebarItem, ChnotSidebarTagItem } from "../sidebar-item";
import { chnotTagNameList } from "@/krate/mdwt/service";
import {
  ChnotViewType,
  StateChnotLike,
  useChnotHeadStore,
  useChnotSingleStore,
} from "../../store";
import { TID } from "@/lib/id_util";
import Header from "../header/chnot-sidebar-header";

const ChnotSingleSidebar = ({ viewType }: { viewType: ChnotViewType }) => {
  const { fetchMore, clearCache, isFetchingNextPage, mapByOtid } =
    useChnotSingleStore((store) => {
      return {
        fetchMore: store.fetchMore,
        clearCache: store.clearCache,
        isFetchingNextPage: store.isFetchingNextPage,
        mapByOtid: store.mapByOtid,
      };
    });

  const { tags, setTagsInset, kinds, searchStr } = useChnotHeadStore(
    (store) => {
      return {
        tags: store.tags,
        setTagsInset: store.setTagsInset,
        kinds: store.kinds,
        searchStr: store.searchStr,
      };
    },
  );

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
    clearCache();
    fetchMore();
  }, [tags, searchStr, mkspaces, kinds]);
  console.log("chnots", mapByOtid);

  return (
    <Sidebar>
      <SidebarHeader className="text-sm">
        <Header viewType={viewType} />
      </SidebarHeader>
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
            onFetchMore={fetchMore}
            isFetchingNextPage={isFetchingNextPage}
            hasNextPage={mapByOtid.hasMore}
          >
            {[...mapByOtid.cache.values()].map((chnot) => (
              <ChnotSidebarItem
                item={chnot}
                key={chnot.meta.otid}
                showKSpace={mkspaces.length > 0}
                getCurrent={function (): StateChnotLike | undefined {
                  return undefined;
                }}
                overwrite={function (chnot: StateChnotLike): void {}}
                setCurrOtid={function (cutOtid?: TID): void {}}
                unvalidate={function (toRemoves: TID[]): void {}}
              />
            ))}
          </KPageList>
        </div>
      </SidebarContent>
    </Sidebar>
  );
};

export default ChnotSingleSidebar;
