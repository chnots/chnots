import Icon from "./icon";
import { useNamespaceStore } from "@/store/namespace";
import { Menu, MenuButton, MenuItem } from "@szhsin/react-menu";

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
    <Menu
      menuButton={
        <MenuButton className={menuClassName}>
          <NamespaceIcon
            name={currentNamespace}
            className={className}
          ></NamespaceIcon>
        </MenuButton>
      }
      transition
    >
      {namespaces().map((e) => (
        <MenuItem
          key={e.name}
          onClick={() => {
            onSelect(e.name);
          }}
        >
          {e.name}
        </MenuItem>
      ))}
    </Menu>
  );
};
