import React, { useEffect } from "react";
import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/common/component/ui/dropdown-menu";
import { useKSpaceStore } from "@/krate/kspace/store";
import type { KSpace } from "../po";

export const KSpaceIcon = ({
  name,
  className,
}: {
  name?: string;
  className?: string;
}) => {
  if (name === "public") {
    return <Icon.BookKey className={className} />;
  } else if (name === "work") {
    return <Icon.BriefcaseBusiness className={className} />;
  } else if (name === "private") {
    return <Icon.BookLock className={className} />;
  } else {
    return <Icon.Dice1 className={className} />;
  }
};

export const KSpaceSelectDropDownGroup = ({
  kspace,
  onSelect,
  extra,
}: {
  kspace: string;
  onSelect: (kspace: string) => void;
  extra?: (kspace: KSpace) => React.ReactNode;
}) => {
  const { kspaceMapByName, refreshKSpaces } = useKSpaceStore((store) => {
    return {
      kspaceMapByName: store.kspaceMapByName,
      refreshKSpaces: store.refreshKSpaces,
    };
  });
  const [position, setPosition] = React.useState(kspace);

  useEffect(() => {
    refreshKSpaces();
  }, [refreshKSpaces]);

  return (
    <DropdownMenuRadioGroup value={position} onValueChange={setPosition}>
      {[...kspaceMapByName.values()].map((e) => (
        <DropdownMenuRadioItem value={e.name} key={e.name}>
          <div className="flex items-center justify-between w-full px-0 py-0">
            <div
              role="none"
              className="flex items-center flex-1 gap-2 cursor-pointer "
              onClick={() => onSelect(e.name)}
              onKeyDown={(event) =>
                (event.key === "Enter" || event.key === " ") && onSelect(e.name)
              }
            >
              <KSpaceIcon name={e.name} />
              <span className="truncate">{e.name}</span>
            </div>
          </div>
          {extra?.(e)}
        </DropdownMenuRadioItem>
      ))}
    </DropdownMenuRadioGroup>
  );
};

export const KSpaceSelect = ({
  onSelect,
  currentKSpace,
}: {
  onSelect: (kspace: string) => void;
  currentKSpace: string;
  showMKspaces?: boolean;
}) => {
  const { toggleMKSpace, mkspaces } = useKSpaceStore((store) => {
    return {
      toggleMKSpace: store.toggleMKSpace,
      mkspaces: store.mkspaces,
    };
  });

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="outline"
          className=" bg-transparent"
          size="sm"
          aria-label="Select knowledge space"
        >
          <KSpaceIcon name={currentKSpace} className="w-4 h-4" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56">
        <DropdownMenuLabel>KSpace</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <KSpaceSelectDropDownGroup
          kspace={currentKSpace}
          onSelect={onSelect}
          extra={(e) => (
            <Button
              variant="ghost"
              className="h-5"
              onClick={(event) => {
                event.stopPropagation();
                toggleMKSpace(e.name);
              }}
              aria-label={`Add ${e.name} to mkspace`}
            >
              {currentKSpace === e.name || mkspaces.includes(e.name) ? (
                <Icon.CircleCheckBig />
              ) : (
                <Icon.PlusCircle />
              )}
            </Button>
          )}
        />
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
