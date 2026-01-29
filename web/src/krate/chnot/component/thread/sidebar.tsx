import { useCallback, useEffect, useState } from "react";
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
import {
  ChnotViewType,
  useChnotHeadStore,
  useChnotThreadStore,
} from "../../store";
import Header from "../header/chnot-sidebar-header";
import { ChnotSidebarItemMemo, ChnotSidebarTagItem } from "../sidebar-item";
import { chnotThreadMetaOverwrite } from "../../service";

const ChnotThreadSidebar = () => {
  const {
    curOtid,
    isFetchingNextPage,
    mapByOtid,
    fetchMore,
    clearCache,
    setCurOtid,
    changeCompCurOtid,
    unvalidate,
  } = useChnotThreadStore((store) => {
    return {
      curOtid: store.curOtid,
      isFetchingNextPage: store.isFetchingNextPage,
      mapByOtid: store.mapByOtid,
      fetchMore: store.fetchMore,
      clearCache: store.clearCache,
      setCurOtid: store.setCurOtid,
      changeCompCurOtid: store.changeCompCurOtid,
      unvalidate: store.unvalidate,
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
    if (kinds || mkspaces) {
      clearCache();
      fetchMore();
    }
  }, [tags, searchStr, clearCache, fetchMore, kinds, mkspaces]);

  const handleTogglePin = useCallback(
    async (otid: TID) => {
      const meta = mapByOtid.cache.get(otid)?.meta;
      if (meta) {
        chnotThreadMetaOverwrite({
          meta_otid: otid,
          pinned: !!meta.pin_tid && meta.pin_tid > 0,
        });
      }
    },
    [mapByOtid],
  );

  const handleArchive = useCallback(
    async (otid: TID) => {
      const meta = mapByOtid.cache.get(otid)?.meta;
      if (meta) {
        chnotThreadMetaOverwrite({
          meta_otid: otid,
          archive: true,
        });
      }
    },
    [mapByOtid],
  );

  const handleChangeKspace = useCallback(
    async (otid: TID, kspace: string) => {
      const meta = mapByOtid.cache.get(otid)?.meta;
      if (meta) {
        chnotThreadMetaOverwrite({
          meta_otid: otid,
          kspace: kspace,
        });
      }
    },
    [mapByOtid],
  );

  const handleSetCurOtid = useCallback(
    (curOtid?: TID) => {
      setCurOtid(curOtid);
      if (changeCompCurOtid) {
        changeCompCurOtid(curOtid);
      }
    },
    [changeCompCurOtid, setCurOtid],
  );

  const handleUnvalidate = useCallback((toRemoves: TID[]) => {
    unvalidate(toRemoves);
  }, []);
  return (
    <Sidebar>
      <SidebarHeader className="text-sm">
        <Header viewType={ChnotViewType.Thread} />
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
                showKSpace={mkspaces.length > 0}
                isCurrent={curOtid === chnot.meta.otid}
                setCurOtid={handleSetCurOtid}
                unvalidate={handleUnvalidate}
                onArchive={handleArchive}
                onTogglePin={handleTogglePin}
                onChangeKspace={handleChangeKspace}
              />
            ))}
          </KPageList>
        </div>
      </SidebarContent>
    </Sidebar>
  );
};

export default ChnotThreadSidebar;
