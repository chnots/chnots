import IconButton from "@mui/joy/IconButton";
import Menu from "@mui/joy/Menu";
import MenuItem from "@mui/joy/MenuItem";
import ListItemDecorator from "@mui/joy/ListItemDecorator";
import MenuButton from "@mui/joy/MenuButton";
import Dropdown from "@mui/joy/Dropdown";
import Icon from "./Icon";
import { useDomainStore } from "@/store/v1/domain";

export const DomainIcon = ({
  name,
  className,
}: {
  name: string;
  className?: string;
}) => {
  if (name === "public") {
    return <Icon.Globe2 className={className} />;
  } else if (name === "work") {
    return <Icon.BriefcaseBusiness className={className} />;
  } else {
    return <Icon.Notebook className={className} />;
  }
};

export const DomainSelect = () => {
  const domainStore = useDomainStore();

  return (
    <Dropdown>
      <MenuButton
        slots={{ root: IconButton }}
        slotProps={{
          root: { variant: "plain", color: "neutral" },
        }}
      >
        <DomainIcon name={domainStore.current.name} />
      </MenuButton>
      <Menu placement="bottom-end">
        {domainStore.domains().map((item) => {
          return (
            <MenuItem onClick={() => domainStore.changeDomain(item.name)}>
              <ListItemDecorator>
                <DomainIcon name={item.name} />
              </ListItemDecorator>
              {item.name}
            </MenuItem>
          );
        })}
      </Menu>
    </Dropdown>
  );
};
