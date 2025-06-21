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
import React, { useEffect } from "react";

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

export const KSpaceSelect = ({
  onSelect,
  currentKSpace,
}: {
  onSelect: (kspace: string) => void;
  currentKSpace: string;
  showMKspaces?: boolean;
}) => {
  const { allKSpaces, toggleMKSpace, mkspaces } = useKSpaceStore();
  const [position, setPosition] = React.useState(currentKSpace);
  useEffect(() => {
    onSelect(position);
  }, [position]);

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
        <DropdownMenuRadioGroup value={position} onValueChange={setPosition}>
          {allKSpaces().map((e) => (
            <DropdownMenuRadioItem value={e.name} key={e.name}>
              <div className="flex items-center justify-between w-full px-0 py-0">
                <div
                  className="flex items-center flex-1 gap-2 cursor-pointer "
                  onClick={() => setPosition(e.name)}
                  tabIndex={0}
                  aria-label={`Select ${e.name}`}
                  onKeyDown={(event) =>
                    (event.key === "Enter" || event.key === " ") &&
                    setPosition(e.name)
                  }
                >
                  <KSpaceIcon name={e.name} />
                  <span className="truncate">{e.name}</span>
                </div>

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
              </div>
            </DropdownMenuRadioItem>
          ))}
        </DropdownMenuRadioGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
