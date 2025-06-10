import Icon from "@/common/component/icon";
import { Button, Button as KButton } from "@/common/component/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/common/component/ui/dropdown-menu";
import { useKSpaceStore } from "@/krate/kspace/store/store";
import * as RadixDropmenu from "@radix-ui/react-dropdown-menu";
import { ChevronDown } from "lucide-react";

const KSpaceIcon = ({
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
  onlyIcon,
}: {
  onSelect: (kspace: string) => void;
  currentKSpace: string;
  onlyIcon?: boolean;
}) => {
  const { kspaces } = useKSpaceStore();
  return (
    <DropdownMenu>
      <DropdownMenuTrigger className="flex  justify-center items-center text-sm">
        {onlyIcon ? (
          <Button variant={"ghost"}>
            <KSpaceIcon name={currentKSpace} className="w-4 h-4" />
          </Button>
        ) : (
          <>
            <div className="space-x-4 flex items-center">
              <KSpaceIcon name={currentKSpace} className="w-4 h-4" />
              <span>{currentKSpace}</span>
            </div>
            <ChevronDown className="ml-auto w-4 h-4" />
          </>
        )}
      </DropdownMenuTrigger>

      <DropdownMenuContent className="RadixDropmenuContent z-20" sideOffset={5}>
        {kspaces().map((e) => (
          <DropdownMenuItem
            className="p-2"
            key={e.name}
            onClick={() => {
              onSelect(e.name);
            }}
          >
            {e.name}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
