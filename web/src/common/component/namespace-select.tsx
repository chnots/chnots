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
    return <Icon.Globe2 className={clsx(className, "w-6 h-auto shrink-0")} />;
  } else if (name === "work") {
    return (
      <Icon.BriefcaseBusiness
        className={clsx(className, "w-6 h-auto shrink-0")}
      />
    );
  } else if (name === "private") {
    return <Icon.Notebook className={clsx(className, "w-6 h-auto shrink-0")} />;
  } else {
    return <Icon.Dice1 className={clsx(className, "w-6 h-auto shrink-0")} />;
  }
};

export const NamespaceSelect = ({
  onSelect,
  currentNamespace,
}: {
  onSelect: (namespace: string) => void;
  currentNamespace: string;
}) => {
  const { namespaces } = useNamespaceStore();
  return (
    <Menu
      menuButton={
        <MenuButton>
          <NamespaceIcon name={currentNamespace}></NamespaceIcon>
        </MenuButton>
      }
      transition
      className={"p-2"}
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
