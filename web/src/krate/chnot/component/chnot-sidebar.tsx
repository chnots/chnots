import { useEffect, useState } from "react";

import { useChnotStore } from "@/krate/chnot/store/store";
import KPageList from "@/common/component/kpagelist";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";
import { chnotTagNames } from "@/krate/chnot/store/service";
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
import { useKSpaceStore } from "@/krate/kspace/store/store";
import { ChnotSidebarItem, ChnotSidebarTagItem } from "./chnot-sidebar-item";

const TagPath = () => {
  const { setListViewType, listViewType } = useChnotStore();
  if (listViewType.kind !== "tagtree") {
    return <></>;
  }

  const { tagkind, tagpath } = listViewType;
  const parts = tagpath.length > 0 ? tagpath.split("/") : [];

  const segments: string[] = [];
  if (parts.length > 0) {
    segments.push(parts[0]);
    for (let i = 1; i < parts.length; i++) {
      segments.push(`${segments[i - 1]}/${parts[i]}`);
    }
    parts[0] = parts[0].replace(RegExp("#"), "");
  }

  return (
    <div className="w-full flex flex-row space-x-1 items-center">
      <Toggle
        className="py-1 px-2"
        onClick={() => {
          if (tagkind === "children") {
            setListViewType({ ...listViewType, tagkind: "descendants" });
          } else {
            setListViewType({ ...listViewType, tagkind: "children" });
          }
        }}
      >
        <Icon.Flag />
      </Toggle>
      <Button
        className="p-1"
        key={"#root"}
        variant={"link"}
        onClick={() => {
          setListViewType({ ...listViewType, tagpath: "" });
        }}
      >
        #
      </Button>
      {parts.map((layer, index) => {
        return (
          <>
            <Button
              className="p-1"
              variant={"link"}
              key={segments[index]}
              onClick={() => {
                setListViewType({ ...listViewType, tagpath: segments[index] });
              }}
            >
              <span>{layer}</span>
            </Button>
            <span>/</span>
          </>
        );
      })}
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
    listViewType,
    setListViewType,
  } = useChnotStore();

  const [keyword, setKeyword] = useState<string>();
  const [tagList, setTagList] = useState<string[]>();

  const { currentKSpace, setKSpace: changeKSpace, mkspaces } = useKSpaceStore();

  useEffect(() => {
    refreshChnots();
  }, [currentKSpace]);

  useEffect(() => {
    changeKeyword(keyword);
  }, [keyword]);

  useEffect(() => {
    if (listViewType.kind === "tagtree") {
      chnotTagNames({
        start_index: 0,
        page_size: 9999,
        tag_tree: listViewType,
        query: keyword,
      }).then((rsp) => {
        setTagList(rsp.data);
      });
    }
    refreshChnots();
  }, [
    listViewType,
    keyword,
    mkspaces,
    currentKSpace,
  ]);

  return (
    <Sidebar variant="inset">
      <SidebarHeader className="text-sm">
        <div className="flex flex-row">
          <KSpaceSelect
            onSelect={function (kspace: string): void {
              changeKSpace(kspace);
            }}
            currentKSpace={currentKSpace}
            onlyIcon={true}
            showMKspaces={true}
          />
          <Toggle
            size={"sm"}
            onClick={() => {
              if (listViewType.kind !== "timeline") {
                setListViewType({ kind: "timeline" });
              } else {
                setListViewType({
                  kind: "tagtree",
                  tagkind: "children",
                  tagpath: "",
                });
              }
            }}
          >
            <Icon.Folder />
          </Toggle>
        </div>
        <TagPath />
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
      <SidebarSeparator />
      <SidebarContent>
        <div className="overflow-auto h-full overflow-x-hidden overflow-y-auto">
          {tagList && listViewType.kind === "tagtree" && (
            <ul className="m-0 grid gap-2 pt-2 pr-1 pb-1 pl-2">
              {tagList.map((tagpath) => (
                <ChnotSidebarTagItem
                  key={tagpath}
                  tag={tagpath}
                  onClick={() => {
                    if (listViewType.kind === "tagtree") {
                      setListViewType({ ...listViewType, tagpath });
                    }
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
                key={chnot.record.id}
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
