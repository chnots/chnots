import { Label } from "@radix-ui/react-dropdown-menu";
import { Hash, Search, Settings } from "lucide-react";
import { NavLink } from "react-router-dom";
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
import { useChnotStore } from "../../store";
import { ChnotKindSelect } from "./chnot-kind-select";

const TagsView = () => {
  const { setTagsInset, tags } = useChnotStore((store) => {
    return {
      setTagsInset: store.setTagsInset,
      tags: store.tags,
    };
  });

  return (
    <div className="w-full flex-row space-x-1 items-center inline">
      {tags?.id === "Inset" ? (
        tags.data.map((tag) => (
          <Button
            key={tag}
            onClick={() => {
              setTagsInset([...new Set(tags.data.filter((e) => e !== tag))]);
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

const Header = () => {
  const { tags, setTagsInset } = useChnotStore((store) => {
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

  const { changeSearchStr } = useChnotStore((store) => {
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
            <Hash />
          </Toggle>
        </div>
        <NavLink to={RoutePaths.Settings} id={"settings"}>
          <Settings className="w-4 h-4" />
        </NavLink>
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
