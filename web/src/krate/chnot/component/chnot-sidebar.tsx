import { useEffect, useState } from "react";

import { useChnotStore } from "@/krate/chnot/store";
import KPageList from "@/common/component/kpagelist";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";
import { chnotTagNames } from "@/krate/chnot/service";
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
import { toast } from "sonner";
import { ChnotKindSelect } from "./chnot-kind-select";

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

  const { currentKSpace, setKSpace, mkspaces } = useKSpaceStore();

  useEffect(() => {
    refreshChnots();
  }, [currentKSpace]);

  useEffect(() => {
    changeKeyword(keyword);
  }, [keyword]);

  useEffect(() => {
    if (tags) {
      chnotTagNames({
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
  }, [tags, keyword, mkspaces, currentKSpace, kinds]);

  return (
    <Sidebar>
      <SidebarHeader className="text-sm">
        <div className="flex flex-row space-x-2">
          <KSpaceSelect
            onSelect={function (kspace: string): void {
              setKSpace(kspace);
            }}
            currentKSpace={currentKSpace}
            onlyIcon={true}
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
                chnot={chnot}
                key={chnot.record.tid}
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
