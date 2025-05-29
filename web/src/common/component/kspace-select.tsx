import Icon from "./icon";
import { useKSpaceStore } from "@/krate/kspace/store/store";
import * as RadixDropmenu from "@radix-ui/react-dropdown-menu";
import KButton from "./kbutton";

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
  className,
  menuClassName,
}: {
  onSelect: (kspace: string) => void;
  currentKSpace: string;
  className?: string;
  menuClassName?: string;
}) => {
  const { kspaces } = useKSpaceStore();
  return (
    <RadixDropmenu.Root>
      <RadixDropmenu.Trigger asChild>
        <KButton className={menuClassName}>
          <KSpaceIcon name={currentKSpace} className={className}></KSpaceIcon>
        </KButton>
      </RadixDropmenu.Trigger>

      <RadixDropmenu.Portal>
        <RadixDropmenu.Content
          className="RadixDropmenuContent z-20"
          sideOffset={5}
        >
          {kspaces().map((e) => (
            <RadixDropmenu.Item
              className="p-2"
              key={e.name}
              onClick={() => {
                onSelect(e.name);
              }}
            >
              {e.name}
            </RadixDropmenu.Item>
          ))}
        </RadixDropmenu.Content>
      </RadixDropmenu.Portal>
    </RadixDropmenu.Root>
  );
};
