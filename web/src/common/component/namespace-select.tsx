import Icon from "./icon";
import { useNamespaceStore } from "@/store/namespace";
import * as RadixDropmenu from "@radix-ui/react-dropdown-menu";
import KButton from "./kbutton";

const NamespaceIcon = ({
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

export const NamespaceSelect = ({
  onSelect,
  currentNamespace,
  className,
  menuClassName,
}: {
  onSelect: (namespace: string) => void;
  currentNamespace: string;
  className?: string;
  menuClassName?: string;
}) => {
  const { namespaces } = useNamespaceStore();
  return (
    <RadixDropmenu.Root>
      <RadixDropmenu.Trigger asChild>
        <KButton className={menuClassName}>
          <NamespaceIcon
            name={currentNamespace}
            className={className}
          ></NamespaceIcon>
        </KButton>
      </RadixDropmenu.Trigger>

      <RadixDropmenu.Portal>
        <RadixDropmenu.Content
          className="RadixDropmenuContent z-20"
          sideOffset={5}
        >
          {namespaces().map((e) => (
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
