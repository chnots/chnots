import Icon from "./icon";
import { useNamespaceStore } from "@/store/namespace";
import clsx from "clsx";
import { Menu, MenuButton, MenuItem } from "@szhsin/react-menu";

const NamespaceIcon = ({
  name,
  className,
}: {
  name?: string;
  className?: string;
}) => {
  if (name === "public") {
    return <Icon.Globe2 className={clsx(className, "h-auto shrink-0")} />;
  } else if (name === "work") {
    return (
      <Icon.BriefcaseBusiness
        className={clsx(className, "h-auto shrink-0")}
      />
    );
  } else if (name === "private") {
    return <Icon.Notebook className={clsx(className, "h-auto shrink-0")} />;
  } else {
    return <Icon.Dice1 className={clsx(className, "h-auto shrink-0")} />;
  }
};

export const NamespaceSelect = ({
  onSelect,
  currentNamespace,
  className
}: {
  onSelect: (namespace: string) => void;
  currentNamespace: string;
    className?: string;
}) => {
  const { namespaces } = useNamespaceStore();
  return (
    <Menu
      menuButton={
        <MenuButton>
          <NamespaceIcon name={currentNamespace} className={className}></NamespaceIcon>
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
