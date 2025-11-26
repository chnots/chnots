import { Label } from "@radix-ui/react-dropdown-menu";
import { Search } from "lucide-react";
import { NavLink } from "react-router-dom";
import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarInput,
} from "@/common/component/ui/sidebar";
import { Toggle } from "@/common/component/ui/toggle";
import { KSpaceSelect } from "@/krate/kspace/component/kspace-select";
import { useKSpaceStore } from "@/krate/kspace/store";
import { RoutePaths } from "@/router";
import { type ChnotViewType, useChnotHeadStore } from "../../store";
import { ChnotKindSelect } from "./chnot-kind-select";
import ChnotThreadSwitch from "./chnot-thread-switch";

const TagsView = () => {
  const { setTagsInset, tags } = useChnotHeadStore((store) => {
    return {
      setTagsInset: store.setTagsInset,
      tags: store.tags,
    };
  });

  return (
    <div className="w-full flex-row space-x-1 items-center inline">
      {tags ? (
        tags.Inset.map((tag) => (
          <Button
            key={tag}
            onClick={() => {
              setTagsInset([...new Set(tags.Inset.filter((e) => e !== tag))]);
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

const Header = ({ viewType }: { viewType: ChnotViewType }) => {
  const { tags, setTagsInset } = useChnotHeadStore((store) => {
    return {
      tags: store.tags,
      setTagsInset: store.setTagsInset,
    };
  });

  const { currentKSpace, selectKSpace } = useKSpaceStore((store) => {
    return {
      currentKSpace: store.currentKSpace,
      selectKSpace: store.selectKSpace,
    };
  });

  const { changeSearchStr } = useChnotHeadStore((store) => {
    return {
      changeSearchStr: store.changeSearchStr,
    };
  });

  return (
    <>
      <div className="flex justify-between items-center">
        <div className="flex align-center space-x-1">
          <KSpaceSelect
            onSelect={(kspace: string): void => {
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
                setTagsInset(undefined);
              } else {
                setTagsInset([]);
              }
            }}
          >
            <Icon.Hash />
          </Toggle>
        </div>
        <div className="flex">
          <ChnotThreadSwitch viewType={viewType} />
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
                changeSearchStr(e.target.value);
              }}
            />
            <Search className="pointer-events-none absolute top-1/2 left-2 size-4 -translate-y-1/2 opacity-50 select-none" />
          </SidebarGroupContent>
        </SidebarGroup>
      </form>
    </>
  );
};

export default Header;
