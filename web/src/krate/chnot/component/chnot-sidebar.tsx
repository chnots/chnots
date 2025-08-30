import { useEffect, useState } from "react";

import { useChnotStore } from "@/krate/chnot/store";
import KPageList from "@/common/component/kpagelist";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarInput,
  SidebarSeparator,
} from "@/common/component/ui/sidebar";
import { Toggle } from "@/common/component/ui/toggle";
import { Label } from "@radix-ui/react-dropdown-menu";
import { Search } from "lucide-react";
import { KSpaceSelect } from "@/krate/kspace/component/kspace-select";
import { useKSpaceStore } from "@/krate/kspace/store";
import { ChnotSidebarItem, ChnotSidebarTagItem } from "./chnot-sidebar-item";
import { useShallow } from "zustand/react/shallow";
import { ChnotKindSelect } from "./chnot-kind-select";
import { useCommonStore } from "@/common/store";
import { NavLink } from "react-router-dom";
import { RoutePaths } from "@/router";
import { chnotThreadTagNames } from "../service";

const TagsView = () => {
  const { setTagsInset, tags } = useChnotStore(
    useShallow((store) => {
      return {
        setTagsInset: store.setTagsInset,
        tags: store.tags,
      };
    }),
  );

  return (
    <div className="w-full flex-row space-x-1 items-center inline">
      {tags ? (
        tags.Inset.map((tag) => (
          <Button
            key={tag}
            onClick={() => {
              setTagsInset([
                ...new Set([...tags.Inset.filter((e) => e != tag)]),
              ]);
            }}
          >
            {tag}
          </Button>
        ))
      ) : (
        <div />
      )}
    </div>
  );
};

const ChnotSidebar = () => {
  const {
    fetchMoreChnots,
    refreshChnots,
    isFetchingNextPage,
    chnotMapByMetaId,
    changeKeyword,
    tags,
    setTagsInset,
    setTags,
    kinds,
  } = useChnotStore();

  const [keyword, setKeyword] = useState<string>();
  const [tagList, setTagList] = useState<string[]>();
  const { toggleSettings, showSettings } = useCommonStore();

  const { currentKSpace, selectKSpace, mkspaces } = useKSpaceStore((store) => {
    return {
      currentKSpace: store.currentKSpace,
      selectKSpace: store.selectKSpace,
      mkspaces: store.mkspaces,
    };
  });

  useEffect(() => {
    changeKeyword(keyword);
  }, [keyword]);

  useEffect(() => {
    if (tags) {
      chnotThreadTagNames({
        start_index: 0,
        page_size: 9999,
        tags,
        query: keyword,
      }).then((rsp) => {
        setTagList(rsp.data);
      });
    } else {
      setTagList(undefined);
    }
    refreshChnots();
  }, [tags, keyword, mkspaces, kinds]);

  return (
    <Sidebar>
      <SidebarHeader className="text-sm">
        <div className="flex justify-between items-center">
          <div className="flex align-center space-x-1">
            <KSpaceSelect
              onSelect={function (kspace: string): void {
                selectKSpace(kspace);
              }}
              currentKSpace={currentKSpace}
              showMKspaces={true}
            />
            <ChnotKindSelect />
            <Toggle
              size={"sm"}
              onClick={() => {
                if (tags) {
                  setTags(undefined);
                } else {
                  setTagsInset([]);
                }
              }}
            >
              <Icon.Hash />
            </Toggle>
          </div>
          <div>
            <NavLink to={RoutePaths.Settings} id={"Settings"}>
              <div>
                <Icon.Settings className="w-4 h-4 mx-2" />
              </div>
            </NavLink>
          </div>
        </div>
        <TagsView />
        <form>
          <SidebarGroup className="py-0">
            <SidebarGroupContent className="relative">
              <Label className="sr-only">Search</Label>
              <SidebarInput
                id="search"
                placeholder="Search the docs..."
                className="pl-8"
                onChange={(e) => {
                  setKeyword(e.target.value);
                }}
              />
              <Search className="pointer-events-none absolute top-1/2 left-2 size-4 -translate-y-1/2 opacity-50 select-none" />
            </SidebarGroupContent>
          </SidebarGroup>
        </form>
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
            onFetchMore={fetchMoreChnots}
            isFetchingNextPage={isFetchingNextPage}
            hasNextPage={chnotMapByMetaId.hasNextPage}
          >
            {[...chnotMapByMetaId.dbCache.values()].map((chnot) => (
              <ChnotSidebarItem
                chnotThread={chnot}
                key={chnot.head_chnot.tid}
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
