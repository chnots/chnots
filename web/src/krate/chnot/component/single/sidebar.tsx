import { useCallback, useEffect, useState } from "react";

import {
  type ChnotViewType,
  useChnotHeadStore,
  useChnotSingleStore,
} from "../../store";
import Header from "../header/chnot-sidebar-header";
import { ChnotSidebarItemMemo, ChnotSidebarTagItem } from "../sidebar-item";

import KPageList from "@/common/component/kpagelist";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarSeparator,
} from "@/common/component/ui/sidebar";
import { useKSpaceStore } from "@/krate/kspace/store";
import { chnotTagNameList } from "@/krate/mdwt/service";
import type { TID } from "@/lib/id_util";

const ChnotSingleSidebar = ({ viewType }: { viewType: ChnotViewType }) => {
  const {
    curOtid,
    isFetchingNextPage,
    mapByOtid,
    fetchMore,
    clearCache,
    setCurOtid,
    changeCompCurOtid,
  } = useChnotSingleStore((store) => {
    return {
      curOtid: store.curOtid,
      isFetchingNextPage: store.isFetchingNextPage,
      mapByOtid: store.mapByOtid,
      fetchMore: store.fetchMore,
      clearCache: store.clearCache,
      setCurOtid: store.setCurOtid,
      changeCompCurOtid: store.changeCompCurOtid,
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
  }, [tags, searchStr, clearCache, fetchMore]);

  const ph = useCallback(() => {}, []);
  const handleSetCurOtid = useCallback(
    (curOtid?: TID) => {
      setCurOtid(curOtid);
      if (changeCompCurOtid) {
        changeCompCurOtid(curOtid);
      }
    },
    [changeCompCurOtid, setCurOtid],
  );

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
              <ChnotSidebarItemMemo
                item={chnot}
                key={chnot.meta.otid}
                kind={chnot.meta.kind}
                showKSpace={mkspaces.length > 0}
                isCurrent={curOtid === chnot.meta.otid}
                setCurOtid={handleSetCurOtid}
                unvalidate={ph}
                onArchive={ph}
                onTogglePin={ph}
              />
            ))}
          </KPageList>
        </div>
      </SidebarContent>
    </Sidebar>
  );
};

export default ChnotSingleSidebar;
